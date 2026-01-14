#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{prelude::*};
use alloc::string::String;
use alloc::vec::Vec;
use sha2::{Sha256, Digest};

// #[storage] defines the persistent storage layout of the contract.
// Even if unused, it's required for the entrypoint struct.
#[storage]
#[entrypoint] // #[entrypoint] marks this struct as the main entry point to the contract.
pub struct BtcVerifier;

#[public] // #[public] makes methods in this impl block callable from other contracts/EOAs.
impl BtcVerifier {
    /// verifying a Bitcoin block header often requires double-SHA256 (Hash256).
    /// This function takes a hex string, decodes it, hashes it twice, and returns the result.
    pub fn hash_btc_header(&self, header_hex: String) -> Result<String, Vec<u8>> {
        // 1. Decode the input hex string into bytes.
        // In a real scenario, you might accept bytes directly to save gas.
        let bytes = hex::decode(header_hex).map_err(|_| Vec::new())?;
        
        // 2. Perform the first SHA-256 hash.
        let mut hasher1 = Sha256::new();
        hasher1.update(&bytes);
        let hash1 = hasher1.finalize();

        // 3. Perform the second SHA-256 hash on the result of the first.
        // Bitcoin uses this "Hash256" (SHA256d) for block headers and txids.
        let mut hasher2 = Sha256::new();
        hasher2.update(hash1);
        let hash2 = hasher2.finalize();

        // 4. Return the double-hashed result as a hex string.
        Ok(hex::encode(hash2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_sha256() {
        // "hello" in ASCII is 68656c6c6f in hex.
        // To verify: `echo -n "hello" | openssl dgst -sha256 -binary | openssl dgst -sha256`
        // Result: 9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50
        let input = "68656c6c6f";             
        
        let verifier = BtcVerifier {};
        let result = verifier.hash_btc_header(input.into()).unwrap();

        assert_eq!(result, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
    }
}
