//! Txs App submit_tx module
#![forbid(unsafe_code)]
use crate::{
    config::AppCfg,
    entrypoint::{self, EntryPointTxsCmd},
    prelude::app_config,
    save_tx::save_tx,
    sign_tx::sign_tx,
};
use anyhow::{Error, anyhow};
use cli::{diem_client::DiemClient, AccountData, AccountStatus};
use diem_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    test_utils::KeyPair,
};
use diem_global_constants::OPERATOR_KEY;
use diem_json_rpc_types::views::{TransactionView, VMStatusView};
use diem_secure_storage::{CryptoStorage, Namespaced, OnDiskStorage, Storage};
use diem_types::{account_address::AccountAddress, waypoint::Waypoint};
use diem_types::{
    chain_id::ChainId,
    transaction::{authenticator::AuthenticationKey, SignedTransaction, TransactionPayload},
};
use ol_keys::{scheme::KeyScheme, wallet};

use diem_wallet::WalletLibrary;
use ol_types::{
    self,
    config::{TxCost, TxType},
    fixtures,
};
use reqwest::Url;
use std::{
    io::{stdout, Write},
    path::PathBuf,
    thread, time,
};

/// All the parameters needed for a client transaction.
#[derive(Debug)]
pub struct TxParams {
    /// User's 0L authkey used in mining.
    pub auth_key: AuthenticationKey,
    /// Address of the signer of transaction, e.g. owner's operator
    pub signer_address: AccountAddress,
    /// Optional field for Miner, for operator to send owner
    // TODO: refactor so that this is not par of the TxParams type
    pub owner_address: AccountAddress,
    /// Url
    pub url: Url,
    /// waypoint
    pub waypoint: Waypoint,
    /// KeyPair
    pub keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey>,
    /// tx cost and timeout info
    pub tx_cost: TxCost,
    // /// User's Maximum gas_units willing to run. Different than coin.
    // pub max_gas_unit_for_tx: u64,
    // /// User's GAS Coin price to submit transaction.
    // pub coin_price_per_unit: u64,
    // /// User's transaction timeout.
    // pub user_tx_timeout: u64, // for compatibility with UTC's timestamp.
    /// Chain id
    pub chain_id: ChainId,
}

#[derive(Debug)]
/// a transaction error type specific to ol txs
pub struct TxError {
    /// the actual error type
    pub err: Option<Error>,
    /// transaction view if the transaction got that far
    pub tx_view: Option<TransactionView>,
    /// Move module or script where error occurred
    pub location: Option<String>,
    /// Move abort code used in error
    pub abort_code: Option<u64>,
}

impl From<Error> for TxError {
    fn from(e: Error) -> Self {
        TxError{ err: Some(e), tx_view: None, location: None, abort_code: None }
    }
}

/// wrapper for sending a transaction.
pub fn maybe_submit(
    script: TransactionPayload,
    tx_params: &TxParams,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let mut client =
        DiemClient::new(tx_params.url.clone(), tx_params.waypoint).map_err(|e| TxError {
            err: Some(e),
            tx_view: None,
            location: None,
            abort_code: None,
        })?;

    let (mut account_data, txn) = stage(script, tx_params, &mut client)?;
    if let Some(path) = save_path {
        // TODO: This will not work with batch operations like autopay_batch, last one will overwrite the file.
        save_tx(txn.clone(), path);
    }

    match submit_tx(client, txn.clone(), &mut account_data) {
        Ok(res) => eval_tx_status(res),
        Err(e) => Err(TxError {
            err: Some(e),
            tx_view: None,
            location: None,
            abort_code: None,
        }),
    }
}

/// wrapper for saving a transction without sending
pub fn save_dont_send_tx(
    script: TransactionPayload,
    tx_params: &TxParams,
    save_path: Option<PathBuf>,
) -> Result<SignedTransaction, TxError> {
    let mut client = DiemClient::new(tx_params.url.clone(), tx_params.waypoint)
    .map_err(|e| { TxError { 
      err: Some(e),
      tx_view:  None,
      location: None,
      abort_code: None,
      }
    })?;

    let (_account_data, txn) = stage(script, tx_params, &mut client)?;
    if let Some(path) = save_path {
        // TODO: This will not work with batch operations like autopay_batch, last one will overwrite the file.
        save_tx(txn.clone(), path);
    }
    Ok(txn)
}

/// convenience for wrapping multiple transactions
pub fn batch_wrapper(
    batch: Vec<TransactionPayload>,
    tx_params: &TxParams,
    no_send: bool,
    save_path: Option<PathBuf>,
) -> Result<(), Error> {
    batch.into_iter().enumerate().for_each(|(i, s)| {
        // TODO: format path for batch scripts

        let new_path = match &save_path {
            Some(p) => Some(p.join(i.to_string())),
            None => None,
        };

        // TODO: handle saving of batches to file.
        // The user may be expecting the batch transaction to be atomic.
        if no_send {
            save_dont_send_tx(s.clone(), tx_params, new_path).unwrap();
        } else {
            maybe_submit(s, tx_params, new_path).unwrap();
        }
    });
    Ok(())
}

fn stage(
    script: TransactionPayload,
    tx_params: &TxParams,
    client: &mut DiemClient,
) -> Result<(AccountData, SignedTransaction), TxError> {
    match client.get_metadata() {
        Ok(meta) => {
            if let Some(av) = client.get_account(&tx_params.signer_address)? {
                let sequence_number = av.sequence_number;
                // Sign the transaction script
                let txn = sign_tx(
                  script,
                  tx_params,
                  sequence_number,
                  ChainId::new(meta.chain_id)
                )?;

                // Get account_data struct
                let signer_account_data = AccountData {
                    address: tx_params.signer_address,
                    authentication_key: Some(tx_params.auth_key.to_vec()),
                    key_pair: Some(tx_params.keypair.clone()),
                    sequence_number,
                    status: AccountStatus::Persisted,
                };
                Ok((signer_account_data, txn))
            } else {
              Err(anyhow!("cannot get account_state from chain").into())
            }
        },
        _ => {
            let msg = format!("ERROR: could not get chain metadata, cannot send tx");
            println!("{}", &msg);
            Err(anyhow!(msg).into())
        },
    }
}

/// Submit a transaction to the network.
pub fn submit_tx(
    mut client: DiemClient,
    txn: SignedTransaction,
    mut _signer_account_data: &mut AccountData,
) -> Result<TransactionView, Error> {
    // Submit the transaction with diem_client
    match client.submit_transaction(&txn) {
        Ok(_) => match wait_for_tx(txn.sender(), txn.sequence_number(), &mut client) {
            Some(res) => Ok(res),
            None => Err(Error::msg("No Transaction View returned")),
        },
        Err(err) => Err(err),
    }
}

/// Main get tx params logic based on the design in this URL:
/// https://github.com/OLSF/libra/blob/tx-sender/txs/README.md#txs-logic--usage
pub fn tx_params_wrapper(tx_type: TxType) -> Result<TxParams, Error> {
    let EntryPointTxsCmd {
        url,
        waypoint,
        swarm_path,
        swarm_persona,
        is_operator,
        use_upstream_url,
        ..
    } = entrypoint::get_args();
    let app_config = app_config().clone();
    tx_params(
        app_config,
        url,
        waypoint,
        swarm_path,
        swarm_persona,
        tx_type,
        is_operator,
        use_upstream_url,
        None,
    )
}

/// tx_parameters format
pub fn tx_params(
    config: AppCfg,
    url_opt: Option<Url>,
    waypoint: Option<Waypoint>,
    swarm_path: Option<PathBuf>,
    swarm_persona: Option<String>,
    tx_type: TxType,
    is_operator: bool,
    use_upstream_url: bool,
    wallet_opt: Option<&WalletLibrary>,
) -> Result<TxParams, Error> {
    let url = url_opt.unwrap_or_else(|| {
        config.what_url(use_upstream_url)
    });

    let mut tx_params: TxParams = match swarm_path {
    Some(s) => {
        get_tx_params_from_swarm(
            s,
            swarm_persona.expect("need a swarm 'persona' with credentials in fixtures."),
            is_operator,
        )?
    }, 
     _ => {
        if is_operator {
            get_oper_params(&config, tx_type, url, waypoint)?
        } else {
            // Get from 0L.toml e.g. ~/.0L/0L.toml, or use Profile::default()
            get_tx_params_from_toml(
                config.clone(),
                tx_type,
                wallet_opt,
                url,
                waypoint,
                swarm_path.as_ref().is_some(),
            )?
        }
      }
    };

    if let Some(w) = waypoint {
        tx_params.waypoint = w
    }

    Ok(tx_params)
}

/// Extract params from a local running swarm
pub fn get_tx_params_from_swarm(
    swarm_path: PathBuf,
    swarm_persona: String,
    is_operator: bool,
) -> Result<TxParams, Error> {
    let (url, waypoint) = ol_types::config::get_swarm_rpc_url(swarm_path);
    let mnem = fixtures::get_persona_mnem(&swarm_persona.as_str());
    let keys = KeyScheme::new_from_mnemonic(mnem);

    let keypair = if is_operator {
        KeyPair::from(keys.child_1_operator.get_private_key())
    } else {
        KeyPair::from(keys.child_0_owner.get_private_key())
    };

    let pubkey = keys.child_0_owner.get_public();
    let auth_key = AuthenticationKey::ed25519(&pubkey);
    let address = auth_key.derived_address();

    let tx_params = TxParams {
        auth_key,
        signer_address: address,
        owner_address: address,
        url,
        waypoint,
        keypair,
        tx_cost: TxCost {
            max_gas_unit_for_tx: 100_000,
            coin_price_per_unit: 1, // in micro_gas
            user_tx_timeout: 5_000,
        },

        chain_id: ChainId::new(4),
    };

    println!("Info: Got tx params from swarm");
    Ok(tx_params)
}

/// Form tx parameters struct
pub fn get_oper_params(
    config: &AppCfg,
    tx_type: TxType,
    url: Url,
    wp: Option<Waypoint>,
) -> Result<TxParams, Error> {
    let orig_storage = Storage::OnDiskStorage(OnDiskStorage::new(
        config.workspace.node_home.join("key_store.json").to_owned(),
    ));
    let storage = Storage::NamespacedStorage(Namespaced::new(
        format!("{}-oper", &config.profile.account.to_hex()),
        Box::new(orig_storage),
    ));
    // export_private_key_for_version
    let privkey = storage
        .export_private_key(OPERATOR_KEY)
        .expect("could not parse operator key in key_store.json");

    let keypair = KeyPair::from(privkey);
    let pubkey = &keypair.public_key; // keys.child_0_owner.get_public();
    let auth_key = AuthenticationKey::ed25519(pubkey);

    let waypoint = match wp {
        Some(w) => w,
        None => config.get_waypoint(None)?,
    };

    let tx_cost = config.tx_configs.get_cost(tx_type);
    Ok(TxParams {
        auth_key,
        signer_address: auth_key.derived_address(),
        owner_address: config.profile.account, // address of sender
        url,
        waypoint,
        keypair,
        tx_cost,
        chain_id: ChainId::new(1),
    })
}

/// Gets transaction params from the 0L project root.
pub fn get_tx_params_from_toml(
    config: AppCfg,
    tx_type: TxType,
    wallet_opt: Option<&WalletLibrary>,
    url: Url,
    wp: Option<Waypoint>,
    is_swarm: bool,
) -> Result<TxParams, Error> {
    let (auth_key, address, wallet) = if let Some(wallet) = wallet_opt {
        wallet::get_account_from_wallet(wallet)?
    } else {
        wallet::get_account_from_prompt()
    };

    let waypoint = match wp {
        Some(w) => w,
        None => config.get_waypoint(None)?,
    };
    let keys = KeyScheme::new_from_mnemonic(wallet.mnemonic());
    let keypair = KeyPair::from(keys.child_0_owner.get_private_key());
    let tx_cost = config.tx_configs.get_cost(tx_type);

    let chain_id = if is_swarm {
        ChainId::new(4)
    } else {
        // main net id
        ChainId::new(1)
    };

    let tx_params = TxParams {
        auth_key,
        signer_address: address,
        owner_address: address,
        url,
        waypoint,
        keypair,
        tx_cost: tx_cost.to_owned(),
        // max_gas_unit_for_tx: config.tx_configs.management_txs.max_gas_unit_for_tx,
        // coin_price_per_unit: config.tx_configs.management_txs.coin_price_per_unit, // in micro_gas
        // user_tx_timeout: config.tx_configs.management_txs.user_tx_timeout,
        chain_id,
    };

    Ok(tx_params)
}

/// Gets transaction params from the 0L project root.
pub fn get_tx_params_from_keypair(
    config: AppCfg,
    tx_type: TxType,
    keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey>,
    wp: Option<Waypoint>,
    use_upstream_url: bool,
    is_swarm: bool,
) -> Result<TxParams, Error> {
    let waypoint = match wp {
        Some(w) => w,
        None => config.get_waypoint(None)?,
    };
    let chain_id = if is_swarm {
        ChainId::new(4)
    } else {
        // main net id
        ChainId::new(1)
    };

    let tx_params = TxParams {
        auth_key: config.profile.auth_key,
        signer_address: config.profile.account,
        owner_address: config.profile.account,
        url: config.what_url(use_upstream_url),
        waypoint,
        keypair,
        tx_cost: config.tx_configs.get_cost(tx_type),
        chain_id,
    };

    Ok(tx_params)
}

/// Wait for the response from the diem RPC.
pub fn wait_for_tx(
    signer_address: AccountAddress,
    sequence_number: u64,
    client: &mut DiemClient,
) -> Option<TransactionView> {
    println!(
        "\nAwaiting tx status \nSubmitted from account: {} with sequence number: {}",
        signer_address, sequence_number
    );

    const MAX_ITERATIONS: u8 = 120;

    let mut iter = 0;
    loop {
        thread::sleep(time::Duration::from_millis(3_000));
        // prevent all the logging the client does while
        // it loops through the query.
        stdout().flush().unwrap();

        match &mut client.get_txn_by_acc_seq(&signer_address, sequence_number, false) {
            Ok(Some(txn_view)) => {
                return Some(txn_view.to_owned());
            }
            Err(e) => {
                println!("Response with error: {:?}", e);
            }
            _ => {
                print!(".");
            }
        }
        iter += 1;

        if iter == MAX_ITERATIONS {
            println!("Timeout waiting for response");
            return None;
        }
    }
}

/// Evaluate the response of a submitted txs transaction.
pub fn eval_tx_status(result: TransactionView) -> Result<TransactionView, TxError> {
    match &result.vm_status {
        VMStatusView::Executed => {
            println!("\nSuccess: transaction executed");
            Ok(result)
        },
        VMStatusView::MoveAbort {location, abort_code, explanation: _ } => {
            println!("Transaction failed");
            Err(TxError{
                err: Some(Error::msg(format!("Rejected with code: {:?}", result.vm_status))),
                tx_view: Some(result.clone()),
                location: Some(location.to_string()),
                abort_code: Some(*abort_code),
            })
            // let msg = format!("Rejected with code: {:?}", result.vm_status);
            // let e = Error::msg(msg);
            // Err(e.context(result.vm_status.clone()))
        },
        _ => {
            let msg = format!("Rejected with code: {:?}", result.vm_status);
            let e = Error::msg(msg);
            Err(TxError{
                err: Some(e),
                tx_view: Some(result),
                location: None,
                abort_code: None,
            })
        }
    }
}

impl TxParams {
    /// creates params for unit tests
    pub fn test_fixtures() -> TxParams {
        // This mnemonic is hard coded into the swarm configs. see configs/config_builder
        // let mnem_path = format!("./fixtures/mnemonic/{}.mnem", persona);
        let mnemonic = "talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse".to_string();
        let keys = KeyScheme::new_from_mnemonic(mnemonic);
        let keypair = KeyPair::from(keys.child_0_owner.get_private_key());
        let pubkey = keys.child_0_owner.get_public();
        let signer_auth_key = AuthenticationKey::ed25519(&pubkey);
        let signer_address = signer_auth_key.derived_address();

        let url = Url::parse("http://localhost:8080").unwrap();
        let waypoint: Waypoint =
            "0:732ea2e1c3c5ee892da11abcd1211f22c06b5cf75fd6d47a9492c21dbfc32a46"
                .parse()
                .unwrap();

        TxParams {
            auth_key: signer_auth_key,
            signer_address,
            owner_address: signer_address,
            url,
            waypoint,
            keypair,
            tx_cost: TxCost::new(5_000),
            // max_gas_unit_for_tx: 5_000,
            // coin_price_per_unit: 1, // in micro_gas
            // user_tx_timeout: 5_000,
            chain_id: ChainId::new(4), // swarm/testnet
        }
    }
}
