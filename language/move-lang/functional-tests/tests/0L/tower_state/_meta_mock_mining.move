//! account: alice, 1, 0, validator

//! new-transaction
//! sender: alice
script {
    use 0x1::TowerState;
    // use 0x1::Signer;

    fun main(sender: signer) {
        // TowerState::init_miner_state(sender);
        TowerState::test_helper_mock_mining(&sender, 5);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    use 0x1::TowerState;
    
    fun main(sender: signer) {
        assert(TowerState::get_count_in_epoch(@{{alice}}) == 5, 73570001);
        TowerState::test_helper_mock_reconfig(&sender, @{{alice}});
        assert(TowerState::get_epochs_mining(@{{alice}}) == 1, 73570002);
    }
}
//check: EXECUTED