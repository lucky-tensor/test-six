//! `start` subcommand - example of how to write a subcommand

use crate::{block::ValConfigs, submit_tx::get_params};
use crate::config::MinerConfig;
use crate::prelude::*;
use anyhow::Error;
use libra_types::waypoint::Waypoint;
use crate::submit_tx::submit_onboard_tx;
use std::path::PathBuf;

// use rustyline::error::ReadlineError;
// use rustyline::Editor;

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

/// `start` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct OnboardCmd {
    // Option for --waypoint, to set a specific waypoint besides genesis_waypoint which is found in key_store.json
    #[options(help = "Provide a waypoint for tx submission. Will otherwise use what is in key_store.json")]
    waypoint: String,
    // Path of the block_0.json to submit.
    #[options(help = "Path of the block_0.json to submit.")]
    file: PathBuf, 
}

impl Runnable for OnboardCmd {
    /// Start the application.
    fn run(&self) {
        let miner_configs = app_config();

        println!("Enter your 0L mnemonic:");
        let mnemonic_string = rpassword::read_password_from_tty(Some("\u{1F511} ")).unwrap();

        let waypoint: Waypoint;
        let parsed_waypoint: Result<Waypoint, Error> = self.waypoint.parse();
        match parsed_waypoint {
            Ok(v) => {
                println!("Using Waypoint from CLI args:\n{}", v);
                waypoint = parsed_waypoint.unwrap();
            }
            Err(_e) => {
                println!("Info: No waypoint parsed from command line args. Received: {:?}\n\
                Using waypoint in miner.toml\n {:?}",
                self.waypoint,
                miner_configs.chain_info.base_waypoint);
                waypoint = miner_configs.get_waypoint().parse().unwrap();

            }
        }

        let tx_params = get_params(&mnemonic_string, waypoint, &miner_configs);
        let init_data = ValConfigs::get_init_data(&self.file).unwrap();
        
        match submit_onboard_tx(
            &tx_params,
            init_data.block_zero.preimage.to_owned(),
            init_data.block_zero.proof.to_owned(),
            init_data.consensus_pubkey,
            init_data.validator_network_identity_pubkey,
            init_data.validator_network_address,
            init_data.full_node_network_identity_pubkey,
            init_data.full_node_network_address,
        ) {
            Ok(_res) => {
                status_ok!("Success", "Validator initialization committed, exiting.");
            }
            Err(e) => {
                status_warn!(format!("Validator initialization error: {:?}", e));

            }
        }
    }
}

impl config::Override<MinerConfig> for OnboardCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: MinerConfig) -> Result<MinerConfig, FrameworkError> {
        Ok(config)
    }
}
