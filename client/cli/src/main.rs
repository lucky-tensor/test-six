// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use chrono::{
    prelude::{SecondsFormat, Utc},
    DateTime,
};
use cli::{
    client_proxy::ClientProxy,
    commands::{get_commands, parse_cmd, report_error, Command},
};
use libra_types::waypoint::Waypoint;
use rustyline::{config::CompletionType, error::ReadlineError, Config, Editor};
use std::{
    num::NonZeroU16,
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Libra Client",
    author = "The Libra Association",
    about = "Libra client to connect to a specific validator"
)]
struct Args {
    /// JSON RPC port to connect to.
    #[structopt(short = "p", long, default_value = "5000")]
    pub port: NonZeroU16,
    /// Host address/name to connect to.
    #[structopt(short = "a", long)]
    pub host: String,
    /// Path to the generated keypair for the faucet account. The faucet account can be used to
    /// mint coins. If not passed, a new keypair will be generated for
    /// you and placed in a temporary directory.
    /// To manually generate a keypair, use generate-keypair:
    /// `cargo run -p generate-keypair -- -o <output_file_path>`
    #[structopt(short = "m", long = "faucet-key-file-path")]
    pub faucet_account_file: Option<String>,
    /// Host that operates a faucet service
    /// If not passed, will be derived from host parameter
    #[structopt(short = "f", long)]
    pub faucet_server: Option<String>,
    /// File location from which to load mnemonic word for user account address/key generation.
    /// If not passed, a new mnemonic file will be generated by libra-wallet in the current
    /// directory.
    #[structopt(short = "n", long)]
    pub mnemonic_file: Option<String>,
    /// If set, client will sync with validator during wallet recovery.
    #[structopt(short = "r", long = "sync")]
    pub sync: bool,
    /// If set, a client uses the waypoint parameter for its initial LedgerInfo verification.
    #[structopt(
        name = "waypoint",
        long,
        help = "Explicitly specify the waypoint to use"
    )]
    pub waypoint: Option<Waypoint>,
    #[structopt(
        name = "waypoint_url",
        long,
        help = "URL for a file with the waypoint to use"
    )]
    pub waypoint_url: Option<String>,
    /// Verbose output.
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

fn main() {
    ::libra_logger::Logger::new().init();
    crash_handler::setup_panic_handler();
    let args = Args::from_args();

    let (commands, alias_to_cmd) = get_commands(args.faucet_account_file.is_some());

    let faucet_account_file = args
        .faucet_account_file
        .clone()
        .unwrap_or_else(|| "".to_string());
    let mnemonic_file = args.mnemonic_file.clone();

    // If waypoint is given explicitly, use its value,
    // otherwise if waypoint_url is given, try to retrieve the waypoint from the URL,
    // otherwise waypoint is None.
    let waypoint = args.waypoint.or_else(|| {
        args.waypoint_url.as_ref().map(|url_str| {
            retrieve_waypoint(url_str.as_str()).unwrap_or_else(|e| {
                panic!("Failure to retrieve a waypoint from {}: {}", url_str, e)
            })
        })
    });
    let mut client_proxy = ClientProxy::new(
        &args.host,
        args.port.get(),
        &faucet_account_file,
        args.sync,
        args.faucet_server.clone(),
        mnemonic_file,
        waypoint,
    )
    .expect("Failed to construct client.");

    // Test connection to validator
    let latest_li = client_proxy
        .test_validator_connection()
        .unwrap_or_else(|e| {
            panic!(
                "Not able to connect to validator at {}:{}. Error: {}",
                args.host, args.port, e,
            )
        });
    let ledger_info_str = format!(
        "latest version = {}, timestamp = {}",
        latest_li.ledger_info().version(),
        DateTime::<Utc>::from(
            UNIX_EPOCH + Duration::from_micros(latest_li.ledger_info().timestamp_usecs())
        )
    );
    let cli_info = format!(
        "Connected to validator at: {}:{}, {}",
        args.host, args.port, ledger_info_str
    );
    if args.mnemonic_file.is_some() {
        match client_proxy.recover_accounts_in_wallet() {
            Ok(account_data) => {
                println!(
                    "Wallet recovered and the first {} child accounts were derived",
                    account_data.len()
                );
                for data in account_data {
                    println!("#{} address {}", data.index, hex::encode(data.address));
                }
            }
            Err(e) => report_error("Error recovering Libra wallet", e),
        }
    }
    print_help(&cli_info, &commands);
    println!("Please, input commands: \n");

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .auto_add_history(true)
        .build();
    let mut rl = Editor::<()>::with_config(config);
    loop {
        let readline = rl.readline("libra% ");
        match readline {
            Ok(line) => {
                let params = parse_cmd(&line);
                if params.is_empty() {
                    continue;
                }
                match alias_to_cmd.get(&params[0]) {
                    Some(cmd) => {
                        if args.verbose {
                            println!("{}", Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true));
                        }
                        cmd.execute(&mut client_proxy, &params);
                    }
                    None => match params[0] {
                        "quit" | "q!" => break,
                        "help" | "h" => print_help(&cli_info, &commands),
                        "" => continue,
                        x => println!("Unknown command: {:?}", x),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

/// Print the help message for the client and underlying command.
fn print_help(client_info: &str, commands: &[std::sync::Arc<dyn Command>]) {
    println!("{}", client_info);
    println!("usage: <command> <args>\n\nUse the following commands:\n");
    for cmd in commands {
        println!(
            "{} {}\n\t{}",
            cmd.get_aliases().join(" | "),
            cmd.get_params_help(),
            cmd.get_description()
        );
    }

    println!("help | h \n\tPrints this help");
    println!("quit | q! \n\tExit this client");
    println!("\n");
}

/// Retrieve a waypoint given the URL.
fn retrieve_waypoint(url_str: &str) -> anyhow::Result<Waypoint> {
    let response = ureq::get(url_str).timeout_connect(10_000).call();
    match response.status() {
        200 => response
            .into_string()
            .map_err(|_| anyhow::format_err!("Failed to parse waypoint from URL {}", url_str))
            .and_then(|r| Waypoint::from_str(r.trim())),
        _ => Err(anyhow::format_err!(
            "URL {} returned {}",
            url_str,
            response.status_line()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_port() {
        let args = Args::from_iter(&["test", "--host=h"]);
        assert_eq!(args.port.get(), 8000);
        assert_eq!(format!("{}:{}", args.host, args.port.get()), "h:8000");
        let args = Args::from_iter(&["test", "--port=65535", "--host=h"]);
        assert_eq!(args.port.get(), 65535);
    }

    #[test]
    fn test_args_port_too_large() {
        let result = Args::from_iter_safe(&["test", "--port=65536", "--host=h"]);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_args_port_invalid() {
        let result = Args::from_iter_safe(&["test", "--port=abc", "--host=h"]);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_args_port_zero() {
        let result = Args::from_iter_safe(&["test", "--port=0", "--host=h"]);
        assert_eq!(result.is_ok(), false);
    }
}
