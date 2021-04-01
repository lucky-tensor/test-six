///////////////////////////////////////////////////////////////////////////
// 0L Module
// Subsidy 
///////////////////////////////////////////////////////////////////////////
// The logic for determining the appropriate level of subsidies at a given time in the network
// File Prefix for errors: 1901
///////////////////////////////////////////////////////////////////////////

address 0x1 {
  module Subsidy {
    use 0x1::CoreAddresses;
    use 0x1::GAS::GAS;
    use 0x1::Libra;
    use 0x1::Signer;
    use 0x1::LibraAccount;
    use 0x1::LibraSystem;
    use 0x1::Vector;
    use 0x1::FixedPoint32::{Self, FixedPoint32};    
    use 0x1::Stats;
    use 0x1::ValidatorUniverse;
    use 0x1::Globals;
    use 0x1::LibraTimestamp;
    use 0x1::TransactionFee;
    use 0x1::Roles;
    use 0x1::Testnet::is_testnet;
    // use 0x1::StagingNet::is_staging_net;    
    // Method to calculate subsidy split for an epoch.
    // This method should be used to get the units at the beginning of the epoch.

        // Function code: 03 Prefix: 190103
    public fun process_subsidy(
      vm_sig: &signer,
      subsidy_units: u64,
      outgoing_set: &vector<address>,
      _fee_ratio: &vector<FixedPoint32>) {
      let sender = Signer::address_of(vm_sig);
      assert(sender == CoreAddresses::LIBRA_ROOT_ADDRESS(), 190101034010);

      // Get the split of payments from Stats.
      let len = Vector::length<address>(outgoing_set);

      //TODO: assert the lengths of vectors are the same.
      let i = 0;
      while (i < len) {

        let node_address = *(Vector::borrow<address>(outgoing_set, i));
        // let node_ratio = *(Vector::borrow<FixedPoint32>(fee_ratio, i));
        
        let subsidy_granted = 0;
        if (subsidy_units > len) {
          subsidy_granted = subsidy_units/len;
        };
        // should not be possible
        if (subsidy_granted == 0) break;
        // Transfer gas from vm address to validator
        let minted_coins = Libra::mint<GAS>(vm_sig, subsidy_granted);
        LibraAccount::vm_deposit_with_metadata<GAS>(
          vm_sig,
          node_address,
          minted_coins,
          x"",
          x""
        );
        i = i + 1;
      };
    }


    // Function code: 07 Prefix: 190107
    public fun calculate_subsidy(vm: &signer, height_start: u64, height_end: u64):u64 {

      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::LIBRA_ROOT_ADDRESS(), 190101014010);

      // skip genesis
      assert(!LibraTimestamp::is_genesis(), 190101021000);

      // Gets the transaction fees in the epoch
      let txn_fee_amount = TransactionFee::get_amount_to_distribute(vm);

      // Calculate the split for subsidy and burn
      let subsidy_ceiling_gas = Globals::get_subsidy_ceiling_gas();
      let network_density = Stats::network_density(vm, height_start, height_end);
      let max_node_count = Globals::get_max_node_density();
      let guaranteed_minimum = subsidy_curve(
        subsidy_ceiling_gas,
        network_density,
        max_node_count,
      );

      // deduct transaction fees from guaranteed minimum.
      if (guaranteed_minimum > txn_fee_amount ){
        return guaranteed_minimum - txn_fee_amount
      };
      0u64
    }

    // Function code: 04 Prefix: 190104
    public fun subsidy_curve(
      subsidy_ceiling_gas: u64,
      network_density: u64,
      max_node_count: u64
      ): u64 {
      
      let min_node_count = 4u64;

      // Return early if we know the value is below 4.
      // This applies only to test environments where there is network of 1.
      if (network_density <= min_node_count) {
        return subsidy_ceiling_gas
      };

      let slope = FixedPoint32::divide_u64(
        subsidy_ceiling_gas,
        FixedPoint32::create_from_rational(max_node_count - min_node_count, 1)
        );
      //y-intercept
      let intercept = slope * max_node_count;
      //calculating subsidy and burn units
      // NOTE: confirm order of operations here:
      let guaranteed_minimum = intercept - slope * network_density;
      guaranteed_minimum
    }

    // Function code: 06 Prefix: 190106
    public fun genesis(vm_sig: &signer) acquires FullnodeSubsidy{
      //Need to check for association or vm account
      let vm_addr = Signer::address_of(vm_sig);
      assert(vm_addr == CoreAddresses::LIBRA_ROOT_ADDRESS(), 190101044010);

      // Get eligible validators list
      let genesis_validators = ValidatorUniverse::get_eligible_validators(vm_sig);
      let len = Vector::length(&genesis_validators);

      let i = 0;
      while (i < len) {

        let node_address = *(Vector::borrow<address>(&genesis_validators, i));
        let old_validator_bal = LibraAccount::balance<GAS>(node_address);
        let count_proofs = 1;

        if (is_testnet()) {
          // start with sufficient gas for expensive tests e.g. upgrade
          count_proofs = 10;
        };
        
        let subsidy_granted = distribute_fullnode_subsidy(vm_sig, node_address, count_proofs, true);
        //Confirm the calculations, and that the ending balance is incremented accordingly.

        assert(LibraAccount::balance<GAS>(node_address) == old_validator_bal + subsidy_granted, 19010105100);

        i = i + 1;
      };
    }
    
    public fun process_fees(
      vm: &signer,
      outgoing_set: &vector<address>,
      _fee_ratio: &vector<FixedPoint32>,
    ){
      assert(Signer::address_of(vm) == CoreAddresses::LIBRA_ROOT_ADDRESS(), 190103014010);
      let capability_token = LibraAccount::extract_withdraw_capability(vm);

      let len = Vector::length<address>(outgoing_set);

      let bal = TransactionFee::get_amount_to_distribute(vm);
    // leave fees in tx_fee if there isn't at least 1 gas coin per validator.
      if (bal < len) {
        LibraAccount::restore_withdraw_capability(capability_token);
        return
      };

      let i = 0;
      while (i < len) {
        let node_address = *(Vector::borrow<address>(outgoing_set, i));
        // let node_ratio = *(Vector::borrow<FixedPoint32>(fee_ratio, i));
        let fees = bal/len;
        
        LibraAccount::vm_deposit_with_metadata<GAS>(
            vm,
            node_address,
            TransactionFee::get_transaction_fees_coins_amount<GAS>(vm, fees),
            x"",
            x""
        );
        i = i + 1;
      };
      LibraAccount::restore_withdraw_capability(capability_token);
    }

    //////// FULLNODE /////////

    resource struct FullnodeSubsidy {
        previous_epoch_proofs: u64,
        current_proof_price: u64,
        current_cap: u64,
        current_subsidy_distributed: u64,
        current_proofs_verified: u64,
    }

    public fun init_fullnode_sub(vm: &signer) {
      let genesis_validators = LibraSystem::get_val_set_addr();
      let validator_count = Vector::length(&genesis_validators);
      if (validator_count < 10) validator_count = 10;
      // baseline_cap: baseline units per epoch times the mininmum as used in tx, times minimum gas per unit.

      // estimated gas unit cost for proof verification divided coin scaling factor
      // Cost for verification test/easy difficulty: 1173 / 1000000
      // Cost for verification prod/hard difficulty: 2294 / 1000000
      // Cost for account creation prod/hard: 4336

      let baseline_tx_cost = 4336; // microgas
      let ceiling = baseline_auction_units() * baseline_tx_cost * validator_count;

      Roles::assert_libra_root(vm);
      assert(!exists<FullnodeSubsidy>(Signer::address_of(vm)), 130112011021);
      move_to<FullnodeSubsidy>(vm, FullnodeSubsidy{
        previous_epoch_proofs: 0u64,
        current_proof_price: baseline_tx_cost * 24 * 8 * 3, // number of proof submisisons in 3 initial epochs.
        current_cap: ceiling,
        current_subsidy_distributed: 0u64,
        current_proofs_verified: 0u64,
      });
      }

    public fun distribute_fullnode_subsidy(vm: &signer, miner: address, count: u64, is_genesis: bool ):u64 acquires FullnodeSubsidy{
      Roles::assert_libra_root(vm);
      // only for fullnodes, ie. not in current validator set.
      if (!is_genesis){
        if (LibraSystem::is_validator(miner)) return 0;
      };
      let state = borrow_global_mut<FullnodeSubsidy>(Signer::address_of(vm));
      // fail fast, abort if ceiling was met
      if (state.current_subsidy_distributed > state.current_cap) return 0;
      let proposed_subsidy = state.current_proof_price * count;
      if (proposed_subsidy < 1) return 0;

      let subsidy;
      // check if payments will exceed ceiling.
      if (state.current_subsidy_distributed + proposed_subsidy > state.current_cap) {
        // pay the remainder only
        // TODO: This creates a race. Check ordering of list.
        subsidy = state.current_cap - state.current_subsidy_distributed;
      } else {

        // happy case, the ceiling is not met.
        subsidy = proposed_subsidy;
      };

      if (subsidy == 0) return 0;

      let minted_coins = Libra::mint<GAS>(vm, subsidy);
      LibraAccount::vm_deposit_with_metadata<GAS>(
        vm,
        miner,
        minted_coins,
        x"",
        x""
      );

      state.current_subsidy_distributed = state.current_subsidy_distributed + subsidy;

      subsidy
    }

    public fun fullnode_reconfig(vm: &signer) acquires FullnodeSubsidy {
      Roles::assert_libra_root(vm);

      // update values for the proof auction.
      auctioneer(vm);
      let state = borrow_global_mut<FullnodeSubsidy>(Signer::address_of(vm));
       // save 
      state.previous_epoch_proofs = state.current_proofs_verified;
      // reset counters
      state.current_subsidy_distributed = 0u64;
      state.current_proofs_verified = 0u64;
    }

    public fun set_global_count(vm: &signer, count: u64) acquires FullnodeSubsidy{
      let state = borrow_global_mut<FullnodeSubsidy>(Signer::address_of(vm));
      state.current_proofs_verified = count;
    }

    fun baseline_auction_units():u64 {
      let epoch_length_mins = 24 * 60;
      let steady_state_nodes = 1000;
      let target_delay = 10;
      steady_state_nodes * (epoch_length_mins/target_delay)
    }

    fun auctioneer(vm: &signer) acquires FullnodeSubsidy {

      Roles::assert_libra_root(vm);

      let state = borrow_global_mut<FullnodeSubsidy>(Signer::address_of(vm));

      // The targeted amount of proofs to be submitted network-wide per epoch.
      let baseline_auction_units = baseline_auction_units(); 
      // The max subsidy that can be paid out in the next epoch.
      let ceiling = fullnode_subsidy_ceiling(vm);


      // Failure case
      if (ceiling < 1) ceiling = 1;

      state.current_proof_price = calc_auction(
        ceiling,
        baseline_auction_units,
        state.current_proofs_verified
      );
      // Set new ceiling
      state.current_cap = ceiling;
    }

    public fun calc_auction(
      ceiling: u64,
      baseline_auction_units: u64,
      current_proofs_verified: u64,
    ): u64 {
      // Calculate price per proof
      // Find the baseline price of a proof, which will be altered based on performance.
      // let baseline_proof_price = FixedPoint32::divide_u64(
      //   ceiling,
      //   FixedPoint32::create_from_raw_value(baseline_auction_units)
      // );
      let baseline_proof_price = ceiling/baseline_auction_units;
      // print(&baseline_proof_price);

      // print(&FixedPoint32::get_raw_value(copy baseline_proof_price));
      // Calculate the appropriate multiplier.
      let proofs = current_proofs_verified;
      if (proofs < 1) proofs = 1;

      let multiplier = baseline_auction_units/proofs;
      
      // let multiplier = FixedPoint32::create_from_rational(
      //   baseline_auction_units,
      //   proofs
      // );
      // print(&multiplier);

      // Set the proof price using multiplier.
      // New unit price cannot be more than the ceiling
      // let proposed_price = FixedPoint32::multiply_u64(
      //   baseline_proof_price,
      //   multiplier
      // );

      let proposed_price = baseline_proof_price * multiplier;

      // print(&proposed_price);

      if (proposed_price < ceiling) {
        return proposed_price
      };
      //Note: in failure case, the next miner gets the full ceiling
      return ceiling
    }

    fun fullnode_subsidy_ceiling(vm: &signer):u64 {
      //get TX fees from previous epoch.
      let fees = TransactionFee::get_amount_to_distribute(vm);
      // Recover from failure case where there are no fees
      if (fees < baseline_auction_units()) return baseline_auction_units();
      fees
    }
}
}