from bitcoinrpc.authproxy import AuthServiceProxy

rpc_connection = AuthServiceProxy(
    "http://user_013:nml3pjkYBusJ@217.76.54.77:8332")

# Parameters equivalent to the bitcoin-cli command
address = "1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa"
signature = "HCsBcgB+Wcm8kOGMH8IpNeg0H4gjCrlqwDf/GlSXphZGBYxm0QkKEPhh9DTJRp2IDNUhVr0FhP9qCqo2W0recNM="
message = "1E9YwDtYf9R29ekNAfbV7MvB4LNv7v3fGa"

# Equivalent to: bitcoin-cli verifymessage <address> <signature> <message>
is_valid = rpc_connection.verifymessage(address, signature, message)
if is_valid:
    print("true")
else:
    print("false")
