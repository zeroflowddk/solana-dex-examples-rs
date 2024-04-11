use bincode;
use eyre::Result;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;

/*
    @struct SwapManager
    @params rpc_url - u rpc link
    @params api_base_url - api JupiterV6 Aggregator
*/

pub struct SwapManager {
    pub rpc_url: String,
    pub api_base_url: String,
}

/*
    @impl SwapManager
    @fn new - self dunction for impl
    @fn swap_instructions - function for swap example on Jupiter
*/

impl SwapManager {
    pub fn new(rpc_url: &str, api_base_url: &str) -> Self {
        SwapManager {
            rpc_url: rpc_url.to_string(),
            api_base_url: api_base_url.to_string(),
        }
    }

    pub async fn swap_instructions(
        &self,
        amount: u64,
        input_mint: Pubkey,
        output_mint: Pubkey,
        slippage: u16,
        keypair: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let jupiter_swap_api_client = JupiterSwapApiClient::new(self.api_base_url.clone());

        let quote_request = QuoteRequest {
            amount,
            input_mint,
            output_mint,
            slippage_bps: slippage,
            ..QuoteRequest::default()
        };

        let quote_response = jupiter_swap_api_client.quote(&quote_request).await?;

        let bytes = bs58::decode(keypair)
            .into_vec()
            .map_err(|e| eyre::eyre!("Failed to decode Base58 string: {}", e))?;

        let keypair = Keypair::from_bytes(&bytes)
            .map_err(|e| eyre::eyre!("Failed to create keypair from bytes: {}", e))?;

        let swap_response = jupiter_swap_api_client
            .swap(&SwapRequest {
                user_public_key: keypair.pubkey(),
                quote_response: quote_response.clone(),
                config: TransactionConfig::default(),
            })
            .await?;

        let mut versioned_transaction: VersionedTransaction =
            bincode::deserialize(&swap_response.swap_transaction).unwrap();

        let rpc_client = RpcClient::new(self.rpc_url.clone());
        let latest_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

        versioned_transaction
            .message
            .set_recent_blockhash(latest_blockhash);

        let transaction =
            VersionedTransaction::try_new(versioned_transaction.message.clone(), &[&keypair])
                .unwrap();

        let result = rpc_client.send_and_confirm_transaction(&transaction).await;
        match result {
            Ok(_) => {
                println!("Transaction successful! {:?}", transaction.signatures);
                Ok(())
            }
            Err(err) => {
                println!("Error: {:?}", err);
                Err(err.into())
            }
        }
    }
}
