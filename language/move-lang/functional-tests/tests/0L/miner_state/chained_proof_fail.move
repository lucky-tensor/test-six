//! account: dummy-prevents-genesis-reload, 100000 ,0, validator
//! account: alice, 10000000GAS

// Alice Submit VDF Proof
//! new-transaction
//! sender: alice
script {
    use 0x1::TowerState;
    use 0x1::TestFixtures;

    // SIMULATES A MINER 0 PROOF ADDED IN GENESIS (block_0.json)
    // The first transaction should succeed, but the second sends a valid 
    // vdf proof but is not matched to previous proof. 
    fun main(sender: signer) {
        let difficulty = 100;
        let height_after = 0;

        // return solution
        TowerState::test_helper_init_miner(
            &sender,
            difficulty,
            TestFixtures::alice_0_easy_chal(),
            TestFixtures::alice_0_easy_sol()
        );

        // check for initialized TowerState
        let verified_tower_height_after = TowerState::test_helper_get_height(@{{alice}});

        assert(verified_tower_height_after == height_after, 10008001);
    }
}
// check: EXECUTED


//! new-transaction
//! sender: alice
script {
    use 0x1::TowerState;
    use 0x1::TestFixtures;

    // SIMULATES THE SECOND PROOF OF THE MINER (block_1.json)
    fun main(sender: signer) {
        let difficulty = 100u64;
        assert(TowerState::test_helper_get_height(@{{alice}}) == 0, 10008001);
        let height_after = 1;
        
        let proof = TowerState::create_proof_blob(
            // a correct pair, but does not match the previous proof alice sent.
            TestFixtures::easy_chal(),
            difficulty,
            TestFixtures::easy_sol()
        );
        TowerState::commit_state(&sender, proof);

        let verified_height = TowerState::test_helper_get_height(@{{alice}});
        assert(verified_height == height_after, 10008002);
    }
}
// check: ABORTED
