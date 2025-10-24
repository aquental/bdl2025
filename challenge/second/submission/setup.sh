wget https://bitcoincore.org/bin/bitcoin-core-27.1/bitcoin-27.1-x86_64-linux-gnu.tar.gz
tar -xzvf bitcoin-27.1-x86_64-linux-gnu.tar.gz
ln -s $PWD/bitcoin-27.1/bin/* /usr/local/bin/
mkdir -p ~/.bitcoin
echo "rpcconnect=217.76.54.77" >> ~/.bitcoin/bitcoin.conf
echo "rpcuser=autograder" >> ~/.bitcoin/bitcoin.conf
echo "rpcpassword=2NTDj2nFflH9" >> ~/.bitcoin/bitcoin.conf
