use bitcoincore_rpc::{
    RpcApi,
    bitcoin::{PublicKey, Txid},
    json,
};
use rust::get_rpc_from_config;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let txid_str = "37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517";
    let txid = Txid::from_str(txid_str)?;

    let verbose_tx = rpc.get_raw_transaction_info(&txid, None)?;

    // Get the witness data as hex strings
    let k1 = &verbose_tx.vin[0].txinwitness.as_ref().unwrap()[1];
    let k2 = &verbose_tx.vin[1].txinwitness.as_ref().unwrap()[1];
    let k3 = &verbose_tx.vin[2].txinwitness.as_ref().unwrap()[1];
    let k4 = &verbose_tx.vin[3].txinwitness.as_ref().unwrap()[1];

    // println!("{:?}", k1);
    // println!("{:?}", k2);
    // println!("{:?}", k3);
    // println!("{:?}", k4);

    // Parse hex strings to PublicKey objects
    let pk1 = PublicKey::from_slice(&k1)?;
    let pk2 = PublicKey::from_slice(&k2)?;
    let pk3 = PublicKey::from_slice(&k3)?;
    let pk4 = PublicKey::from_slice(&k4)?;

    // println!("{:?}", pk1);
    // println!("{:?}", pk2);
    // println!("{:?}", pk3);
    // println!("{:?}", pk4);

    // Create the keys vector with references
    let keys: Vec<json::PubKeyOrAddress> = vec![
        json::PubKeyOrAddress::PubKey(&pk1),
        json::PubKeyOrAddress::PubKey(&pk2),
        json::PubKeyOrAddress::PubKey(&pk3),
        json::PubKeyOrAddress::PubKey(&pk4),
    ];

    //println!("{:?}", keys);

    let multisig = rpc.add_multisig_address(1, &keys, None, None)?;

    println!("{:?}", multisig);

    // Assume valid for the network (mainnet/testnet/regtest)
    let address = multisig.address.assume_checked();
    println!("{}", address);
    // Outputs: 3GyWg1CCD3RDpbwCbuk9TTRQptkRfczDz8

    Ok(())
}
/*
# Create a 1-of-4 P2SH multisig address from the public keys in the four inputs of this tx:
#   `37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517`
raw=$(bitcoin-cli getrawtransaction 37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517)
decoded=$(bitcoin-cli decoderawtransaction $raw)
k1=$(echo $decoded | jq -r '.vin[0].txinwitness[1]')
k2=$(echo $decoded | jq -r '.vin[1].txinwitness[1]')
k3=$(echo $decoded | jq -r '.vin[2].txinwitness[1]')
k4=$(echo $decoded | jq -r '.vin[3].txinwitness[1]')
bitcoin-cli createmultisig 1 '["'$k1'","'$k2'","'$k3'","'$k4'"]' | jq -r '.address'
# 3GyWg1CCD3RDpbwCbuk9TTRQptkRfczDz8
*/
