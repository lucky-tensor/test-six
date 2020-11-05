// Copyright (c) 0lsf
// SPDX-License-Identifier: Apache-2.0

use language_e2e_tests::{
    account::{Account},
    executor::FakeExecutor
};
use libra_types::{transaction::TransactionStatus, vm_status::KeptVMStatus};
use transaction_builder::encode_minerstate_commit_script;


#[test]
fn demo() {
  let mut executor = FakeExecutor::from_genesis_file();
  // use system account
  let sender = Account::new_libra_root();
  let hello_world= 100u64;
  let seq_num = 1;
  let script = transaction_builder::encode_demo_e2e_script(hello_world);
  let txn = sender
      .transaction()
      .script(script)
      .sequence_number(seq_num)
      .sign();
  
        // execute transaction
    let output = executor.execute_transaction(txn);
    assert_eq!(
        output.status(),
        &TransactionStatus::Keep(KeptVMStatus::Executed)
    );
    executor.apply_write_set(output.write_set());

}
// #[test]
// fn miner_commit () {
//   encode_minerstate_commit_script(
//       challenge,
//       solution,
//     );
//     sender
//         .transaction()
//         .script(script)
//         .sequence_number(seq_num)
//         .sign()
// }