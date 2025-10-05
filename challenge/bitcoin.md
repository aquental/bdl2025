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
   > get number of processors: `sysctl -n hw.ncpu`
   >
   > recommended `cmake -B build -DBUILD_GUI=OFF -DBUILD_TESTS=ON`
   >
   > build `make -C /Users/aquental/projects/bitcoin/bitcoin/build -j$(sysctl -n hw.ncpu)`

```shell
brew install cmake boost pkgconf libevent
```

4. Run all the functional tests: `build/test/functional/test_runner.py`
   1. create a RAM disk (macOS)
      1. create: `diskutil erasevolume HFS+ ramdisk $(hdiutil attach -nomount ram://8388608)`
      2. run tests using `build/test/functional/test_runner.py --cachedir=/Volumes/ramdisk/cache --tmpdir=/Volumes/ramdisk/tmp`
      3. unmount: `umount /Volumes/ramdisk`
   2. target test: `??`
   3. change: _line x_
