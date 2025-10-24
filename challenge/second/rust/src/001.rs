use bitcoincore_rpc::RpcApi;
use rust::get_rpc_from_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    // Equivalent to: bitcoin-cli getblockhash 654321
    let block_hash = rpc
        .get_block_hash(654321)
        .expect("Failed to get block hash");
    println!("{}", block_hash);
    Ok(())
}
/*
# What is the hash of block 654,321?
bitcoin-cli getblockhash 654321
# 000000000000000000058452bbe379ad4364fe8fda68c45e299979b492858095
*/
