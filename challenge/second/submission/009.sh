# How many satoshis did this transaction pay for fee?:
# c346d9277128f5d67740f8847f11aff5cef440b6d102fcd5ddcdb40d9a12df42
txid="c346d9277128f5d67740f8847f11aff5cef440b6d102fcd5ddcdb40d9a12df42"
decoded=$(bitcoin-cli getrawtransaction "$txid" 1)
# Calculate total input value in BTC (sum values from previous outputs)
input_sum=0
vin_count=$(echo "$decoded" | jq '.vin | length')
for ((i=0; i<vin_count; i++)); do
    prev_tx=$(echo "$decoded" | jq -r ".vin[$i].txid")
    vout_idx=$(echo "$decoded" | jq ".vin[$i].vout")
    prev_decoded=$(bitcoin-cli getrawtransaction "$prev_tx" 1)
    value=$(echo "$prev_decoded" | jq ".vout[$vout_idx].value")
    input_sum=$(echo "$input_sum + $value" | bc -l)
done
# value in BTC
output_sum=$(echo "$decoded" | jq '[.vout[].value] | add')
# fee in BTC
fee_btc=$(echo "$input_sum - $output_sum" | bc -l)
# Convert to satoshis (multiply by 100,000,000)
echo "$fee_btc * 100000000" | bc -l | cut -d. -f1  # Truncate decimal for integer sats
# 8780
