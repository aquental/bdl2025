use bitcoincore_rpc::RpcApi;
use rust::get_rpc_from_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the RPC endpoint with credentials
    let rpc = get_rpc_from_config().expect("Failed to connect to Bitcoin Core RPC");

    let pk = "xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2";
    let derived = 100;
    let descriptor = format!("tr({}/{})", pk, derived);
    //println!("Descriptor: {}", descriptor);

    // Get descriptor info and checksum
    let desc_info = rpc.get_descriptor_info(&descriptor)?;
    let checksum = desc_info
        .checksum
        .as_ref()
        .expect("Checksum should be present");
    let full_descriptor = format!("{}#{}", descriptor, checksum);

    //println!("Full descriptor: {}", full_descriptor);

    // Derive the address (single address for non-ranged descriptor)
    let addresses = rpc.derive_addresses(&full_descriptor, None)?;

    match addresses.get(0) {
        Some(address) => {
            // address is of type Address<NetworkUnchecked>, assume network is valid to display it
            println!("{}", address.clone().assume_checked());
        }
        None => println!("No address found"),
    }

    Ok(())
}
/*
# Using descriptors, compute the taproot address at index 100 derived from this extended public key:
#   `xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2`

CHECKSUM=$(bitcoin-cli getdescriptorinfo "tr(xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2/100)" | jq -r '.checksum')
bitcoin-cli deriveaddresses "tr(xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2/100)#$CHECKSUM" | jq -r '.[0]'
# bc1p3yrtpvv6czx63h2sxwmeep8q98h94w4288fc4cvrkqephalydfgszgacf9
*/
