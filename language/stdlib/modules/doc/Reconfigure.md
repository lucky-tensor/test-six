
<a name="0x1_Reconfigure"></a>

# Module `0x1::Reconfigure`



-  [Function `reconfigure`](#0x1_Reconfigure_reconfigure)


<pre><code><b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="Globals.md#0x1_Globals">0x1::Globals</a>;
<b>use</b> <a href="LibraSystem.md#0x1_LibraSystem">0x1::LibraSystem</a>;
<b>use</b> <a href="MinerState.md#0x1_MinerState">0x1::MinerState</a>;
<b>use</b> <a href="NodeWeight.md#0x1_NodeWeight">0x1::NodeWeight</a>;
<b>use</b> <a href="Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="Stats.md#0x1_Stats">0x1::Stats</a>;
<b>use</b> <a href="Subsidy.md#0x1_Subsidy">0x1::Subsidy</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_Reconfigure_reconfigure"></a>

## Function `reconfigure`



<pre><code><b>public</b> <b>fun</b> <a href="Reconfigure.md#0x1_Reconfigure_reconfigure">reconfigure</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Reconfigure.md#0x1_Reconfigure_reconfigure">reconfigure</a>(vm: &signer) {
    <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 180101014010);

    // Process outgoing validators:
    // Distribute Transaction fees and subsidy payments <b>to</b> all outgoing validators

    <b>let</b> subsidy_units = <a href="Subsidy.md#0x1_Subsidy_calculate_Subsidy">Subsidy::calculate_Subsidy</a>(vm);
    <a href="Subsidy.md#0x1_Subsidy_process_subsidy">Subsidy::process_subsidy</a>(vm, subsidy_units);

    <a href="Subsidy.md#0x1_Subsidy_process_fees">Subsidy::process_fees</a>(vm);

    // Propose upcoming validator set:
    // Step 1: Sort Top N Elegible validators
    // Step 2: Jail non-performing validators
    // Step 3: Reset counters
    // Step 4: Bulk <b>update</b> validator set (reconfig)

    // prepare_upcoming_validator_set(vm);
    <b>let</b> top_accounts = <a href="NodeWeight.md#0x1_NodeWeight_top_n_accounts">NodeWeight::top_n_accounts</a>(
        vm, <a href="Globals.md#0x1_Globals_get_max_validator_per_epoch">Globals::get_max_validator_per_epoch</a>());
    <b>let</b> jailed_set = <a href="LibraSystem.md#0x1_LibraSystem_get_jailed_set">LibraSystem::get_jailed_set</a>(vm);

    <b>let</b> proposed_set = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>();
    <b>let</b> i = 0;
    <b>while</b> (i &lt; <a href="Vector.md#0x1_Vector_length">Vector::length</a>(&top_accounts)) {
        <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&top_accounts, i);
        <b>if</b> (!<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>(&jailed_set, &addr)){
            <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> proposed_set, addr);
        };
        i = i+ 1;
    };

    // If the cardinality of validator_set in the next epoch is less than 4, we keep the same validator set.
    <b>if</b>(<a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&proposed_set)&lt;= 4) proposed_set = <a href="LibraSystem.md#0x1_LibraSystem_get_val_set_addr">LibraSystem::get_val_set_addr</a>();
    // Usually an issue in staging network for QA only.
    // This is very rare and theoretically impossible for network <b>with</b> at least 6 nodes and 6 rounds. If we reach an epoch boundary <b>with</b> at least 6 rounds, we would have at least 2/3rd of the validator set <b>with</b> at least 66% liveliness.

    //Reset Counters
    <a href="Stats.md#0x1_Stats_reconfig">Stats::reconfig</a>(vm, &proposed_set);
    <a href="MinerState.md#0x1_MinerState_reconfig">MinerState::reconfig</a>(vm);

    // <a href="Reconfigure.md#0x1_Reconfigure">Reconfigure</a> the network
    <a href="LibraSystem.md#0x1_LibraSystem_bulk_update_validators">LibraSystem::bulk_update_validators</a>(vm, proposed_set);
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
