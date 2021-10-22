//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

//! new-transaction
script{
  use 0x1::VDF;
  use 0x1::TestFixtures;
  use 0x1::Debug::print;
  fun main() {
    // this tests the happy case, that a proof is submitted with all three 
    // correct parameters.

    let wrong_difficulty = 100;

    let challenge = TestFixtures::alice_0_hard_chal();
    // Generate solutions with:
    // cd ./verfiable_delay/vdf-cli && cargo run --release -- -l=512 aa 100 -tpietrzak
    // NOTE: the -l=512 is important because this is the security paramater of 0L miner.
    let proof = TestFixtures::alice_0_hard_sol();

    let res = VDF::verify(
      &challenge,
      &proof,
      &wrong_difficulty,
      &TestFixtures::security()
    );
    
    print(&res);
    assert(res == false, 1);
  }
}