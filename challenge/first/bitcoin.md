# First test - compile and create a commit

[challenge repo](https://github.com/vinteum-foss-program/bdl-2025-test-the-test-aquental)

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

   1. Check if `v29.1` is commit `fd784f277427aea7b25a8cdcd328b18a9fa64c0d`

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
   > recommended: `cmake -B build -DBUILD_GUI=OFF -DBUILD_TESTS=ON`
   >
   > build: `make -C /Users/aquental/projects/bitcoin/bitcoin/build -j$(sysctl -n hw.ncpu)`

   1. Prerequisites
      ```shell
      brew install cmake boost pkgconf libevent
      ```

4. Run all the functional tests: `build/test/functional/test_runner.py`

   1. create a RAM disk (macOS)
      1. create: `diskutil erasevolume HFS+ ramdisk $(hdiutil attach -nomount ram://8388608)`
      2. run tests using `build/test/functional/test_runner.py --cachedir=/Volumes/ramdisk/cache --tmpdir=/Volumes/ramdisk/tmp`
      3. unmount: `umount /Volumes/ramdisk`
   2. target test: `p2p_seednode.py` (318/318)
   3. change: [**src/net.cpp**](https://github.com/bitcoin/bitcoin/blob/master/src/net.cpp?plain=1#L2678) _line 2678_ : `//add_addr_fetch = true; // BREAK HERE`
   4. rationale: This line is responsible for setting the flag that triggers `seednode` addition to the `addr_fetch` queue.
      The test should fail, specifically the `test_seednode_empty_addrman` and `test_seednode_non_empty_addrman` test cases, as they will no longer see the expected log messages about adding seednodes.

5. generate diff (`git show <commit>`)

   ```diff
   commit 9748c07aff173bdd21a000a2073b657d3ae1534b (HEAD)
   Author: aquental <aquental@users.noreply.github.com>
   Date:   Sun Oct 5 01:53:05 2025 -0300

      break p2p_seednode.py
      comment line 2678 of bitcoin/src/net.cpp
      This line is responsible for setting the flag that triggers `seednode`
      addition to the `addr_fetch` queue. The test should fail, specifically
      the `test_seednode_empty_addrman` and `test_seednode_non_empty_addrman`
      test cases, as they will no longer see the expected log messages about
      adding seednodes.

   diff --git a/src/net.cpp b/src/net.cpp
   index 735985a841..5752e486b7 100644
   --- a/src/net.cpp
   +++ b/src/net.cpp
   @@ -2676,7 +2676,7 @@ void CConnman::ThreadOpenConnections(const std::vector<std::string> connect, Spa
            if (!seed_nodes.empty() && nOutboundFullRelay < SEED_OUTBOUND_CONNECTION_THRESHOLD) {
               if (NodeClock::now() > seed_node_timer + ADD_NEXT_SEEDNODE) {
                  seed_node_timer = NodeClock::now();
   -                add_addr_fetch = true;
   +                //add_addr_fetch = true; // BREAK HERE
               }
            }

   ```
