from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

txid = "e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163"

# Get the decoded raw transaction (verbose=True)
decoded_tx = rpc_connection.getrawtransaction(txid, True)

# Extract redeem script hex from vin[0].txinwitness[2]
redeem_script_hex = decoded_tx['vin'][0]['txinwitness'][2]

# Decode the script
decoded_script = rpc_connection.decodescript(redeem_script_hex)

# Get the asm, split by space, and take the second element (index 1)
asm_parts = decoded_script['asm'].split()
public_key = asm_parts[1]

# Outputs: 025d524ac7ec6501d018d322334f142c7c11aa24b9cffec03161eca35a1e32a71f
print(public_key)
