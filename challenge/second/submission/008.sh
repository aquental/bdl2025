# Which public key signed input 0 in this tx:
#   `e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163`
txid="e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163"
decoded=$(bitcoin-cli getrawtransaction $txid 2)
# redeem script hex (witness[2])
redeem_script=$(echo $decoded | jq -r '.vin[0].txinwitness[2]')
decoded_script=$(bitcoin-cli decodescript "$redeem_script")
echo "$decoded_script" | jq -r '.asm | split(" ") | .[1]'
# 025d524ac7ec6501d018d322334f142c7c11aa24b9cffec03161eca35a1e32a71f
