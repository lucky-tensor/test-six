// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0
use vdf::{VDFParams, VDF};
use libra_types::{
    transaction::authenticator::AuthenticationKey, 
    vm_status::{StatusCode}
};
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::{Reference, Value},
};
use std::collections::VecDeque;
use std::convert::TryFrom;
use vm::errors::{PartialVMError, PartialVMResult};
// use hex;

/// Rust implementation of Move's `native public fun verify(challenge: vector<u8>, difficulty: u64, alleged_solution: vector<u8>): bool`
pub fn verify(
    context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    if arguments.len() != 3 {
        let msg = format!(
            "wrong number of arguments for sha3_256 expected 3 found {}",
            arguments.len()
        );
        return Err(PartialVMError::new(StatusCode::UNREACHABLE).with_message(msg));
    }

    let alleged_solution = pop_arg!(arguments, Reference)
        .read_ref()?
        .value_as::<Vec<u8>>()?;
    let difficulty = pop_arg!(arguments, Reference)
        .read_ref()?
        .value_as::<u64>()?;
    let challenge = pop_arg!(arguments, Reference)
        .read_ref()?
        .value_as::<Vec<u8>>()?;

    // TODO change the `cost_index` when we have our own cost table.
    let cost = native_gas(context.cost_table(), NativeCostIndex::VDF_VERIFY, 1);

    let v = vdf::WesolowskiVDFParams(4096).new();
    
    // println!("vdf.rs - challenge: {}", hex::encode(&challenge));
    // println!("vdf.rs - difficulty: {:?}", &difficulty);


    let result = v.verify(&challenge, difficulty, &alleged_solution);

    // println!("vdf.rs - result: {:?}", result);

    let return_values = vec![Value::bool(result.is_ok())];
    Ok(NativeResult::ok(cost, return_values))
}

// Extracts the first 32 bits of the vdf challenge which is the auth_key
// Auth Keys can be turned into an AccountAddress type, to be serialized to a move address type.
pub fn extract_address_from_challenge(
    context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let cost = native_gas(context.cost_table(), NativeCostIndex::VDF_PARSE, 1);

    let challenge_vec = pop_arg!(arguments, Reference)
        .read_ref()?
        .value_as::<Vec<u8>>()?;

    let auth_key_vec = &challenge_vec[..32];

    // TODO: Error handle on wrong size.
    // if len < 32 {
    //     return Err(NativeResult::err(
    //         cost,
    //         VMStatus::new(StatusCode::NATIVE_FUNCTION_ERROR)
    //             .with_sub_status(DEFAULT_ERROR_CODE),
    //     ));
    // };

    let auth_key = AuthenticationKey::try_from(auth_key_vec).expect("Check length");
    let address = auth_key.derived_address();
    let return_values = vec![Value::address(address), Value::vector_u8(auth_key_vec[..16].to_owned())];
    Ok(NativeResult::ok(cost, return_values))
}
