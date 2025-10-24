# Which tx in block 257,343 spends the coinbase output of block 256,128?
coinbase_hash=$(bitcoin-cli getblockhash 256128)
coinbase_tx=$(bitcoin-cli getblock $coinbase_hash 1 | jq -r '.tx[0]')
block_hash=$(bitcoin-cli getblockhash 257343)
bitcoin-cli getblock $block_hash 2 | jq -r --arg txid $coinbase_tx '.tx[] | select(any(.vin[]; .txid == $txid and .vout == 0)) | .txid'
# c54714cb1373c2e3725261fe201f267280e21350bdf2df505da8483a6a4805fc
