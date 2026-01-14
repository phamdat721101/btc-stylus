use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let rpc_url = env::var("ARB_URL").expect("ARB_URL must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet: LocalWallet = private_key.parse()?;
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(421614u64)); // Arbitrum Sepolia Chain ID: 421614
    let client = Arc::new(client);

    println!("Connected to Arbitrum Sepolia");
    
    // contract address will be replaced later
    let contract_address: Address = "0xb4864bb622f3020a5d424ff2cc20738b3327f7e2".parse()?; 

    // Define the function signature: hash_btc_header(string)
    // We can use the low-level call approach or abigen. 
    // Since it's simple, we'll use inline abigen or manually build calldata?
    // Abigen is cleaner.
    
    abigen!(
        BtcVerifier,
        r#"[
            function hashBtcHeader(string memory header_hex) public view returns (string memory)
        ]"#
    );

    let contract = BtcVerifier::new(contract_address, client);
    
    let header_hex = "0200000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c"; // Genesis block header? Or just checking dummy input.
    // The example in lib.rs uses "68656c6c6f" -> "hello"
    // Use the header_hex provided by the user
    println!("Broadcasting transaction for hash_btc_header...");
    println!("Input: {}", header_hex);

    // .send() broadcasts the transaction. await returns a PendingTransaction.
    // .await again waits for the receipt.
    let call = contract.hash_btc_header(header_hex.to_string());
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.expect("Transaction failed to be included in a block");

    println!("Transaction successfully broadcasted and included!");
    println!("Transaction Hash: {:?}", receipt.transaction_hash);
    println!("Gas Used: {:?}", receipt.gas_used);

    Ok(())
}
