//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use crate::{entrypoint, submit_tx::{TxParams, maybe_submit, tx_params_wrapper}};
use abscissa_core::{Command, Options, Runnable};
use anyhow::Error;
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::transaction::{SignedTransaction, authenticator::AuthenticationKey};
use ol_types::config::TxType;
use std::{path::PathBuf, process::exit};
/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct CreateAccountCmd {
    #[options(short = "a", help = "the new user's long address (authentication key)")]
    authkey: String,
    #[options(short = "c", help = "the amount of coins to send to new user")]
    coins: u64,
}

impl Runnable for CreateAccountCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        let authkey = match self.authkey.parse::<AuthenticationKey>(){
            Ok(a) => a,
            Err(e) => {
              println!("ERROR: could not parse this account address: {}, message: {}", self.authkey, &e.to_string());
              exit(1);
            },
        };
        let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();


        match create_from_auth_and_coin(authkey, self.coins, tx_params, entry_args.no_send, entry_args.save_path) {
            Ok(_) => println!("Success: Account created for authkey: {}", authkey),
            Err(e) => {
              println!("ERROR: could not create account, message: {}", &e.to_string());
              exit(1);
            },
        }
    }
}

/// create an account by sending coin to it
pub fn create_from_auth_and_coin(authkey: AuthenticationKey, coins: u64, tx_params: TxParams, no_send: bool, save_path: Option<PathBuf>) -> Result<SignedTransaction, Error>{

  let account = authkey.derived_address();
  let prefix = authkey.prefix();
  // NOTE: coins here do not have the scaling factor. Rescaling is the responsibility of the Move script. See the script in ol_accounts.move for detail.
  let script = transaction_builder::encode_create_user_by_coin_tx_script_function(
      account,
      prefix.to_vec(),
      coins,
  );

  maybe_submit(script, &tx_params, no_send, save_path)
}

