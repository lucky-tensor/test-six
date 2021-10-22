//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

//! new-transaction
script{
  use 0x1::VDF;
  use 0x1::TestFixtures;
  fun main() {
    // this tests the happy case, that a proof is submitted with all three 
    // correct parameters.
    
    let challenge = TestFixtures::alice_0_hard_chal();
    // Generate solutions with:
    // cd ./verfiable_delay/vdf-cli && cargo run --release -- -l=512 aa 100 -tpietrzak
    // NOTE: the -l=512 is important because this is the security paramater of 0L miner.
    let proof = TestFixtures::alice_0_hard_sol();

    assert(VDF::verify(&challenge, &proof, &TestFixtures::hard_difficulty(), &TestFixtures::security()) == true, 1);
  }
}
