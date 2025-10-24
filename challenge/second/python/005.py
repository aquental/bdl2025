from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

txid = "37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517"

# Get raw transaction
raw_tx = rpc_connection.getrawtransaction(txid)

# Decode raw transaction
decoded = rpc_connection.decoderawtransaction(raw_tx)

# Extract public keys from witness data of the first four inputs
pubkeys = []
for i in range(4):
    witness = decoded["vin"][i]["txinwitness"]
    # Index 1 is the public key (after signature at index 0)
    pubkey = witness[1]
    pubkeys.append(pubkey)

# Create 1-of-4 P2SH multisig
multisig_info = rpc_connection.createmultisig(1, pubkeys)

# Output the address
print(multisig_info["address"])
