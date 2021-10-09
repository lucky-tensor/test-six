//! account: alice, 1000000GAS
//! account: bob, 10000GAS

// We test processing of autopay at differnt epochs and balance transfers
// Finally, we also check the end_epoch functionality of autopay

// creating the instruction
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;    
    AutoPay::enable_autopay(sender);
    assert(AutoPay::is_enabled(Signer::address_of(sender)), 7357001);
    AutoPay::create_instruction(sender, 1, 0, @{{bob}}, 2, 500);
    let (type, payee, end_epoch, percentage) = AutoPay::query_instruction(
      Signer::address_of(sender), 1
    );
    assert(type == 0u8, 7357002);
    assert(payee == @{{bob}}, 7357003);
    assert(end_epoch == 2, 7357004);
    assert(percentage == 500, 7357005);
  }
}
// check: EXECUTED

// //! new-transaction
// //! sender: diemroot
// script {
//     use 0x1::Wallet;

//     fun main(vm: signer) {
//       Wallet::init_comm_list(&vm);
//     }
// }

// // check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::Wallet;
    use 0x1::Vector;

    fun main(sender: signer) {
      Wallet::set_comm(&sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 1, 7357006);
    }
}

// check: EXECUTED

// Processing AutoPay to see if payments are done
//! new-transaction
//! sender: diemroot
script {
  use 0x1::AutoPay;
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  fun main(sender: signer) {
    let alice_balance = DiemAccount::balance<GAS>(@{{alice}});
    let bob_balance = DiemAccount::balance<GAS>(@{{bob}});
    assert(alice_balance == 1000000, 7357007);
    AutoPay::process_autopay(&sender);
    
    let alice_balance_after = DiemAccount::balance<GAS>(@{{alice}});
    assert(alice_balance_after < alice_balance, 7357008);
    
    let transferred = alice_balance - alice_balance_after;    
    let bob_received = DiemAccount::balance<GAS>(@{{bob}}) - bob_balance;    
    assert(transferred==bob_received, 7357009)
  }
}
// check: EXECUTED