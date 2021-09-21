//! account: alices_app

//! new-transaction
//! sender: alices_app
script {
  use 0x1::Bonding;
  use 0x1::Signer;
  // use 0x1::Debug::print;

  fun main(sender: signer) {
    let coin = 10;
    let supply = 100;
    Bonding::initialize_curve(&sender, coin, supply);

    let addr = Signer::address_of(&sender);
    let (reserve, supply) = Bonding::get_curve_state(addr);
    assert(reserve == 10, 735701);
    assert(supply == 100, 735701);
  }
}