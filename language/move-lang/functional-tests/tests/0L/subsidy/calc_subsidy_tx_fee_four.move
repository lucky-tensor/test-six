//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator
//! account: eve, 1000000, 0, validator
//! account: frank, 1000000, 0, validator
//! account: gene, 1000000, 0, validator


//! block-prologue
//! proposer: alice
//! block-time: 1

//! new-transaction
//! sender: diemroot
script {
  
  use 0x1::Subsidy;
  use 0x1::Vector;
  use 0x1::Stats;
  use 0x1::TransactionFee;
  use 0x1::GAS::GAS;
  use 0x1::Diem;
  use 0x1::Globals;
  
  fun main(vm: signer) {
    // check the case of a network density of 4 active validators.

    let vm = &vm;
    let validators = Vector::singleton<address>(@{{alice}});
    Vector::push_back(&mut validators, @{{bob}});
    Vector::push_back(&mut validators, @{{carol}});
    Vector::push_back(&mut validators, @{{dave}});

    // create mock validator stats for full epoch
    let i = 0;
    while (i < 16) {
      Stats::process_set_votes(vm, &validators);
      i = i + 1;
    };
    let mock_tx_fees = 100000000;
    TransactionFee::pay_fee(Diem::mint<GAS>(vm, mock_tx_fees));

    let guaranteed_minimum = Subsidy::subsidy_curve(
      Globals::get_subsidy_ceiling_gas(),
      4,
      Globals::get_max_validators_per_set(),
    );

    let expected_subsidy = guaranteed_minimum - mock_tx_fees;

    // deducts gas from txs from subsidy.
    let (subsidy, _) = Subsidy::calculate_subsidy(vm, 4);
    assert(subsidy == expected_subsidy, 7357190101021000);

    }
}
// check: EXECUTED
