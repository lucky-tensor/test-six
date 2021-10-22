//! account: alice, 1000000GAS, 0, validator
//! account: bob, 1000000GAS, 0
//! account: carol, 1000000GAS, 0



//! new-transaction
//! sender: bob
script {
    use 0x1::TowerState;
    use 0x1::TestFixtures;

    fun main(sender: signer) {
        // add one proof and init the state.
        TowerState::test_helper_init_miner(
            &sender,
            TestFixtures::easy_chal(),
            TestFixtures::easy_sol(),
            TestFixtures::easy_difficulty(),
            TestFixtures::security(),
        );
    }
}

//! new-transaction
//! sender: carol
script {
    use 0x1::TowerState;
    use 0x1::TestFixtures;

    fun main(sender: signer) {
        // add one proof and init the state.
        TowerState::test_helper_init_miner(
            &sender,
            TestFixtures::easy_chal(),
            TestFixtures::easy_sol(),
            TestFixtures::easy_difficulty(),
            TestFixtures::security(),
        );
    }
}

// Clear the clocks

//! new-transaction
//! sender: diemroot
script {
    use 0x1::TowerState;

    fun main(vm: signer) {
      TowerState::epoch_reset(&vm);
    }
}


//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

// First, Make Alice a Case 1 validator so that there is a subsidy to be paid to validator set.

//! new-transaction
//! sender: alice
script {
    use 0x1::DiemSystem;
    use 0x1::TowerState;
    use 0x1::NodeWeight;
    use 0x1::GAS::GAS;
    use 0x1::DiemAccount;

    fun main(sender: signer) {
        // Tests on initial size of validators
        // assert(DiemSystem::validator_set_size() == 5, 7357300101011000);
        assert(DiemSystem::is_validator(@{{alice}}) == true, 735701);

        assert(TowerState::get_count_in_epoch(@{{alice}}) == 1, 735702);
        assert(DiemAccount::balance<GAS>(@{{alice}}) == 1000000, 735703);
        assert(NodeWeight::proof_of_weight(@{{alice}}) == 0, 735704);

        // Alice continues to mine after genesis.
        // This test is adapted from chained_from_genesis.move
        TowerState::test_helper_mock_mining(&sender, 5);
        assert(TowerState::get_count_in_epoch(@{{alice}}) == 5, 735705);

    }
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::Stats;

    // This is the the epoch boundary.
    fun main(vm: signer) {
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, @{{alice}});

        // Overwrite the statistics to mock that all have been validating.
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(&vm, &voters);
            i = i + 1;
        };
    }
}
//check: EXECUTED



//! new-transaction
//! sender: bob
script {
    use 0x1::DiemSystem;
    use 0x1::TowerState;
    use 0x1::Debug::print;

    fun main(sender: signer) {
        // Tests on initial size of validators
        assert(DiemSystem::is_validator(@{{alice}}), 735706);
        assert(!DiemSystem::is_validator(@{{bob}}), 735707);
        
        print(&TowerState::get_count_in_epoch(@{{bob}}));

        // bring bob to 10 proofs. (Note: alice has one proof as a fullnode from genesis, so it will total 11 fullnode proofs.);
        TowerState::test_helper_mock_mining(&sender, 10);

        // assert(TowerState::get_count_in_epoch(@{{bob}}) == 1, 7357300101041000);
        print(&TowerState::get_count_in_epoch(@{{bob}}));
        print(&TowerState::get_fullnode_proofs());
    }
}
// check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::TowerState;

    fun main(sender: signer) {

        // bring bob to 10 proofs. (Note: alice has one proof as a fullnode from genesis, so it will total 11 fullnode proofs.);
        TowerState::test_helper_mock_mining(&sender, 10);
    }
}
// check: EXECUTED

//////////////////////////////////////////////
///// Trigger reconfiguration at 61 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 61000000
//! round: 15

///// TEST RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////

//! new-transaction
//! sender: diemroot
script {  
    // use 0x1::NodeWeight;
    use 0x1::GAS::GAS;
    use 0x1::DiemAccount;
    use 0x1::Subsidy;
    use 0x1::Globals;
    // use 0x1::Debug::print;

    fun main(_vm: signer) {
        // We are in a new epoch.

        let expected_subsidy = Subsidy::subsidy_curve(
          Globals::get_subsidy_ceiling_gas(),
          0,
          Globals::get_max_validators_per_set(),
        );

        let starting_balance = 1000000;

        let ending_balance = starting_balance + expected_subsidy/2;
        // bob gets the whole subsidy

        // bob and carol submitted same number of proofs and will share the fullnode subsidy
        assert(DiemAccount::balance<GAS>(@{{bob}}) == ending_balance, 7357000180113);
        assert(DiemAccount::balance<GAS>(@{{bob}}) == ending_balance, 7357000180114);  
    }
}
//check: EXECUTED