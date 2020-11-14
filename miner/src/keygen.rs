//! Key generation
use libra_wallet::WalletLibrary;
use libra_types::{
  account_address::AccountAddress,
  transaction::authenticator::AuthenticationKey
};


/// Genereates keys from WalletLibrary, updates a MinerConfig
pub fn keygen() -> (AuthenticationKey, AccountAddress) {
        // Generate new keys
        let mut wallet = WalletLibrary::new();
        let mnemonic_string = wallet.mnemonic();
        // NOTE: Authkey uses the child number 0 by default
        let (auth_key, _) = wallet.new_address().expect("Could not generate address");
        let account = auth_key.derived_address();
        //////////////// Info ////////////////
        
        println!("0L Auth Key:\n\
        You will need this in your miner.toml configs.\n\
        ---------\n\
        {:?}\n", &auth_key.to_string());

        println!("0L Address:\n\
        This address is derived from your Auth Key, it has not yet been created on chain. You'll need to submit a genesis miner proof for that.\n\
        ---------\n\
        {:?}\n", &account);

        println!("0L mnemonic:\n\
        WRITE THIS DOWN NOW. This is the last time you will see this mnemonic. It is not saved anywhere. Nobody can help you if you lose it.\n\
        ---------\n\
        {}\n", &mnemonic_string.as_str());

        (auth_key, account)
}


#[test]
fn wallet() { 
    use libra_wallet::Mnemonic;
    // let mut wallet = WalletLibrary::new();

    let mut wallet = WalletLibrary::new();

    let (auth_key, child_number) = wallet.new_address().expect("Could not generate address");
    let mnemonic_string = wallet.mnemonic(); //wallet

    println!("auth_key:\n{:?}", auth_key.to_string());
    println!("child_number:\n{:?}", child_number);
    println!("mnemonic:\n{}", mnemonic_string);

    // let mnemonic_string = r#"average list time circle item couch resemble tool diamond spot winter pulse cloth laundry slice youth payment cage neutral bike armor balance way ice"#;

    let mut wallet = WalletLibrary::new_from_mnemonic(Mnemonic::from(&mnemonic_string).unwrap());

    // println!("wallet\n:{:?}", wallet);

    let (main_addr, child_number ) = wallet.new_address().unwrap();
    println!("wallet\n:{:?} === {:x}", child_number, main_addr);

    let vec_addresses = wallet.get_addresses().unwrap();

    println!("vec_addresses\n:{:?}", vec_addresses);

    // Expect this to be zero before we haven't populated the address map in the repo
    assert!(vec_addresses.len() == 1);
    // Empty hashmap should be fine
    // let mut vec_account_data = Vec::new();
}