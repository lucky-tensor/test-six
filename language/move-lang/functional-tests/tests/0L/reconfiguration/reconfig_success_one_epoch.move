// Case 1: Validators are compliant. 
// This test is to check if validators are present after the first epoch.
// Here EPOCH-LENGTH = 15 Blocks.
// NOTE: This test will fail with Staging and Production Constants, only for Debug - due to epoch length.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator
//! account: eve, 1000000, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: libraroot
script {
    
    use 0x1::LibraSystem;
    fun main(_account: &signer) {
        // Tests on initial size of validators 
        assert(LibraSystem::validator_set_size() == 5, 7357000180101);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(LibraSystem::is_validator({{bob}}) == true, 7357000180103);
    }
}
// check: EXECUTED

//! block-prologue
//! proposer: alice
//! block-time: 2

//! block-prologue
//! proposer: alice
//! block-time: 3

//! block-prologue
//! proposer: alice
//! block-time: 4

//! block-prologue
//! proposer: alice
//! block-time: 5

//! block-prologue
//! proposer: alice
//! block-time: 6

//! block-prologue
//! proposer: alice
//! block-time: 7

//! block-prologue
//! proposer: alice
//! block-time: 8

//! block-prologue
//! proposer: alice
//! block-time: 9

//! block-prologue
//! proposer: alice
//! block-time: 10

//! block-prologue
//! proposer: alice
//! block-time: 11

//! block-prologue
//! proposer: alice
//! block-time: 12

//! block-prologue
//! proposer: alice
//! block-time: 13

//! block-prologue
//! proposer: alice
//! block-time: 14

//! new-transaction
//! sender: libraroot
script {
    
    use 0x1::LibraSystem;
    fun main(_account: &signer) {
        // Tests on initial size of validators 
        assert(LibraSystem::validator_set_size() == 5, 7357000180104);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357000180105);
        assert(LibraSystem::is_validator({{bob}}) == true, 7357000180106);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Vector;
    use 0x1::Stats;
    // This is the the epoch boundary.
    fun main(vm: &signer) {
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});

        // Overwrite the statistics to mock that all have been validating.
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };
    }
}
//check: EXECUTED

//! block-prologue
//! proposer: alice
//! block-time: 15
//! round: 15

//////////////////////////////////////////////
///// CHECKS RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////