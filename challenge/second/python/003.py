from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

block_stats = rpc_connection.getblockstats(123456)
num_outputs = block_stats['outs']
print(num_outputs)
