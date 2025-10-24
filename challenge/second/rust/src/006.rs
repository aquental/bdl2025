use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc::bitcoincore_rpc_json::GetBlockResult;
use rust::get_rpc_from_config;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let coinbase_height: u64 = 256128;
    let coinbase_hash = rpc.get_block_hash(coinbase_height)?;
    let coinbase_block: GetBlockResult = rpc.get_block_info(&coinbase_hash)?;
    let coinbase_txid = coinbase_block.tx[0].clone();

    let spending_height: u64 = 257343;
    let spending_hash = rpc.get_block_hash(spending_height)?;
    let params = [
        Value::String(spending_hash.to_string()),
        Value::Number(serde_json::Number::from(2u64)),
    ];
    let verbose_block: Value = rpc.call("getblock", &params)?;

    let txs = verbose_block["tx"]
        .as_array()
        .ok_or("Invalid block structure")?;
    //println!("{:?}------\n", txs);
    for tx_json in txs {
        let txid = tx_json["txid"].as_str().ok_or("Missing txid")?.to_string();
        let vins = tx_json["vin"].as_array().ok_or("Missing vin")?;
        //println!("{:?}------\n", vins);
        for vin_json in vins {
            if let (Some(vin_txid), Some(vin_vout)) =
                (vin_json["txid"].as_str(), vin_json["vout"].as_u64())
            {
                if vin_txid == coinbase_txid.to_string() && vin_vout == 0 {
                    println!("{}", txid);
                    return Ok(());
                }
            }
        }
    }

    Err("Spending tx not found".into())
}
/*
# Which tx in block 257,343 spends the coinbase output of block 256,128?
coinbase_hash=$(bitcoin-cli getblockhash 256128)
coinbase_tx=$(bitcoin-cli getblock $coinbase_hash 1 | jq -r '.tx[0]')
block_hash=$(bitcoin-cli getblockhash 257343)
bitcoin-cli getblock $block_hash 2 | jq -r --arg txid $coinbase_tx '.tx[] | select(any(.vin[]; .txid == $txid and .vout == 0)) | .txid'
# c54714cb1373c2e3725261fe201f267280e21350bdf2df505da8483a6a4805fc
 */
