# Only one tx in block 444,431 signals opt-in RBF. What is its txid?:
block_hash=$(bitcoin-cli getblockhash 444431)
bitcoin-cli getblock $block_hash 2 | jq -r '.tx[] | select(any(.vin[]; .sequence < 4294967294)) | .txid'
# 9e8cece3df00c95140a230d9cc1a11474bd1e78b750c148ca604e1f4487e05ee
