// Module to test bulk validator updates function in LibraSystem.move
//! account: alice, 1000000, 0, validator

//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  use 0x1::TestFixtures;
  use 0x1::VDF;
  use 0x1::ValidatorConfig;
  use 0x1::Roles;

  fun main(_sender: &signer) {
  let challenge = TestFixtures::alice_1_easy_chal();
  let solution = TestFixtures::alice_1_easy_sol();
  let (parsed_address, _auth_key_prefix) = VDF::extract_address_from_challenge(&challenge);


  LibraAccount::create_validator_account_with_proof(
    &challenge,
    &solution,
    x"8108aedfacf5cf1d73c67b6936397ba5fa72817f1b5aab94658238ddcdc08010", // consensus_pubkey: vector<u8>,
    b"192.168.0.1", // validator_network_addresses: vector<u8>,
    b"192.168.0.1", // fullnode_network_addresses: vector<u8>,
    x"1ee7", // human_name: vector<u8>,
  );

  // Check the account has the Validator role

  assert(Roles::assert_validator_addr(parsed_address), 7357130101011000);


  assert(ValidatorConfig::is_valid(parsed_address), 7357130101021000);
  // Check the account exists and the balance is 0
  assert(LibraAccount::balance<GAS>(parsed_address) == 0, 7357130101031000);
  }
}
//check: EXECUTED