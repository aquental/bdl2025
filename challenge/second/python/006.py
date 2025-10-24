from bitcoinrpc.authproxy import AuthServiceProxy

rpc = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

# Step 1: Get the block hash for height 256128
coinbase_height = 256128
coinbase_hash = rpc.getblockhash(coinbase_height)

# Step 2: Get the coinbase transaction ID (first tx in the block, verbosity 1)
coinbase_block = rpc.getblock(coinbase_hash, 1)
coinbase_txid = coinbase_block['tx'][0]

# Step 3: Get the block hash for height 257343
target_height = 257343
target_hash = rpc.getblockhash(target_height)

# Step 4: Get the full block details (verbosity 2) and find the spending transaction
target_block = rpc.getblock(target_hash, 2)
spending_txid = None
for tx in target_block['tx']:
    # Skip the coinbase transaction itself
    if tx['txid'] == coinbase_txid:
        continue
    for vin in tx['vin']:
        # Check for inputs that are not coinbase and match the coinbase txid and vout 0
        if 'coinbase' not in vin and vin['txid'] == coinbase_txid and vin['vout'] == 0:
            spending_txid = tx['txid']
            break
    if spending_txid:
        break

print(spending_txid)
