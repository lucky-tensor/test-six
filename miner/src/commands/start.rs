//! `start` subcommand - example of how to write a subcommand

use crate::{backlog, block::*, submit_tx::get_params};
use crate::config::MinerConfig;
use crate::prelude::*;
use anyhow::Error;
use libra_types::waypoint::Waypoint;
use std::path::PathBuf;

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
pub struct StartCmd {
    // Option for --waypoint, to set a specific waypoint besides genesis_waypoint which is found in key_store.json
    #[options(help = "Provide a waypoint for tx submission. Will otherwise use what is in key_store.json")]
    waypoint: String,
    // Option for --backlog, only sends backlogged transactions.
    #[options(help = "Start but don't mine, and only resubmit backlog of proofs")]
    backlog: bool,
    // Option for setting path for the blocks/proofs that are mined.
    #[options(help = "The home directory where the blocks will be stored")]
    home: PathBuf, 
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let miner_configs = app_config();
        let waypoint: Waypoint;
        let parsed_waypoint: Result<Waypoint, Error> = self.waypoint.parse();
        match parsed_waypoint {
            Ok(v) => {
                println!("Using Waypoint from CLI args:\n{}", v);
                waypoint = parsed_waypoint.unwrap();
            }
            Err(_e) => {
                waypoint = miner_configs.get_waypoint().parse().unwrap();

                status_info!("Waypoint:",format!("No waypoint parsed from command line args. Using waypoint in key_store.json {:?}", waypoint));
            }
        }

        println!("Enter your 0L mnemonic:");
        let mnemonic_string = rpassword::read_password_from_tty(Some("\u{1F511} ")).unwrap();

        let tx_params = get_params(&mnemonic_string, waypoint, &miner_configs);
        
        // Check for, and submit backlog proofs.
        backlog::process_backlog(&miner_configs, &tx_params);

        if !self.backlog {
            // Steady state.
            let result = build_block::mine_and_submit(&miner_configs, tx_params);
            match result {
                Ok(_val) => {}
                Err(err) => {
                    println!("Failed to mine_and_submit: {}", err);
                }
            }
        } else {
            // Chain needs to catch up to backlog of proofs.
            backlog::process_backlog(&miner_configs, &tx_params);
        }
    }
}

impl config::Override<MinerConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: MinerConfig) -> Result<MinerConfig, FrameworkError> {
        Ok(config)
    }
}
