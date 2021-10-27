// Todo: These GAS values have no effect, all accounts start with 1M GAS
//! account: alice, 2000000GAS, 0 
//! account: bob,   1000000GAS, 0, validator
//! account: carol, 1000000GAS, 0 

// test runs various autopay instruction types to ensure they are being executed as expected

//! new-transaction
//! sender: carol
script {
  use 0x1::Wallet;
  use 0x1::Vector;

  fun main(sender: signer) {
    Wallet::set_comm(&sender);
    let list = Wallet::get_comm_list();
    assert(Vector::length(&list) == 1, 7357001);
  }
}

// check: EXECUTED


// alice commits to paying carol 200 GAS per epoch
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;
    AutoPay::enable_autopay(sender);
    assert(AutoPay::is_enabled(Signer::address_of(sender)), 0);
    
    AutoPay::create_instruction(sender, 1, 2, @{{carol}}, 2, 200);

    let (type, payee, end_epoch, percentage) = AutoPay::query_instruction(
      Signer::address_of(sender), 1
    );
    assert(type == 2, 1);
    assert(payee == @{{carol}}, 1);
    assert(end_epoch == 2, 1);
    assert(percentage == 200, 1);
  }
}
// check: EXECUTED


///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 31000000
//! round: 23
///////////////////////////////////////////////////


// Weird. This next block needs to be added here otherwise the prologue above does not run.
///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 32000000
//! round: 24
///////////////////////////////////////////////////

//! new-transaction
//! sender: diemroot
script {
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: signer) {
    let ending_balance = DiemAccount::balance<GAS>(@{{alice}});
    assert(ending_balance == 1999800, 7357002);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 61000000
//! round: 65
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 92000000
//! round: 66
///////////////////////////////////////////////////
///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 93000000
//! round: 67
///////////////////////////////////////////////////

//! new-transaction
//! sender: diemroot
script {
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: signer) {
    // alice will have paid 5% on the 10000 she received last epoch
    let ending_balance = DiemAccount::balance<GAS>(@{{alice}});
    assert(ending_balance == 1999600, 7357003);

    // check balance of recipients
    let ending_balance = DiemAccount::balance<GAS>(@{{carol}});
    assert(ending_balance == 1000400, 7357004);
  }
}
// check: EXECUTED
