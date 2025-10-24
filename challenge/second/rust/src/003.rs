use bitcoincore_rpc::{RpcApi, bitcoincore_rpc_json::GetBlockStatsResult};
use rust::get_rpc_from_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    // Get block stats for height 123456
    let stats: GetBlockStatsResult = rpc.get_block_stats(123456)?;

    //println!("{:?}", stats);
    // Print the number of outputs created in the block
    println!("{}", stats.outs);

    Ok(())
}
/*
# How many new outputs were created by block 123,456?
bitcoin-cli getblockstats 123456 | jq '.outs'
# 24
*/
