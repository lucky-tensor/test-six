// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    smoke_test_environment::SmokeTestEnvironment, test_utils::load_libra_root_storage,
    workspace_builder, workspace_builder::workspace_root,
};
use anyhow::anyhow;
use libra_config::config::{NodeConfig, SecureBackend, WaypointConfig};
use libra_secure_storage::{KVStorage, Storage};
use libra_temppath::TempPath;
use libra_types::{
    account_config::{libra_root_address, treasury_compliance_account_address},
    transaction::{Transaction, WriteSetPayload},
    waypoint::Waypoint,
};
use regex::Regex;
use std::{
    fs, fs::File, io::Write, path::PathBuf, process::Command, str::FromStr, thread::sleep,
    time::Duration,
};
use transaction_builder::encode_remove_validator_and_reconfigure_script;

#[test]
/// This test verifies the flow of a genesis transaction after the chain starts.
/// 1. test the consensus sync_only mode, every node should stop at the same version.
/// 2. test the db-bootstrapper apply a manual genesis transaction (remove validator 0) on libradb directly
/// 3. test the nodes and clients resume working after updating waypoint
/// 4. test a node lag behind can sync to the waypoint
fn test_genesis_transaction_flow() {
    let db_bootstrapper = workspace_builder::get_bin("db-bootstrapper");
    let mut env = SmokeTestEnvironment::new(4);
    println!("1. set sync_only = true for the first node and check it can sync to others");
    let config_path = env.validator_swarm.config.config_files.first().unwrap();
    let mut node_config = NodeConfig::load(config_path).unwrap();
    node_config.consensus.sync_only = true;
    node_config.save(config_path).unwrap();
    env.validator_swarm.launch();
    let mut client_proxy_0 = env.get_validator_client(0, None);
    client_proxy_0.create_next_account(false).unwrap();
    client_proxy_0
        .mint_coins(&["mintb", "0", "10", "Coin1"], true)
        .unwrap();
    println!("2. set sync_only = true for all nodes and restart");
    for (i, config_path) in env
        .validator_swarm
        .config
        .config_files
        .clone()
        .iter()
        .enumerate()
    {
        let mut node_config = NodeConfig::load(config_path).unwrap();
        node_config.consensus.sync_only = true;
        node_config.save(config_path).unwrap();
        env.validator_swarm.kill_node(i);
        env.validator_swarm.add_node(i).unwrap();
    }
    println!("3. delete one node's db and test they can still sync when sync_only is true for every nodes");
    env.validator_swarm.kill_node(0);
    fs::remove_dir_all(node_config.storage.dir()).unwrap();
    env.validator_swarm.add_node(0).unwrap();
    println!("4. verify all nodes are at the same round and no progress being made in 5 sec");
    env.validator_swarm.wait_for_all_nodes_to_catchup();
    let mut known_round = None;
    for i in 0..5 {
        let last_committed_round_str = "libra_consensus_last_committed_round{}";
        for (index, node) in &mut env.validator_swarm.nodes {
            if let Some(round) = node.get_metric(last_committed_round_str) {
                match known_round {
                    Some(r) if r != round => panic!(
                        "round not equal, last known: {}, node {} is {}",
                        r, index, round
                    ),
                    None => known_round = Some(round),
                    _ => continue,
                }
            } else {
                panic!("unable to get round from node {}", index);
            }
        }
        println!(
            "The last know round after {} sec is {}",
            i,
            known_round.unwrap()
        );
        sleep(Duration::from_secs(1));
    }
    println!("5. kill all nodes and prepare a genesis txn to remove validator 0");
    let validator_address = node_config.validator_network.as_ref().unwrap().peer_id();
    let op_tool = env.get_op_tool(0);
    let libra_root = load_libra_root_storage(&node_config);
    let config = op_tool
        .validator_config(validator_address, &libra_root)
        .unwrap();
    let name = config.name.as_bytes().to_vec();

    for index in 0..env.validator_swarm.nodes.len() {
        env.validator_swarm.kill_node(index);
    }
    let genesis_transaction = Transaction::GenesisTransaction(WriteSetPayload::Script {
        execute_as: libra_root_address(),
        script: encode_remove_validator_and_reconfigure_script(0, name, validator_address),
    });
    let genesis_path = TempPath::new();
    genesis_path.create_as_file().unwrap();
    let mut file = File::create(genesis_path.path()).unwrap();
    file.write_all(&lcs::to_bytes(&genesis_transaction).unwrap())
        .unwrap();
    println!("6. prepare the waypoint with the transaction");
    let waypoint_command = Command::new(db_bootstrapper.as_path())
        .current_dir(workspace_root())
        .args(&vec![
            node_config.storage.dir().to_str().unwrap(),
            "--genesis-txn-file",
            genesis_path.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let output = std::str::from_utf8(&waypoint_command.stdout).unwrap();
    let waypoint = parse_waypoint(output);
    let set_waypoint = |node_config: &NodeConfig| {
        let f = |backend: &SecureBackend| {
            let mut storage: Storage = backend.into();
            storage
                .set(libra_global_constants::WAYPOINT, waypoint)
                .expect("Unable to write waypoint");
        };
        let backend = &node_config.consensus.safety_rules.backend;
        f(backend);
        match &node_config.base.waypoint {
            WaypointConfig::FromStorage(backend) => {
                f(backend);
            }
            _ => panic!("unexpected waypoint from node config"),
        }
    };
    println!("7. apply genesis transaction for nodes 1, 2, 3");
    for config_path in env
        .validator_swarm
        .config
        .config_files
        .clone()
        .iter()
        .skip(1)
    {
        let mut node_config = NodeConfig::load(config_path).unwrap();
        set_waypoint(&node_config);
        node_config.execution.genesis = Some(genesis_transaction.clone());
        // reset the sync_only flag to false
        node_config.consensus.sync_only = false;
        node_config.save(config_path).unwrap();
    }
    for i in 1..4 {
        env.validator_swarm.add_node(i).unwrap();
    }
    println!("8. verify it's able to mint after the waypoint");
    let mut client_proxy_1 = env.get_validator_client(1, Some(waypoint));
    client_proxy_1.set_accounts(client_proxy_0.copy_all_accounts());
    client_proxy_1.create_next_account(false).unwrap();
    client_proxy_1
        .mint_coins(&["mintb", "1", "10", "Coin1"], true)
        .unwrap();
    client_proxy_1
        .wait_for_transaction(treasury_compliance_account_address(), 1)
        .unwrap();
    println!("9. add node 0 back and test if it can sync to the waypoint via state synchronizer");
    let op_tool = env.get_op_tool(1);
    let context = op_tool
        .add_validator(validator_address, &load_libra_root_storage(&node_config))
        .unwrap();
    client_proxy_1
        .wait_for_transaction(context.address, context.sequence_number + 1)
        .unwrap();
    // setup the waypoint for node 0
    node_config.execution.genesis = None;
    node_config.execution.genesis_file_location = PathBuf::from("");
    set_waypoint(&node_config);
    node_config
        .save(&env.validator_swarm.config.config_files[0])
        .unwrap();
    env.validator_swarm.add_node(0).unwrap();
    let mut client_proxy_0 = env.get_validator_client(0, Some(waypoint));
    client_proxy_0.set_accounts(client_proxy_1.copy_all_accounts());
    client_proxy_0.create_next_account(false).unwrap();
    client_proxy_1
        .mint_coins(&["mintb", "1", "10", "Coin1"], true)
        .unwrap();
}

fn parse_waypoint(db_bootstrapper_output: &str) -> Waypoint {
    let waypoint = Regex::new(r"Got waypoint: (\d+:\w+)")
        .unwrap()
        .captures(db_bootstrapper_output)
        .ok_or_else(|| anyhow!("Failed to parse db-bootstrapper output."));
    Waypoint::from_str(waypoint.unwrap()[1].into()).unwrap()
}
