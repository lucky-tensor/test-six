
<a name="0x1_ValidatorUniverse"></a>

# Module `0x1::ValidatorUniverse`



-  [Struct `ValidatorEpochInfo`](#0x1_ValidatorUniverse_ValidatorEpochInfo)
-  [Resource `ValidatorUniverse`](#0x1_ValidatorUniverse_ValidatorUniverse)
-  [Function `initialize`](#0x1_ValidatorUniverse_initialize)
-  [Function `add_validator`](#0x1_ValidatorUniverse_add_validator)
-  [Function `get_eligible_validators`](#0x1_ValidatorUniverse_get_eligible_validators)
-  [Function `validator_exists_in_universe`](#0x1_ValidatorUniverse_validator_exists_in_universe)
-  [Function `proof_of_weight`](#0x1_ValidatorUniverse_proof_of_weight)
-  [Function `get_validator_index_`](#0x1_ValidatorUniverse_get_validator_index_)
-  [Function `get_validator`](#0x1_ValidatorUniverse_get_validator)
-  [Function `check_if_active_validator`](#0x1_ValidatorUniverse_check_if_active_validator)
-  [Function `get_validator_weight`](#0x1_ValidatorUniverse_get_validator_weight)


<pre><code><b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="FixedPoint32.md#0x1_FixedPoint32">0x1::FixedPoint32</a>;
<b>use</b> <a href="Globals.md#0x1_Globals">0x1::Globals</a>;
<b>use</b> <a href="Option.md#0x1_Option">0x1::Option</a>;
<b>use</b> <a href="Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_ValidatorUniverse_ValidatorEpochInfo"></a>

## Struct `ValidatorEpochInfo`



<pre><code><b>struct</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_address: address</code>
</dt>
<dd>

</dd>
<dt>
<code>weight: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_ValidatorUniverse_ValidatorUniverse"></a>

## Resource `ValidatorUniverse`



<pre><code><b>resource</b> <b>struct</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validators: vector&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorUniverse::ValidatorEpochInfo</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_ValidatorUniverse_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_initialize">initialize</a>(account: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_initialize">initialize</a>(account: &signer){
  // Check for transactions sender is association
  <b>let</b> sender = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account);
  <b>assert</b>(sender == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 220101014010);

  move_to&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>&gt;(account, <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a> {
      validators: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;()
  });
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_add_validator"></a>

## Function `add_validator`



<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_add_validator">add_validator</a>(addr: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_add_validator">add_validator</a>(addr: address) <b>acquires</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a> {
  <b>let</b> collection = borrow_global_mut&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());
  <b>if</b>(!<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_validator_exists_in_universe">validator_exists_in_universe</a>(collection, addr))
  <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;(
    &<b>mut</b> collection.validators,
    <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>{
    validator_address: addr,
    weight: 1
  });
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_get_eligible_validators"></a>

## Function `get_eligible_validators`



<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_eligible_validators">get_eligible_validators</a>(account: &signer): vector&lt;address&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_eligible_validators">get_eligible_validators</a>(account: &signer) : vector&lt;address&gt; <b>acquires</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a> {
  <b>let</b> sender = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account);
  <b>assert</b>(sender == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 220101014010);

  <b>let</b> eligible_validators = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;();
  // Create a vector <b>with</b> all eligible validator addresses
  // Get all the data from the <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a> <b>resource</b> stored in the association/system address.
  <b>let</b> collection = borrow_global&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());

  <b>let</b> i = 0;
  <b>let</b> validator_list = &collection.validators;
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;(validator_list);
  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&len);
  <b>while</b> (i &lt; len) {
      <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> eligible_validators, <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;(validator_list, i).validator_address);
      i = i + 1;
  };
  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&len);
  eligible_validators
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_validator_exists_in_universe"></a>

## Function `validator_exists_in_universe`



<pre><code><b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_validator_exists_in_universe">validator_exists_in_universe</a>(validatorUniverse: &<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorUniverse">ValidatorUniverse::ValidatorUniverse</a>, addr: address): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_validator_exists_in_universe">validator_exists_in_universe</a>(validatorUniverse: &<a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>, addr: address): bool {
  <b>let</b> i = 0;
  <b>let</b> validator_list = &validatorUniverse.validators;
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;(validator_list);
  <b>while</b> (i &lt; len) {
      <b>if</b> (<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;(validator_list, i).validator_address == addr) <b>return</b> <b>true</b>;
      i = i + 1;
  };
  <b>false</b>
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_proof_of_weight"></a>

## Function `proof_of_weight`



<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_proof_of_weight">proof_of_weight</a>(account: &signer, addr: address, is_validator_in_current_epoch: bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_proof_of_weight">proof_of_weight</a>(account: &signer, addr: address, is_validator_in_current_epoch: bool): u64 <b>acquires</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a> {
  <b>let</b> sender = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account);
  <b>assert</b>(sender == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 22010105014010);

  //1. borrow the Validator's <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>
  // Get the validator
  <b>let</b> collection =  borrow_global_mut&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());

  // Getting index of the validator
  <b>let</b> index_vec = <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator_index_">get_validator_index_</a>(&collection.validators, addr);
  <b>assert</b>(<a href="Option.md#0x1_Option_is_some">Option::is_some</a>(&index_vec), 220105022040);
  <b>let</b> index = *<a href="Option.md#0x1_Option_borrow">Option::borrow</a>(&index_vec);

  <b>let</b> validator_list = &<b>mut</b> collection.validators;
  <b>let</b> validatorInfo = <a href="Vector.md#0x1_Vector_borrow_mut">Vector::borrow_mut</a>&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;(validator_list, index);


  // Weight is metric based on: The number of epochs the miners have been mining for
  <b>let</b> weight = 1;

  // If the validator mined in current epoch, increment it's weight.
  <b>if</b>(is_validator_in_current_epoch)
    weight = validatorInfo.weight + 1;

  validatorInfo.weight = weight;
  weight
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_get_validator_index_"></a>

## Function `get_validator_index_`



<pre><code><b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator_index_">get_validator_index_</a>(validators: &vector&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorUniverse::ValidatorEpochInfo</a>&gt;, addr: address): <a href="Option.md#0x1_Option_Option">Option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator_index_">get_validator_index_</a>(validators: &vector&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>&gt;, addr: address): <a href="Option.md#0x1_Option">Option</a>&lt;u64&gt;{
  <b>let</b> size = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(validators);

  <b>let</b> i = 0;
  <b>while</b> (i &lt; size) {
      <b>let</b> validator_info_ref = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(validators, i);
      <b>if</b> (validator_info_ref.validator_address == addr) {
          <b>return</b> <a href="Option.md#0x1_Option_some">Option::some</a>(i)
      };
      i = i + 1;
  };

  <b>return</b> <a href="Option.md#0x1_Option_none">Option::none</a>()
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_get_validator"></a>

## Function `get_validator`



<pre><code><b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator">get_validator</a>(addr: address): <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorUniverse::ValidatorEpochInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator">get_validator</a>(addr: address): <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a> <b>acquires</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>{

  <b>let</b> validators = &borrow_global_mut&lt;<a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).validators;
  <b>let</b> size = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(validators);

  <b>let</b> i = 0;
  <b>while</b> (i &lt; size) {
      <b>let</b> validator_info_ref = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(validators, i);
      <b>if</b> (validator_info_ref.validator_address == addr) {
          <b>return</b> *validator_info_ref
      };
      i = i + 1;
  };

  <b>return</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_ValidatorEpochInfo">ValidatorEpochInfo</a>{
    validator_address: {{<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()}},
    weight: 0
  }
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_check_if_active_validator"></a>

## Function `check_if_active_validator`



<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_check_if_active_validator">check_if_active_validator</a>(_addr: address, epoch_length: u64, current_block_height: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_check_if_active_validator">check_if_active_validator</a>(_addr: address, epoch_length: u64, current_block_height: u64): bool {
  // Calculate the window in which we are evaluating the performance of validators.
  // start and effective end block height for the current epoch
  // End block for analysis happens a few blocks before the block boundar since not all blocks will be committed <b>to</b> all nodes at the end of the boundary.
  <b>let</b> start_block_height = 1;
  <b>if</b> (current_block_height &gt; <a href="Globals.md#0x1_Globals_get_epoch_length">Globals::get_epoch_length</a>()) {
    start_block_height = current_block_height - epoch_length;
  };

  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&0x2201070151200001);


  <b>let</b> adjusted_end_block_height = current_block_height - <a href="Globals.md#0x1_Globals_get_epoch_boundary_buffer">Globals::get_epoch_boundary_buffer</a>();

  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&0x2201070151200002);


  <b>let</b> blocks_in_window = adjusted_end_block_height - start_block_height;

  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&0x2201070151200003);

  // The current block_height needs <b>to</b> be at least the length of one (the first) epoch.
  // <b>assert</b>(current_block_height &gt;= blocks_in_window, 220107015120);

  // Calculating liveness threshold which is signing 66% of the blocks in epoch.
  // Note that nodes in hotstuff stops voting after 2/3 consensus has been reached, and skip <b>to</b> next block.

  <b>let</b> threshold_signing = <a href="FixedPoint32.md#0x1_FixedPoint32_divide_u64">FixedPoint32::divide_u64</a>(66, <a href="FixedPoint32.md#0x1_FixedPoint32_create_from_rational">FixedPoint32::create_from_rational</a>(100, 1)) * blocks_in_window;
  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&0x2201070151200004);

  ////////  TODO: REMOVED IN MERGE PROCESS ///////
  <b>let</b> block_signed_by_validator = 0; // Stats::node_heuristics(addr, start_block_height, adjusted_end_block_height);
  // <a href="Debug.md#0x1_Debug_print">Debug::print</a>(&0x2201070151200005);

  <b>if</b> (block_signed_by_validator &lt; threshold_signing) {
      <b>return</b> <b>false</b>
  };

  <b>true</b>
}
</code></pre>



</details>

<a name="0x1_ValidatorUniverse_get_validator_weight"></a>

## Function `get_validator_weight`



<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator_weight">get_validator_weight</a>(account: &signer, addr: address): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator_weight">get_validator_weight</a>(account: &signer, addr: address): u64 <b>acquires</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">ValidatorUniverse</a>{
  <b>let</b> sender = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account);
  <b>assert</b>(sender == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 220106014010);

  <b>let</b> validatorInfo = <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_validator">get_validator</a>(addr);

  // Validator not in universe error
  <b>assert</b>(validatorInfo.validator_address != <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 220106022040);
  <b>return</b> validatorInfo.weight
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
