# Only one single output remains unspent from block 123,321. What address was it sent to?
blockhash=$(bitcoin-cli getblockhash 123321)
for txid in $(bitcoin-cli getblock $blockhash 2 | jq -r '.tx[].txid')
    do
    num_outputs=$(bitcoin-cli getrawtransaction $txid 1 | jq -r '.vout | length')
    for n in $(seq 0 $((num_outputs-1)))
        do
        if bitcoin-cli gettxout $txid $n | jq -e . > /dev/null 2>&1
        then bitcoin-cli getrawtransaction $txid 1 | jq -r '.vout['$n'].scriptPubKey.address'
        fi
        done
    done

# 1FPDNNmgwEnKuF7GQzSqUcVQdzSRhz4pgX
