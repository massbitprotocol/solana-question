use solana_client::rpc_client::RpcClient;
use solana_transaction_status::{EncodedConfirmedBlock, UiTransactionEncoding};
use serde_json;
const RPC_BLOCK_ENCODING: UiTransactionEncoding = UiTransactionEncoding::Base64;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Waiting for chain-reader");

    let new_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    if let Ok(block) = new_client.get_block_with_encoding(95206555, RPC_BLOCK_ENCODING) {
        let payload = serde_json::to_vec(&block).unwrap();
        println!("block: {:?}", block.blockhash);
        let decode_block: EncodedConfirmedBlock = serde_json::from_slice(&payload).unwrap();
        println!("decode_block: {:?}", &decode_block.blockhash);
    }

    Ok(())
}
