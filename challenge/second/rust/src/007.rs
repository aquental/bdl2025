use bitcoincore_rpc::RpcApi;
use rust::get_rpc_from_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let block_height = 123321u64;
    let block_hash = rpc.get_block_hash(block_height)?;
    let block = rpc.get_block(&block_hash)?;

    let mut unspent_addresses = Vec::new();

    for tx in block.txdata.iter() {
        let txid = tx.compute_txid();
        for (n, _vout) in tx.output.iter().enumerate() {
            if let Ok(Some(tx_out_res)) = rpc.get_tx_out(&txid, n as u32, Some(false)) {
                if let Some(addr) = tx_out_res.script_pub_key.address {
                    unspent_addresses.push(addr.clone());
                }
            }
        }
    }

    // Assuming only one unspent output as per the problem
    if unspent_addresses.len() == 1 {
        println!("{}", unspent_addresses[0].clone().assume_checked());
    } else {
        eprintln!(
            "Expected exactly one unspent output, found {}",
            unspent_addresses.len()
        );
    }

    Ok(())
}
/*
# Only one single output remains unspent from block 123,321. What address was it sent to?
blockhash=$(bitcoin-cli getblockhash 123321)
for txid in $(bitcoin-cli getblock $blockhash 2 | jq -r '.tx[].txid')
    do
    num_outputs=$(bitcoin-cli getrawtransaction $txid 1 | jq -r '.vout | length')
    for n in $(seq 0 $((num_outputs-1)))
        do
        if bitcoin-cli gettxout $txid $n | jq -e . > /dev/null 2>&1
        then bitcoin-cli getrawtransaction $txid 1 | jq -r '.vout['$n'].scriptPubKey.address'
        fi
        done
    done

# 1FPDNNmgwEnKuF7GQzSqUcVQdzSRhz4pgX
 */
