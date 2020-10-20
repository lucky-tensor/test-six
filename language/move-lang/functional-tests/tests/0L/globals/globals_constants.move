//! account: alice, 100000,0, validator
//! new-transaction
//! sender: libraroot
script {
use 0x1::Globals;
use 0x1::Testnet;
use 0x1::LibraSystem;

    fun main(_sender: &signer) {
        assert(LibraSystem::is_validator({{alice}}) == true, 98);

        let len = Globals::get_epoch_length();
        // Debug::print(&len);
        let set = LibraSystem::validator_set_size();
        // Debug::print(&set);

        assert(set == 1u64, 73570001);

        if (Testnet::is_testnet()){
            assert(len == 15u64, 73570001);
        } else {
            assert(len == 196992u64, 73570001);
        }
    }
}
// check: EXECUTED