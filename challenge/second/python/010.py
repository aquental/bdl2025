from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

# Step 1: Get the block hash for height 444431
block_hash = rpc_connection.getblockhash(444431)
# print(f"Block hash: {block_hash}")

# Step 2: Get the block details with verbosity 2 (includes full tx details)
block = rpc_connection.getblock(block_hash, 2)

# Step 3: Find the tx where any vin has sequence < 4294967294 (opt-in RBF signal)
rbf_txid = None
for tx in block['tx']:
    for vin in tx['vin']:
        if 'sequence' in vin and vin['sequence'] < 4294967294:
            rbf_txid = tx['txid']
            break
    if rbf_txid:
        break

if rbf_txid:
    print(rbf_txid)
else:
    print("No RBF-signaling tx found.")
