address 0x1 {

module GAS {
    use 0x1::AccountLimits;
    use 0x1::Libra;
    use 0x1::LibraTimestamp;
    use 0x1::FixedPoint32;
  
    struct GAS { }

    public fun initialize(
        lr_account: &signer,
        tc_account: &signer,
    ) {
        LibraTimestamp::assert_genesis();
        Libra::register_SCS_currency<GAS>(
            lr_account,
            tc_account,
            FixedPoint32::create_from_rational(1, 1), // exchange rate to GAS
            1000000, // scaling_factor = 10^6
            1000,     // fractional_part = 10^2
            b"GAS"
        );
        AccountLimits::publish_unrestricted_limits<GAS>(lr_account);
    }
}
}