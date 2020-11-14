/////////////////////////////////////////////////////////////////////////
// 0L Module
// TestFixtures
// Collection of vdf proofs for testing.
/////////////////////////////////////////////////////////////////////////

address 0x1 {
module TestFixtures{
  use 0x1::Testnet;

    // Here, I experiment with persistence for now
    // Committing some code that worked successfully
    // struct ProofFixture {
    //   challenge: vector<u8>,
    //   solution: vector<u8>
    // }

    // public fun alice(){
    //   // In the actual module, must assert that this is the sender is the association
    //   move_to_sender<State>(State{ hist: Vector::empty() });
    // }

    public fun easy_chal(): vector<u8> {
      assert(Testnet::is_testnet(), 130102014010);
      x"aa"
    }

    public fun easy_sol(): vector<u8>  {
      assert(Testnet::is_testnet(), 130102014010);
      x"001eef1120c0b13b46adae770d866308a5db6fdc1f408c6b8b6a7376e9146dc94586bdf1f84d276d5f65d1b1a7cec888706b680b5e19b248871915bb4319bbe13e7a2e222d28ef9e5e95d3709b46d88424c52140e1d48c1f123f2a1341448b9239e40509a604b1c54cc6c2750ae1255287308d7b2dd5353bae649d4b1bcb65154cffe2e189ec6960d5fa88eef4aa4f1c1939ce8b4808c379562a45ffcda8c502b9558c0999a595dddc02601e837634081977be9195345fae0e858b2cf402e03844ccda24977966ca41706e84c3bf4a841c3845c7bb519547b735cb5644fb0f8a78384827a098b3c80432a4db1135e3df70ade040444d67936b949bd17b68f64fde81000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
    }

    //FROM: libra/fixtures/block_0.json.stage.alice
    public fun alice_0_easy_chal(): vector<u8> {
      assert(Testnet::is_testnet(), 130102014010);
      x"87515d94a244235a1433d7117bc0cb154c613c2f4b1e67ca8d98a542ee3f59f5000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000304c20746573746e65746400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865206e6174696f6e"
    }

    public fun alice_0_easy_sol(): vector<u8>  {
      assert(Testnet::is_testnet(), 130102014010);
      x"002c4dc1276a8a58ea88fc9974c847f14866420cbc62e5712baf1ae26b6c38a393c4acba3f72d8653e4b2566c84369601bdd1de5249233f60391913b59f0b7f797f66897de17fb44a6024570d2f60e6c5c08e3156d559fbd901fad0f1343e0109a9083e661e5d7f8c1cc62e815afeee31d04af8b8f31c39a5f4636af2b468bf59a0010f48d79e7475be62e7007d71b7355944f8164e761cd9aca671a4066114e1382fbe98834fe32cf494d01f31d1b98e3ef6bffa543928810535a063c7bbf491c472263a44d9269b1cbcb0aa351f8bd894e278b5d5667cc3f26a35b9f8fd985e4424bedbb3b77bdcc678ccbb9ed92c1730dcdd3a89c1a8766cbefa75d6eeb7e5921000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
    }

        //FROM: libra/fixtures/block_1.json.stage.alice

    public fun alice_1_easy_chal(): vector<u8> {
      assert(Testnet::is_testnet(), 130102014010);
      x"3190cef88aa2fb86fbfa062f62be33d08d1493e982597d7be286ab5b6d01e4b0"
    }

    public fun alice_1_easy_sol(): vector<u8>  {
      assert(Testnet::is_testnet(), 130102014010);
      x"006e33a9542693512b59aa04081bb2a87f0bf07328c62cfc5dafdebf57c35ddd6a75664ddfa7ebfe0b9cbc6c5d19f03f77841cef9923d32bea8a4a642adfd94a31d2b523cb32e8adc27ee63ec2d793f3c224c0be2c4258dcb7ba5b74ee78d21f1d045165c9bd7e41a42085ea4cdb95fb8ffd437448ad93610d4d445f339807fffbffb3a77ab38d67e301889a7d83a789895fa5a12113213b4674ec4dbd6037bcd7c9e8c5edb6f7bf738e19845aa25c0cd3cf258f978c406195c2a8d7edf8785d1697653d213add8cb632680f167dbb1a6a4716a2b174a91c5319c9b5224504975e94e7b751b55bad30b27678fa9c46d94d02f5bf757d27305b1283c542ca02927427000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
    }

  // // TODO: Replace with fixtures in libra/fixtures.
    public fun alice_0_hard_chal(): vector<u8> {
      assert(Testnet::is_testnet(), 130102014010);
      x"91ffe0bce9806e599cd3565958ed0d3a0e7da4499fb75ccd30e6527761a55a06000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000746573746e6574009f240000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865206e6174696f6e"
    }

    public fun alice_0_hard_sol(): vector<u8>  {
      assert(Testnet::is_testnet(), 130102014010);
      x"0057177684bb7bdc903dbda7747b69ae644357463f32e8f5992d68941db3024c15e4c6689ca1ec6a4661a0f88e8ab8a20b454f485b56a17cc4ca518cec5119a1b2e82ef66c36e692c7421ca68e6a98d53a64f6ff5e7fcaaa2c3b77c04d2a70386e310e7d4d791192e744fdbe3237a278c446747db2ed342fa3eb2e84c060c3e40affdaed7d471aa8e7c69f07fab73675802577741408c1daf5953c111cb90b1ea5262f40acd7db54feca7102b2e32576a997f0018df0df1ed508f4968ed87f2f174378cadc32c03f037aede53c4aa5cf22254b0114da49b63ba67b2b23aa599567f4e69f8f41ff30cd419e4b2f72bda15dc89eeb68661f704bbbdb8201292b0a493100007f2fa518ca6fc7fcadc3f1a0d42297f7a6d6e3c304bca3fe90cafc75b79bac5a053552b287fa47d4109561630bbe6b6c33d197f78d539845ea058db37757734462338dafe08ca8bc778a2a309968ef89c88cd12e4958701a94e36552da5abda4851fef7a75fe85432289a5350b4b43b6717240737f110c6dff291e5ba3eeb60000093c720dafce1325af11b8e531bd5db97eec1c633bd278e0b05ad783cc6d8042808cd08c6cbbf19cdbc61376b09dc57d60a947c58b5f9edd4d8fc3a25835946d97ecce061ca5c979e716709ac19a562c21429e8e757b9eb9a057cc2ebc31f19ed12b0db83f620fe1a8590cbdc029815fee988bba438b58452c30bb04f08faf"
    }
    public fun alice_1_hard_chal(): vector<u8> {
      assert(Testnet::is_testnet(), 130102014010);
      x"7ccfbe11759c6a348a09ebac903c312628cf89a971e73f1e0563930ab9271c69"
    }

    public fun alice_1_hard_sol(): vector<u8>  {
      assert(Testnet::is_testnet(), 130102014010);
      x"006408be2b99428c65d7a431a5e7a9e1657de1e8aae012274c43a744e3038a54a64cce4be427b64518b105f469d6c76eb7be7b2ee64acf5a786f2f2e7b7a3191f1c9ee0409a5780dfe9d979b48497abeab80cf985019363f83357ea64e57de3eb7c61411ea08467306ba7551317c871c8677e3af96d30fb1c33ffd5ce764e3dae4004c6930ef09561130b563b61cac4eb148a06c6d114a7531390edab64dbefeff99fd759ed32b0730a9a2a94fcb0032fc7740bab401a9af78f520150785d5093d38a020d330e875876d60e3aeaad7d10026a4a5fcc66553530b2ae6026c3ed8f3ff727cee3c0d2e96303594aa7a22df71cb8ac361ae687cc77ebae18e1b315a6e5d0001186b2b957c219389f3c4f7f3175332a0b3433ebff2f42a22d2a27b7615721d29ccded6a1a48171cb75b389cbbd5c1185d516e55578a3d0c343e643110eae5bfb244b783bc2ef394352f9c0d340df1397e594a553de0b4ae155eb1a0121ff9928f2318fa622bba08b9f9f21cc4294d58a70b5dd53b834b83fcc2f77d2729f03000001f0c19f6ee6000f09b39da694c6e1dd55bc365e298ec15fd863288565c8b0f06634d728b3e74443b5f365a3c14280795d41c921acbd67f25ee993d8fe2359545ef40191a193c2ff37df709a312573c904722753757cbbfc7b6f3c0cbfc7a21bcf72cec4bfccf640d4d93d1e4ceb54561e95b2cfaa4d964b32c7050ae097cb"
    }

    public fun eve_0_easy_chal(): vector<u8> {
      assert(Testnet::is_testnet(), 130102014010);
      x"2bffcbd0e9016013cb8ca78459f69d2b3dc18d1cf61faac6ac70e3a63f062e4b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000304c20746573746e65746400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865206e6174696f6e"
    }

    public fun eve_0_easy_sol(): vector<u8>  {
      assert(Testnet::is_testnet(), 130102014010);
      x"00168e77068c3e4ebf4908cdad141265a65f390f0e82ac2510f9f92116d32a60f049f0d6e098fc3bf3cd363c34cbed43cf1ea9927db2f02934be9a1a7aba3a2c83f13e19336264b4688b7c329edc45ef510ec8b2c99a1ba2949a0577fbb8815da2e5c0ecc6852a9c42a10e001324547fda3858fae568b6405ee59bd2da7443295c0006c8d4ca51804171d1809f3c04546053b33e1f3b08624f33a68f76711bc27db33d1619f05308de1ac4cb349b8156fc073e6ce4730841363a350c5f2e4ac7a4a931916d5c508bcac40e2bfcc7b0ce475b0a5c492b2e752ecf2284b8bacff76b4ad2004ac8b8423bd11a016faa90ef1817c215a3426c9f80100f511177d4f4e2bd000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
    }
  }
}
