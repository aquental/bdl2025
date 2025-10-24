from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

# Equivalent to: bitcoin-cli getblockhash 654321
block_hash = rpc_connection.getblockhash(654321)
print(block_hash)
