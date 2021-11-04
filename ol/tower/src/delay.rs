//! MinerApp delay module
#![forbid(unsafe_code)]
use anyhow::{Error, bail};
/// Functions for running the VDF.
use vdf::{PietrzakVDFParams, VDF, VDFParams};

/// Runs the VDF
pub fn do_delay(preimage: &[u8], difficulty: u64, security: u16) -> Result<Vec<u8>, Error> {
    // Functions for running the VDF.
    let vdf: vdf::PietrzakVDF = PietrzakVDFParams(security).new();
    match vdf.solve(preimage, difficulty) {
        Ok(proof) => Ok(proof),
        Err(e) => bail!(format!("ERROR: cannot solve VDF, message {:?}", e)),
    }
}

/// Verifies a proof
pub fn verify(preimage: &[u8], proof: &[u8], difficulty: u64, security: u16) -> bool{
    let vdf: vdf::PietrzakVDF = PietrzakVDFParams(security).new();
    
    match vdf.verify(preimage, difficulty, proof) {
       Ok(_) => true,
       Err(e) => {
        println!("Proof is not valid. {:?}", e);
        false
       }
    }
}
