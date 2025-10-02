# First test - compile and create a commit

1. Clone Bitcoin core

```shell
git clone https://github.com/bitcoin/bitcoin.git
```

2. Get tag `v29.1`:

```shell
cd bitcoin
git fetch --tags
git checkout v29.1
```

2.1 Check if `v29.1` is commit `fd784f277427aea7b25a8cdcd328b18a9fa64c0d`

```shell
if [ "$(git rev-parse HEAD)" = "$(git rev-parse fd784f277427aea7b25a8cdcd328b18a9fa64c0d)" ];
then echo "Yes";
else echo "No match";
fi
```

3. Build Bitcoin Core (MacOS)
   > [docs: Building v29.1](https://github.com/bitcoin/bitcoin/tree/v29.1/doc#building)
   >
   > recommended `cmake -B build -DBUILD_GUI=OFF -DBUILD_TESTS=ON -j 8`

```shell
brew install cmake boost pkgconf libevent
brew install zeromq
```
