use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc::bitcoin::TxIn;
use rust::get_rpc_from_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let height: u64 = 444431;
    let block_hash = rpc.get_block_hash(height);
    let block = rpc.get_block(block_hash.as_ref().unwrap())?;

    for tx in block.txdata.iter() {
        let has_rbf = tx
            .input
            .iter()
            .any(|vin: &TxIn| vin.sequence.0 < 4294967294u32);
        if has_rbf {
            println!("{}", tx.compute_txid());
        }
    }

    Ok(())
}
/*
# Only one tx in block 444,431 signals opt-in RBF. What is its txid?:
block_hash=$(bitcoin-cli getblockhash 444431)
bitcoin-cli getblock $block_hash 2 | jq -r '.tx[] | select(any(.vin[]; .sequence < 4294967294)) | .txid'
# 9e8cece3df00c95140a230d9cc1a11474bd1e78b750c148ca604e1f4487e05ee
 */
