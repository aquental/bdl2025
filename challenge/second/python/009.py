from bitcoinrpc.authproxy import AuthServiceProxy
import decimal

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

txid = "c346d9277128f5d67740f8847f11aff5cef440b6d102fcd5ddcdb40d9a12df42"

# Decode the transaction with verbose info (equivalent to bitcoin-cli getrawtransaction txid 1)
decoded_tx = rpc_connection.getrawtransaction(txid, True)

# Calculate total input value in BTC (sum values from previous outputs)
input_sum = decimal.Decimal('0')
for vin in decoded_tx['vin']:
    prev_txid = vin['txid']
    vout_idx = vin['vout']
    prev_decoded = rpc_connection.getrawtransaction(prev_txid, True)
    value = decimal.Decimal(str(prev_decoded['vout'][vout_idx]['value']))
    input_sum += value

# Sum of output values in BTC
output_sum = sum(decimal.Decimal(str(vout['value']))
                 for vout in decoded_tx['vout'])

# Fee in BTC
fee_btc = input_sum - output_sum

# Convert to satoshis (multiply by 100,000,000) and take integer part
fee_sats = int(fee_btc * 100000000)

print(fee_sats)  # Outputs: 8780
