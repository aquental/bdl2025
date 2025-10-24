use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc::bitcoin::ScriptBuf;
use rust::get_rpc_from_config;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let txid_str = "e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163";
    let txid = bitcoincore_rpc::bitcoin::Txid::from_str(txid_str)?;
    let tx = rpc.get_raw_transaction(&txid, None)?;

    // Extract redeem script hex from vin[0].txinwitness[2]
    let redeem_script = tx.input[0].witness.nth(2).ok_or("Missing txinwitness[2]")?;

    // Decode the script
    let decoded_script = ScriptBuf::from(redeem_script.to_vec());

    // Parse ASM: split by whitespace, take the second element (pubkey)
    let script_string = decoded_script.to_string();
    //println!("{:?}", script_string);
    let parts: Vec<&str> = script_string.split_whitespace().collect();
    let pubkey = parts.get(2).ok_or("Invalid ASM format")?;

    println!("{}", pubkey);
    Ok(())
}
/*
# Which public key signed input 0 in this tx:
#   `e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163`
txid="e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163"
decoded=$(bitcoin-cli getrawtransaction $txid 2)
# redeem script hex (witness[2])
redeem_script=$(echo $decoded | jq -r '.vin[0].txinwitness[2]')
decoded_script=$(bitcoin-cli decodescript "$redeem_script")
echo "$decoded_script" | jq -r '.asm | split(" ") | .[1]'
# 025d524ac7ec6501d018d322334f142c7c11aa24b9cffec03161eca35a1e32a71f
 */
