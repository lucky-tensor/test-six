//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use crate::{
    entrypoint,
    prelude::app_config,
    relay,
    submit_tx::{tx_params_wrapper, maybe_submit},
};
use abscissa_core::{Command, Options, Runnable};
use dialoguer::Confirm;
use libra_types::transaction::Script;
use ol::node::node::Node;
use ol_types::{account::ValConfigs, config::TxType};
use reqwest::Url;
use std::{fs::{self, File}, io::Write, path::PathBuf, process::exit};
/// `CreateAccount` subcommand
#[derive(Command, Debug, Options)]
pub struct CreateValidatorCmd {
    #[options(short = "f", help = "path of account.json")]
    account_file: Option<PathBuf>,
    #[options(short = "u", help = "onboard from URL")]
    url: Option<Url>,
}

/// create validator account by submitting transaction on chain
pub fn create_validator_script(new_account: &ValConfigs) -> Script {
    let new_account = new_account.to_owned();
    new_account.check_autopay().unwrap();

    transaction_builder::encode_create_acc_val_script(
        new_account.block_zero.preimage,
        new_account.block_zero.proof,
        new_account.ow_human_name.as_bytes().to_vec(),
        new_account.op_address.parse().unwrap(),
        new_account.op_auth_key_prefix,
        new_account.op_consensus_pubkey,
        new_account.op_validator_network_addresses,
        new_account.op_fullnode_network_addresses,
        new_account.op_human_name.as_bytes().to_vec(),
        // my_trusted_accounts,
        // voter_trusted_accounts,
    )
}

pub fn account_from_url(url: &mut Url, path: &PathBuf) -> PathBuf {
    url.set_port(Some(3030)).unwrap();
    let url_string = url.join("account.json").unwrap();
    let res = reqwest::blocking::get(url_string);
    
    let host_string = url.host().unwrap().to_string();
    let file_path = path.join(format!("{}.account.json", host_string));
    let mut file = File::create(&file_path).expect("couldn't create file");
    let content = res.unwrap().bytes().unwrap().to_vec(); //.text().unwrap();
    file.write_all(content.as_slice()).unwrap();
    file_path
}

impl Runnable for CreateValidatorCmd {
    fn run(&self) {
        let cfg = app_config().clone();
        let entry_args = entrypoint::get_args();
        let tmp;
        if self.account_file.is_none() && self.url.is_none() {
            panic!("No account file nor URL passed in CLI")
        }
        let account_json_path: &PathBuf = if self.account_file.is_some() {
            self.account_file.as_ref().unwrap()
        } else {
            let mut url = self.url.clone().unwrap();
            tmp = account_from_url(&mut url, &cfg.workspace.node_home).clone();
            &tmp
        };

        let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();

        let file = fs::File::open(account_json_path).expect("file should open read only");
        let new_account: ValConfigs =
            serde_json::from_reader(file).expect("file should be proper JSON");
        
        let node = Node::default_from_cfg(cfg);
        let epoch_now = match node.vitals.chain_view {
            Some(c) => c.epoch,
            None => {
              println!("Could not connect to chain to fetch epoch. Exiting");
              exit(1);
            },
        };

        match new_account.check_autopay() {
            Ok(_) => {
                maybe_submit(
                    create_validator_script(&new_account),
                    &tx_params,
                    entry_args.no_send,
                    entry_args.save_path,
                )
                .unwrap();

                // submit autopay if there are any
                new_account.autopay_instructions.unwrap()
                .into_iter()
                .for_each(|i|{
                    println!("{}", i.text_instructions(&epoch_now));
                    match Confirm::new().with_prompt("").interact().unwrap() {
                      true => {},
                      _ =>  {
                        panic!("Autopay configuration aborted. Check batch configuration file or template");
                      }
                    } 
                });

                relay::relay_batch(&new_account.autopay_signed.unwrap(), &tx_params).unwrap();
            }
            Err(_) => {
                println!(
                    "cannot send atomic account creation transaction, error with: PayInstruction."
                );
            }
        }
    }
}
