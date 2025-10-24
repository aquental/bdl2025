from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

# The descriptor without checksum
descriptor = "tr(xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2/100)"

# Step 1: Get descriptor info to retrieve the checksum
info = rpc_connection.getdescriptorinfo(descriptor)
checksum = info['checksum']

# Step 2: Append the checksum to the descriptor
full_descriptor = f"{descriptor}#{checksum}"

# Step 3: Derive the address (defaults to single address, no range)
addresses = rpc_connection.deriveaddresses(full_descriptor)
taproot_address = addresses[0]

print(taproot_address)
