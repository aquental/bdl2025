# Create a 1-of-4 P2SH multisig address from the public keys in the four inputs of this tx:
#   `37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517`
raw=$(bitcoin-cli getrawtransaction 37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517)
decoded=$(bitcoin-cli decoderawtransaction $raw)
k1=$(echo $decoded | jq -r '.vin[0].txinwitness[1]')
k2=$(echo $decoded | jq -r '.vin[1].txinwitness[1]')
k3=$(echo $decoded | jq -r '.vin[2].txinwitness[1]')
k4=$(echo $decoded | jq -r '.vin[3].txinwitness[1]')
bitcoin-cli createmultisig 1 '["'$k1'","'$k2'","'$k3'","'$k4'"]' | jq -r '.address'
# 3GyWg1CCD3RDpbwCbuk9TTRQptkRfczDz8
