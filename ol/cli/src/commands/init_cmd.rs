//! `init` subcommand

#![allow(clippy::never_loop)]

use crate::{application::app_config, config::AppCfg, entrypoint, migrate};
use abscissa_core::{Command, FrameworkError, Options, Runnable, config};
use anyhow::Error;
use libra_genesis_tool::{init, key};
use libra_types::waypoint::Waypoint;
use ol_keys::{scheme::KeyScheme, wallet};
use libra_json_rpc_client::AccountAddress;
use libra_types::transaction::authenticator::AuthenticationKey;
use std::{fs, path::PathBuf};
use libra_wallet::WalletLibrary;
use url::Url;
use fs_extra::file::{copy, CopyOptions};
use fs_extra::dir::{create};

/// `init` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct InitCmd {
    #[options(help = "home path for miner app")]
    path: Option<PathBuf>,
    #[options(help = "An upstream peer to use in 0L.toml")]
    upstream_peer: Option<Url>,
    #[options(help = "Skip miner app configs")]
    skip_miner: bool,
    #[options(help = "Skip validator init")]
    skip_val: bool,
    #[options(help = "Fix config file, and migrate any missing fields")]
    fix: bool,
    #[options(help = "Set a waypoint in config files")]
    waypoint: Option<Waypoint>,
}


impl Runnable for InitCmd {
    /// Print version message
    fn run(&self) {
        if *&self.fix {
          // fix 0L.toml file
          migrate::migrate(self.path.to_owned());

          // fix account.json
          
          // TODO: fix key_store.json
          return
        };

        let entry_args = entrypoint::get_args();
        if let Some(path) = entry_args.swarm_path {
          let swarm_node_home = entrypoint::get_node_home();
          let absolute = fs::canonicalize(path).unwrap();
          initialize_host_swarm(absolute, swarm_node_home, entry_args.swarm_persona);
          return
        }
        
        let (authkey, account, wallet) = wallet::get_account_from_prompt();
        // start with a default value, or read from file if already initialized
        let mut miner_config = app_config().to_owned();
        if !self.skip_miner { 
          miner_config =  initialize_host(
            authkey,
            account, 
            &self.upstream_peer,
            &self.path,
            None, // TODO: probably need an epoch option here.
            self.waypoint,
          ).unwrap()
        };

        if !self.skip_val {
          initialize_validator(&wallet, &miner_config, self.waypoint).unwrap() 
        };
    }
}

/// Initializes the necessary 0L config files: 0L.toml
pub fn initialize_host(authkey: AuthenticationKey, account: AccountAddress, upstream_peer: &Option<Url>, path: &Option<PathBuf>, epoch_opt: Option<u64>, wp_opt: Option<Waypoint>) -> Result <AppCfg, Error>{
    let cfg = AppCfg::init_app_configs(authkey, account, upstream_peer, path, epoch_opt, wp_opt);
    Ok(cfg)
}

/// Initializes the necessary 0L config files: 0L.toml and populate blocks directory
/// assumes the libra source is checked out at $HOME/libra
pub fn initialize_host_swarm(swarm_path: PathBuf, node_home: PathBuf, persona: Option<String>) {
    let cfg = AppCfg::init_app_configs_swarm(swarm_path, node_home);
    if persona.is_some() {
      let source = PathBuf::new().join(&cfg.workspace.source_path.unwrap()).join("ol/fixtures/blocks/test").join(persona.unwrap()).join("block_0.json");
      let bocks_dir = PathBuf::new().join(&cfg.workspace.node_home).join(&cfg.workspace.block_dir);
      let target_file = PathBuf::new().join(&cfg.workspace.node_home).join(&cfg.workspace.block_dir).join("block_0.json");
      println!("copy first block from {:?} to {:?}", source, target_file);
      match create(bocks_dir, false) {
        Err(why) => println!("create block dir failed: {:?}", why),
        _ => match copy(source, target_file, &CopyOptions::new()) {
          Err(why) => println!("copy block failed: {:?}", why),
          _ => (),
        }
      }
    }
}

/// Initializes the necessary validator config files: genesis.blob, key_store.json
pub fn initialize_validator(wallet: &WalletLibrary, miner_config: &AppCfg, way_opt: Option<Waypoint>) -> Result <(), Error>{
    let home_dir = &miner_config.workspace.node_home;
    let keys = KeyScheme::new(wallet);
    let namespace = miner_config.profile.auth_key.to_owned();
    init::key_store_init(home_dir, &namespace, keys, false);
    key::set_operator_key(home_dir, &namespace);
    key::set_owner_key(home_dir, &namespace);
    if let Some(way) = way_opt {
      key::set_genesis_waypoint(home_dir, &namespace, way);
      key::set_waypoint(home_dir, &namespace, way);
    }
    Ok(())
}

impl config::Override<AppCfg> for InitCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: AppCfg) -> Result<AppCfg, FrameworkError> {
        Ok(config)
    }
}