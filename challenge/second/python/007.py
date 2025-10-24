from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

block_height = 123321
block_hash = rpc_connection.getblockhash(block_height)
block = rpc_connection.getblock(block_hash, 2)  # Verbosity 2 includes full tx details

unspent_addresses = []
for tx in block['tx']:
    txid = tx['txid']
    for n, output in enumerate(tx['vout']):
        # gettxout returns None if spent, dict if unspent
        if rpc_connection.gettxout(txid, n):
            if 'address' in output['scriptPubKey']:
                unspent_addresses.append(output['scriptPubKey']['address'])

# Since only one unspent output, print it
if unspent_addresses:
    print(unspent_addresses[0])
else:
    print("No unspent outputs found.")
