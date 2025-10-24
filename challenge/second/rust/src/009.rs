use std::str::FromStr;

use bitcoincore_rpc::RpcApi;
use rust::get_rpc_from_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let txid_str = "c346d9277128f5d67740f8847f11aff5cef440b6d102fcd5ddcdb40d9a12df42";
    let txid = bitcoincore_rpc::bitcoin::Txid::from_str(txid_str)?;

    // Get verbose transaction details
    let verbose_tx = rpc.get_raw_transaction(&txid, None)?;

    // Sum output values in satoshis
    let mut output_sum: u64 = 0;
    for output in &verbose_tx.output {
        output_sum += output.value.to_sat();
    }

    // Sum input values in satoshis
    let mut input_sum: u64 = 0;
    for vin in &verbose_tx.input {
        let prev_txid = vin.previous_output.txid;
        let vout_idx = vin.previous_output.vout;
        // Get verbose previous transaction
        let prev_verbose = rpc.get_raw_transaction(&prev_txid, None)?;
        // Add the value of the referenced output
        input_sum += prev_verbose.output[vout_idx as usize].value.to_sat();
    }

    // Calculate fee in satoshis
    let fee_sats = input_sum - output_sum;

    println!("{}", fee_sats); // Output: 8780

    Ok(())
}
/*
# How many satoshis did this transaction pay for fee?:
# c346d9277128f5d67740f8847f11aff5cef440b6d102fcd5ddcdb40d9a12df42
txid="c346d9277128f5d67740f8847f11aff5cef440b6d102fcd5ddcdb40d9a12df42"
decoded=$(bitcoin-cli getrawtransaction "$txid" 1)
# Calculate total input value in BTC (sum values from previous outputs)
input_sum=0
vin_count=$(echo "$decoded" | jq '.vin | length')
for ((i=0; i<vin_count; i++)); do
    prev_tx=$(echo "$decoded" | jq -r ".vin[$i].txid")
    vout_idx=$(echo "$decoded" | jq ".vin[$i].vout")
    prev_decoded=$(bitcoin-cli getrawtransaction "$prev_tx" 1)
    value=$(echo "$prev_decoded" | jq ".vout[$vout_idx].value")
    input_sum=$(echo "$input_sum + $value" | bc -l)
done
# value in BTC
output_sum=$(echo "$decoded" | jq '[.vout[].value] | add')
# fee in BTC
fee_btc=$(echo "$input_sum - $output_sum" | bc -l)
# Convert to satoshis (multiply by 100,000,000)
echo "$fee_btc * 100000000" | bc -l | cut -d. -f1  # Truncate decimal for integer sats
# 8780
 */
