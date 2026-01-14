# 📘 Arbitrum Stylus: BTCHash Verifier & On-Chain Testing Guide

> [!NOTE]
> **Arbitrum Stylus** empowers you to write efficient smart contracts in Rust (or C/C++) that run alongside Solidity contracts on Arbitrum One and Sepolia. This guide demonstrates a practical BTCFi use case: verifying Bitcoin block headers on-chain.

---

## 🏗️ 1. Understanding the Smart Contract

We have built a **Bitcoin Header Verifier** (`BtcVerifier`). Its primary job is to take a raw Bitcoin block header (in hex) and verify its integrity by performing a **Double SHA-256 Hash**, often called `Hash256` in Bitcoin protocol terms.

### 📝 The Rust Contract (`src/lib.rs`)

The contract logic is simple, clean, and leverages Rust's ecosystem:

```rust
// ... imports ...

#[public]
impl BtcVerifier {
    /// Hashes a hex string twice using SHA-256 (Bitcoin style)
    pub fn hash_btc_header(&self, header_hex: String) -> Result<String, Vec<u8>> {
        // 1. Decode generic hex string
        let bytes = hex::decode(header_hex).map_err(|_| Vec::new())?;
        
        // 2. First SHA-256
        let mut hasher1 = Sha256::new();
        hasher1.update(&bytes);
        let hash1 = hasher1.finalize();

        // 3. Second SHA-256 (Hash256)
        let mut hasher2 = Sha256::new();
        hasher2.update(hash1);
        let hash2 = hasher2.finalize();

        // 4. Return result as hex
        Ok(hex::encode(hash2))
    }
}
```

> [!TIP]
> **Why Rust?**
> Doing this cryptographic heavy lifting in Solidity is expensive (gas-wise). In Stylus (WASM), it is order-of-magnitude cheaper and safer to implement using standard libraries like `sha2`.

---

## 🚀 2. Setting Up & Deploying

### 🛠️ Prerequisites
- **Rust Toolchain**: Stable channel.
- **Stylus CLI**: `cargo install cargo-stylus`
- **Arbitrum Sepolia ETH**: For gas.

### ⚙️ Configuration
Create a `.env` file in your project root:
```ini
PRIVATE_KEY=your_private_key_here
ARB_URL=https://sepolia-rollup.arbitrum.io/rpc
```

### 📦 Deployment
Deploy your contract using the Stylus CLI tool. We use a dummy `bin` target in `Cargo.toml` to satisfy tool requirements, but the core artifact is the library.

```bash
# Sourcing environment variables
source .env

# Deploy (using --no-verify to skip docker for speed in dev)
cargo stylus deploy --private-key $PRIVATE_KEY --endpoint $ARB_URL --no-verify
```

**Success Output:**
```text
Deployed code at address: 0xb4864bb622f3020a5d424ff2cc20738b3327f7e2
Transaction successfully activated contract...
```
*Save this address! You will need it for the interaction script.*

---

## 🧪 3. On-Chain Verification with Script

To truly verify the contract works, we don't just call it locally; we **broadcast a transaction** to the Testnet.

### 📜 The Script (`scripts/src/main.rs`)

We created a robust Rust script using `ethers-rs` to interact with our specific contract ABI.

**Key Features:**
- **Uses `.send()`**: Ensures an actual state-changing transaction (or at least a recorded execution) is broadcast, rather than a local node simulation (`.call()`).
- **Stylus Method Selectors**: Stylus contracts typically export methods in `snake_case` or `camelCase` depending on compilation config. Our script correctly handles the selector mapping.

### 🏃 Running the Verification

1. **Update Target**: Ensure `scripts/src/main.rs` points to your deployed contract address.
   ```rust
   let contract_address: Address = "0xb4864bb...".parse()?;
   ```

2. **Execute**:
   ```bash
   cd scripts
   source ../.env
   cargo run
   ```

3. **Verify Output**:
   ```text
   Broadcasting transaction for hash_btc_header...
   Input: 0200000... (Block Header Hex)
   Transaction successfully broadcasted and included!
   Transaction Hash: 0x87e8ccc...
   Gas Used: Some(66333)
   ```

> [!IMPORTANT]
> **Verification Success:**
> If you see a Transaction Hash and "Transaction successfully broadcasted", your Rust Stylus contract is live and correctly processing cryptographic operations on Arbitrum Sepolia!

---

## 📚 Resources & Next Steps

- **Stylus Docs**: [https://docs.arbitrum.io/stylus](https://docs.arbitrum.io/stylus)
- **Repo Structure**:
    - `src/lib.rs`: Contract source
    - `scripts/`: Verification scripts
    - `Cargo.toml`: Dependency management

Happy Coding! 🦀 + 🔷
