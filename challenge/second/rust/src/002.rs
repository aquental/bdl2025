use base64::{Engine as _, engine::general_purpose};
use bitcoincore_rpc::{
    RpcApi,
    bitcoin::{Address, Network, secp256k1},
};
use rust::get_rpc_from_config;
use secp256k1::ecdsa::Signature;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    // Equivalent to: bitcoin-cli verifymessage <address> <signature> <message>
    let address_str = "1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa";
    let address: Address = Address::from_str(address_str)
        .unwrap()
        .require_network(Network::Bitcoin)
        .unwrap();

    let signature_str =
        "HCsBcgB+Wcm8kOGMH8IpNeg0H4gjCrlqwDf/GlSXphZGBYxm0QkKEPhh9DTJRp2IDNUhVr0FhP9qCqo2W0recNM=";
    let message = "1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa";
    let mut verified = false;

    match general_purpose::STANDARD.decode(signature_str) {
        Ok(decoded) => {
            if decoded.len() != 65 {
                println!("Error: Invalid signature length (expected 65 bytes)");
            } else {
                let sig_bytes = &decoded[1..65]; // Skip header byte (recovery ID)
                match Signature::from_compact(sig_bytes) {
                    Ok(sig) => {
                        verified = rpc.verify_message(&address, &sig, message)?;
                    }
                    Err(e) => println!("Error: malformed signature: {:?}", e),
                }
            }
        }
        Err(e) => println!("Error: Base64 decode failed: {:?}", e),
    }

    println!("{}", verified);

    Ok(())
}
/*
# (true / false) Verify the signature by this address over this message:
#   address: `1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa`
#   message: `1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa`
#   signature: `HCsBcgB+Wcm8kOGMH8IpNeg0H4gjCrlqwDf/GlSXphZGBYxm0QkKEPhh9DTJRp2IDNUhVr0FhP9qCqo2W0recNM=`
bitcoin-cli verifymessage "1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa" "HCsBcgB+Wcm8kOGMH8IpNeg0H4gjCrlqwDf/GlSXphZGBYxm0QkKEPhh9DTJRp2IDNUhVr0FhP9qCqo2W0recNM=" "1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa"
# false
*/
