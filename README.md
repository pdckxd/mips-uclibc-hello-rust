## Introduce
1. Demo how to build rust application for router with mipsel CPU (MT7621)
2. Demo how to link libcurl built with uclibc lib. See src/main.rs and .cargo/config.toml (#[link (name="xxx")])
3. Demo how to call C function "curl_version" from libcurl.so and convert return c_char value into &str and print it out. See src/main.rs

## Pre-request
1. Build Padavan firmware to getting all necessary native libraries for route os according to https://www.kiloleaf.com/posts/cross-compile-rust-for-openwrt/
   1. It's better to flash firmware built by yourself to avoid some version miss match issue
2. Build rustc toolchain for mipsel-unknown-linux-uclibc target according to https://www.kiloleaf.com/posts/cross-compile-rust-for-openwrt/

## Limitation
1. Since uclibc is not fully supported by rust. Most of 3rd part library won't be built such as 
```
user@user-VirtualBox:~/rust_projects/hello$ cargo build --release --target=mipsel-unknown-linux-uclibc
    Updating crates.io index
  Downloaded openssl v0.9.24
  Downloaded lazy_static v0.2.11
  Downloaded bitflags v0.9.1
  Downloaded minihttp v0.1.9  <===== I add this one into Cargo.toml. But hit this issue. Will be appreciated if someone can help to resovle this failure
  Downloaded miniurl v0.1.3
  Downloaded minihttpse v0.1.6
  Downloaded native-tls v0.1.5
  Downloaded 7 crates (208.3 KB) in 3.33s
   Compiling pkg-config v0.3.20
   Compiling cc v1.0.70
   Compiling autocfg v1.0.1
   Compiling libc v0.2.102
error[E0463]: can't find crate for `std`
```

## TODO
[ ] 1. Create a docker container to put all of them together to ease the life

## Note

1. .cargo/config.toml -L (should have no space after -L otherwise rustc will not find any libraries)
```
link-arg=-L/opt/rt-n56u/toolchain-mipsel/toolchain-3.4.x/mipsel-linux-uclibc/sysroot/lib/
```
2. Ninja not found by CMake
```
ln -s /usr/bin/ninja /usr/sbin/ninja
```

3. Setup environment variables correctly before calling cargo build xxxx, suggest adding to ~/.bashrc
```
export PATH=$PATH:/opt/rt-n56u/toolchain-mipsel/toolchain-3.4.x/bin
export STAGING_DIR=/opt/rt-n56u/toolchain-mipsel/toolchain-3.4.x
export CC_mips_unknown_linux_uclibc=mipsel-linux-uclibc-gcc
```

4. How to debug if rustc find library specified by "-L" or "-l"
```
strace rustc 2>&1 | grep <lib name>
```

5. Git clone rust repo with all submodule otherwise you will get error when calling python ./x.py install
```
git clone https://github.com/rust-lang/rust.git
git submodule update --init --recursive
```

## How to

1. Cross Build
```
cargo build --release --target=mipsel-unknown-linux-uclibc
# strip binary size
mipsel-linux-uclibc-strip ~/rust_projects/mips-uclibc-hello-rust/target/mipsel-unknown-linux-uclibc/release/mips-uclibc-hello-rust
# scp binary to route with mips MT7621
scp target/mipsel-unknown-linux-uclibc/release/mips-uclibc-hello-rust admin@192.168.1.4:/tmp
# call the binary
/tmp/mips-uclibc-hello-rust

CPU info Example:
K2P:/tmp # cat /proc/cpuinfo
system type             : MediaTek MT7621 SoC
processor               : 0
cpu model               : MIPS 1004Kc V2.15
```

## Reference

1. https://www.kiloleaf.com/posts/cross-compile-rust-for-openwrt/
2. https://github.com/hanwckf/rt-n56u
3. https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html
4. https://stackoverflow.com/questions/24145823/how-do-i-convert-a-c-string-into-a-rust-string-and-back-via-ffi

## Useful tools
1. tmux
2. htop

## Backup
1. May need to setup GW manually
```
ip ro add default via 192.168.1.1
```