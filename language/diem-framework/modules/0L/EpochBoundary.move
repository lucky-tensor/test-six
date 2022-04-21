///////////////////////////////////////////////////////////////////////////
// 0L Module
// Epoch Prologue
///////////////////////////////////////////////////////////////////////////
// The prologue for transitioning to next epoch after every n blocks.
// File Prefix for errors: 1800
///////////////////////////////////////////////////////////////////////////


address 0x1 {
module EpochBoundary {
    use 0x1::CoreAddresses;
    use 0x1::Subsidy;
    use 0x1::NodeWeight;
    use 0x1::DiemSystem;
    use 0x1::TowerState;
    use 0x1::Globals;
    use 0x1::Vector;
    use 0x1::Stats;
    use 0x1::AutoPay;
    use 0x1::Epoch;
    use 0x1::DiemConfig;
    use 0x1::Audit;
    use 0x1::DiemAccount;
    use 0x1::Burn;
    use 0x1::FullnodeSubsidy;
    use 0x1::ValidatorUniverse;
    use 0x1::Debug::print;

    struct DebugMode has copy, key, drop, store{
      fixed_set: vector<address>
    }

    // private function so that it can only be called by vm session.
    // should never be used in production.
    fun init_debug(vm: &signer, vals: vector<address>) {
      if (!is_debug()) {
        move_to<DebugMode>(vm, DebugMode {
          fixed_set: vals
        });
      }
    }

    fun remove_debug(vm: &signer) acquires DebugMode {
      CoreAddresses::assert_vm(vm);
      if (is_debug()) {
        _ = move_from<DebugMode>(CoreAddresses::VM_RESERVED_ADDRESS());
      }
    }

    fun is_debug(): bool {
      exists<DebugMode>(CoreAddresses::VM_RESERVED_ADDRESS())
    }

    fun get_debug_vals(): vector<address> acquires DebugMode  {
      if (is_debug()) {
        let d = borrow_global<DebugMode>(CoreAddresses::VM_RESERVED_ADDRESS());
        *&d.fixed_set
      } else {
        Vector::empty<address>()
      }
    }

    // use 0x1::Debug::print;
    // This function is called by block-prologue once after n blocks.
    // Function code: 01. Prefix: 180001
    public fun reconfigure(vm: &signer, height_now: u64) acquires DebugMode{
        print(&300300);

        CoreAddresses::assert_vm(vm);
        let height_start = Epoch::get_timer_height_start(vm);
        print(&300310);
        let (outgoing_compliant_set, _) = 
            DiemSystem::get_fee_ratio(vm, height_start, height_now);
        print(&300320);

        // NOTE: This is "nominal" because it doesn't check
        let compliant_nodes_count = Vector::length(&outgoing_compliant_set);
        print(&300330);

        let (subsidy_units, nominal_subsidy_per) = 
            Subsidy::calculate_subsidy(vm, compliant_nodes_count);
        print(&300340);
        process_fullnodes(vm, nominal_subsidy_per);
        print(&300350);

        process_validators(vm, subsidy_units, *&outgoing_compliant_set);
        print(&300360);

        let proposed_set = propose_new_set(vm, height_start, height_now);
        
        // Update all slow wallet limits
        DiemAccount::slow_wallet_epoch_drip(vm, Globals::get_unlock());

        proof_of_burn(vm,nominal_subsidy_per);
        
        reset_counters(vm, proposed_set, outgoing_compliant_set, height_now)
    }

    // process fullnode subsidy
    fun process_fullnodes(vm: &signer, nominal_subsidy_per_node: u64) {
        // Fullnode subsidy
        // loop through validators and pay full node subsidies.
        // Should happen before transactionfees get distributed.
        // Note: need to check, there may be new validators which have not mined yet.
        let miners = TowerState::get_miner_list();
        // fullnode subsidy is a fraction of the total subsidy available to validators.
        let proof_price = FullnodeSubsidy::get_proof_price(nominal_subsidy_per_node);

        let k = 0;
        // Distribute mining subsidy to fullnodes
        while (k < Vector::length(&miners)) {
            let addr = *Vector::borrow(&miners, k);
            if (DiemSystem::is_validator(addr)) { // skip validators
              k = k + 1;
              continue
            };
            
            // TODO: this call is repeated in propose_new_set. 
            // Not sure if the performance hit at epoch boundary is worth the refactor. 
            if (TowerState::node_above_thresh(addr)) {
              let count = TowerState::get_count_above_thresh_in_epoch(addr);

              let miner_subsidy = count * proof_price;
              FullnodeSubsidy::distribute_fullnode_subsidy(vm, addr, miner_subsidy);
            };

            k = k + 1;
        };
    }

    fun process_validators(
        vm: &signer, subsidy_units: u64, outgoing_compliant_set: vector<address>
    ) {
        // Process outgoing validators:
        // Distribute Transaction fees and subsidy payments to all outgoing validators
        
        if (Vector::is_empty<address>(&outgoing_compliant_set)) return;

        if (subsidy_units > 0) {
            Subsidy::process_subsidy(vm, subsidy_units, &outgoing_compliant_set);
        };

        Subsidy::process_fees(vm, &outgoing_compliant_set);
    }

    fun propose_new_set(vm: &signer, height_start: u64, height_now: u64): vector<address> acquires DebugMode{
        // Propose upcoming validator set:
        
        // in emergency admin roles set the validator set
        if (is_debug()) {
          return get_debug_vals()
        };

        // save all the eligible list, before the jailing removes them.
        let proposed_set = Vector::empty();

        let top_accounts = NodeWeight::top_n_accounts(
            vm, Globals::get_max_validators_per_set()
        );

        let jailed_set = DiemSystem::get_jailed_set(vm, height_start, height_now);



        let i = 0;
        while (i < Vector::length<address>(&top_accounts)) {
            let addr = *Vector::borrow(&top_accounts, i);
            let mined_last_epoch = TowerState::node_above_thresh(addr);
            // TODO: temporary until jailing is enabled.
            if (
                !Vector::contains(&jailed_set, &addr) && 
                mined_last_epoch &&
                Audit::val_audit_passing(addr)
            ) {
                Vector::push_back(&mut proposed_set, addr);
            };
            i = i+ 1;
        };


        // If the cardinality of validator_set in the next epoch is less than 4, 

        // if we are failing to qualify anyone. Pick top 1/2 of validator set by proposals. They are probably online.

        if (Vector::length<address>(&proposed_set) <= 3) proposed_set = Stats::get_sorted_vals_by_props(vm, Vector::length<address>(&proposed_set) / 2);


        // If still failing...in extreme case if we cannot qualify anyone. Don't change the validator set.
        // we keep the same validator set. 
        if (Vector::length<address>(&proposed_set) <= 3) proposed_set = DiemSystem::get_val_set_addr(); // Patch for april incident. Make no changes to validator set.

        // Usually an issue in staging network for QA only.
        // This is very rare and theoretically impossible for network with 
        // at least 6 nodes and 6 rounds. If we reach an epoch boundary with 
        // at least 6 rounds, we would have at least 2/3rd of the validator 
        // set with at least 66% liveliness. 
        proposed_set
    }

    fun reset_counters(vm: &signer, proposed_set: vector<address>, outgoing_compliant: vector<address>, height_now: u64) {

        // Reset Stats
        Stats::reconfig(vm, &proposed_set);

        // Migrate TowerState list from elegible.
        TowerState::reconfig(vm, &outgoing_compliant);

        // Reconfigure the network
        DiemSystem::bulk_update_validators(vm, proposed_set);

        // process community wallets
        DiemAccount::process_community_wallets(vm, DiemConfig::get_current_epoch());
        
        // reset counters
        AutoPay::reconfig_reset_tick(vm);
        Epoch::reset_timer(vm, height_now);
    }

    // NOTE: this was previously in propose_new_set since it used the same loop.
    // copied implementation from Teams proposal.
    fun proof_of_burn(vm: &signer, nominal_subsidy_per: u64) {
        CoreAddresses::assert_vm(vm);
        Burn::reset_ratios(vm);

        let burn_value = nominal_subsidy_per / 2; // 50% of the current per validator reward
        let all_vals = ValidatorUniverse::get_eligible_validators(vm);
        print(&all_vals);
        let i = 0;
        while (i < Vector::length<address>(&all_vals)) {
          let addr = *Vector::borrow(&all_vals, i);
          print(&addr);

          Burn::epoch_start_burn(vm, addr, burn_value);
          i = i + 1;
        };
    }
}
}