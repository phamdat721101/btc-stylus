# A Beginner's Guide to Building on Arbitrum Stylus: Bitcoin Hash Verification

Arbitrum Stylus is a powerful new way to write smart contracts on Arbitrum. Unlike traditional EVM development which restricts you to Solidity or Vyper, Stylus allows you to write contracts in **Rust**, **C** and **C++** that compile to WebAssembly (WASM).

This guide will walk you through the basics of Stylus by analyzing a simple yet practical example: **Verifying Bitcoin Block Headers**.

## Why Stylus?

1.  **Performance**: WASM contracts are much more efficient than EVM bytecode, allowing for compute-intensive tasks (like cryptographic verification) that would be too expensive in Solidity.
2.  **Safety**: By using Rust, you get memory safety, strong typing, and a rich ecosystem of libraries (crates) out of the box.
3.  **Interoperability**: Stylus contracts are fully composable with existing Solidity contracts.

## Project Structure

A typical Stylus project looks like a standard Rust project:

```text
.
├── Cargo.toml      # Rust dependencies
├── Stylus.toml     # Stylus-specific configuration
└── src
    └── lib.rs      # Your smart contract logic
```

### 1. Configuration (`Cargo.toml`)

Your `Cargo.toml` file defines the dependencies your contract needs. Crucially, it includes the `stylus-sdk`:

```toml
[dependencies]
stylus-sdk = "0.10.0"
hex = "0.4.3"
sha2 = "0.10.8"
mini-alloc = "1.0.0"  # Minimal memory allocator for WASM
```

- `stylus-sdk`: Provides the macros and types needed to interact with the Arbitrum chain.
- `hex` & `sha2`: Standard Rust crates for handling hex strings and SHA-256 hashing.

### 2. The Contract Logic (`src/lib.rs`)

Let's break down the code for our `BtcVerifier` contract.

#### Imports and Setup

```rust
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{prelude::*};
use alloc::string::String;
use alloc::vec::Vec;
use sha2::{Sha256, Digest};
```

- `no_main`: Tells Rust not to look for a standard `main` function, as the Stylus runtime handles entry.
- `extern crate alloc`: We need dynamic memory allocation (Vectors, Strings) in our contract.

#### Storage Definition

In Stylus, we define the contract's storage layout using a struct.

```rust
#[storage]
#[entrypoint]
pub struct BtcVerifier;
```

- `#[storage]`: Defines persistent storage. Even if we don't store any data (like in this stateless example), we need this struct.
- `#[entrypoint]`: This macro marks `BtcVerifier` as the entry point. When someone calls this contract, Stylus looks here first.

#### Public Methods

This is where the magic happens. We expose functions to the world using the `#[public]` macro.

```rust
#[public]
impl BtcVerifier {
    /// Hashes a hex string twice using SHA-256 (Bitcoin style)
    pub fn hash_btc_header(&self, header_hex: String) -> Result<String, Vec<u8>> {
        // 1. Decode the input hex string into bytes.
        let bytes = hex::decode(header_hex).map_err(|_| Vec::new())?;
        
        // 2. Perform the first SHA-256 hash.
        let mut hasher1 = Sha256::new();
        hasher1.update(&bytes);
        let hash1 = hasher1.finalize();

        // 3. Perform the second SHA-256 hash.
        // Bitcoin uses "Hash256" (double SHA-256) for block headers.
        let mut hasher2 = Sha256::new();
        hasher2.update(hash1);
        let hash2 = hasher2.finalize();

        // 4. Return the result as a hex string.
        Ok(hex::encode(hash2))
    }
}
```

**Key Takeaways:**
- **Standard Rust**: We are using standard libraries (`hex`, `sha2`) just like normal Rust code.
- **Pure Computation**: This function performs heavy cryptography (`SHA256`) which is much cheaper in Stylus than Solidity.

## How to Test

One of the best things about Stylus is that you can test your contracts using standard Rust unit tests!

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_sha256() {
        let input = "68656c6c6f"; // "hello" in hex
        
        let verifier = BtcVerifier {};
        let result = verifier.hash_btc_header(input.into()).unwrap();

        // "hello" double-SHA256 hash
        assert_eq!(result, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
    }
}
```

Run this test with:
```bash
cargo test
```

## Next Steps

1.  **Install the Stylus CLI**: `cargo install cargo-stylus`
2.  **Check your contract**: `cargo stylus check` verifies your contract will compile to valid WASM for Arbitrum.
3.  **Deploy**: Use `cargo stylus deploy` to put your contract on-chain!

This example demonstrates how simple it is to bring powerful Rust logic to Arbitrum using Stylus. Try extending this contract to verify a full Merkle proof!
