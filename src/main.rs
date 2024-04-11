mod dexes;
mod utils;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use spl_token_client::client::{ProgramRpcClient, ProgramRpcClientSendTransaction};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    test_swap_jupiter().await;
}

async fn test_swap_jupiter() {
    let rpc_url = "RPC";
    let api_base_url = "https://quote-api.jup.ag/v6";

    let swap_manager = dexes::jupiter::SwapManager::new(rpc_url, api_base_url);

    let private_key = "PRIVATE_KEY";

    let input_token: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
    let output_token: Pubkey = pubkey!("MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5");

    println!("{:?}, {:?}", input_token, output_token);
    let slippage = 50;
    let amount = 1000;

    if let Err(e) = swap_manager
        .swap_instructions(amount, input_token, output_token, slippage, private_key)
        .await
    {
        eprintln!("Error swapping tokens: {}", e);
    }
}

//ADDED test_swap_raydium
