///////////////////////////////////////////////////////////////////////////
// 0L Module
// Epoch Prologue
///////////////////////////////////////////////////////////////////////////
// The prologue for transitioning to next epoch after every n blocks.
// File Prefix for errors: 1801
///////////////////////////////////////////////////////////////////////////

address 0x1 {
module Reconfigure {
    use 0x1::Signer;
    use 0x1::CoreAddresses;
    use 0x1::Subsidy;
    use 0x1::NodeWeight;
    use 0x1::LibraSystem;
    use 0x1::MinerState;
    use 0x1::Globals;
    use 0x1::Vector;
    use 0x1::Stats;
    use 0x1::ValidatorUniverse;
    use 0x1::AutoPay;
    use 0x1::Epoch;
    use 0x1::FullnodeState;


    // This function is called by block-prologue once after n blocks.
    // Function code: 01. Prefix: 180101
    public fun reconfigure(vm: &signer, height_now: u64) {
        assert(Signer::address_of(vm) == CoreAddresses::LIBRA_ROOT_ADDRESS(), 180101014010);
        
        // Fullnode subsidy
        // loop through validators and pay full node subsidies.
        // Should happen before transactionfees get distributed.
        let miners = ValidatorUniverse::get_eligible_validators(vm);
        let global_proofs_count = 0;
        let k = 0;
        while (k < Vector::length(&miners)) {
            let addr = *Vector::borrow(&miners, k);

            let count = FullnodeState::get_address_proof_count(addr);
            global_proofs_count = global_proofs_count + count;
            
            let value: u64;
            // check if is in onboarding state (or stuck)
            if (FullnodeState::is_onboarding(addr)) {
                value = Subsidy::distribute_onboarding_subsidy(vm, addr);
            } else {
                value = Subsidy::distribute_fullnode_subsidy(vm, addr, count);
            };

            FullnodeState::inc_payment_count(vm, addr, count);
            FullnodeState::inc_payment_value(vm, addr, value);
            FullnodeState::reconfig(vm, addr);

            k = k + 1;
        };

        // Process outgoing validators:
        // Distribute Transaction fees and subsidy payments to all outgoing validators
        let height_start = Epoch::get_timer_height_start(vm);

        let (outgoing_set, fee_ratio) = LibraSystem::get_fee_ratio(vm, height_start, height_now);

        if (Vector::length<address>(&outgoing_set) > 0) {
            let subsidy_units = Subsidy::calculate_subsidy(vm, height_start, height_now);

            if (subsidy_units > 0) {
                Subsidy::process_subsidy(vm, subsidy_units, &outgoing_set, &fee_ratio);
            };
            Subsidy::process_fees(vm, &outgoing_set, &fee_ratio);
        };
        // Propose upcoming validator set:
        // Step 1: Sort Top N eligible validators
        // Step 2: Jail non-performing validators
        // Step 3: Reset counters
        // Step 4: Bulk update validator set (reconfig)

        // prepare_upcoming_validator_set(vm);
        let top_accounts = NodeWeight::top_n_accounts(
            vm, Globals::get_max_validator_per_epoch());
        let jailed_set = LibraSystem::get_jailed_set(vm, height_start, height_now);

        let proposed_set = Vector::empty();
        let i = 0;
        while (i < Vector::length(&top_accounts)) {
            let addr = *Vector::borrow(&top_accounts, i);
            let mined_last_epoch = MinerState::node_above_thresh(vm, addr);
            // TODO: temporary until jail-refactor merge.
            if ((!Vector::contains(&jailed_set, &addr)) && mined_last_epoch) {
                Vector::push_back(&mut proposed_set, addr);
            };
            i = i+ 1;
        };

        // If the cardinality of validator_set in the next epoch is less than 4, we keep the same validator set. 
        if (Vector::length<address>(&proposed_set)<= 3) proposed_set = ValidatorUniverse::get_eligible_validators(vm);
        // Usually an issue in staging network for QA only.
        // This is very rare and theoretically impossible for network with at least 6 nodes and 6 rounds. If we reach an epoch boundary with at least 6 rounds, we would have at least 2/3rd of the validator set with at least 66% liveliness. 


        // needs to be set before the auctioneer runs in Subsidy::fullnode_reconfig
        Subsidy::set_global_count(vm, global_proofs_count);

        //Reset Counters
        Stats::reconfig(vm, &proposed_set);
        MinerState::reconfig(vm);

        // Reconfigure the network
        LibraSystem::bulk_update_validators(vm, proposed_set);
        // reset clocks
        Subsidy::fullnode_reconfig(vm);
        AutoPay::reconfig_reset_tick(vm);
        Epoch::reset_timer(vm, height_now);
    }
}
}