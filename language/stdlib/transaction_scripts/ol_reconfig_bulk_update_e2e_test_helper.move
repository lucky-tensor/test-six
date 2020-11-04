script {
    use 0x1::LibraSystem;
    use 0x1::Vector;
    use 0x1::ValidatorUniverse;
    fun ol_reconfig_bulk_update_e2e_test_helper(account: &signer, alice: address, bob: address, carol: address,
        sha: address, _ram: address) {
        // Create vector of validators and add the desired new validator set
        let vec = Vector::empty();
        Vector::push_back<address>(&mut vec, alice);
        ValidatorUniverse::add_validator(alice);
        Vector::push_back<address>(&mut vec, bob);
        ValidatorUniverse::add_validator(bob);
        Vector::push_back<address>(&mut vec, carol);
        ValidatorUniverse::add_validator(carol);
        assert(Vector::length<address>(&vec) == 3, 5);

        // Update the validator set
        LibraSystem::bulk_update_validators(account, vec);

        // Assert that updates happened correctly
        assert(LibraSystem::validator_set_size() == 3, 6);
        assert(LibraSystem::is_validator(sha) == false, 7);
        assert(LibraSystem::is_validator(bob) == true, 8);
    }
}
