Cross Compile Rust For OpenWRT
Posted Aug 9, 2020 by 前尘逐梦
Updated Feb 17
1. 背景
   编译环境（x86_64，Windows Subsystem for Linux）：

OpenWRT SDK: OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2

工具链目录：/mnt/f/wsl/OpenWRT/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2

rust 版本：1.46.0（官方不会构建 uclibc 的版本，所以有些版本下，uclibc 会编译失败，例如 1.48.0。1.43.0、1.46.0 是可以编译通过的）

目录内的 “mips-openwrt-linux-“ 工具是指向 “mips-openwrt-linux-uclibc-“ 的软连接，即工具链使用的是 uclibc 的版本。

uClibc 和 glibc 有差异，有些应用使用 uClibc 可能无法编译。

Rust 目前的官方构建还不支持 mips-unknown-linux-uclibc[1]，从目前 Rust 支持的平台来看，uclibc 对应的 std 是支持的（但是稳定版中没有），target-list 中虽然可以显示出 mips-unknown-linux-uclibc，但是添加是会显示失败。

$ rustc --print target-list | grep mips
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips-unknown-linux-uclibc
2. 设置环境变量
   需要将工具链相关的路径添加到环境变量中[2]：

export PATH=$PATH:/mnt/f/wsl/OpenWRT/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2/staging_dir/toolchain-mips_34kc_gcc-4.8-linaro_uClibc-0.9.33.2/bin/
export STAGING_DIR=/mnt/f/wsl/OpenWRT/OpenWrt-SDK-ar71xx-for-linux-x86_64-gcc-4.8-linaro_uClibc-0.9.33.2/staging_dir/toolchain-mips_34kc_gcc-4.8-linaro_uClibc-0.9.33.2
export CC_mips_unknown_linux_uclibc=mips-openwrt-linux-uclibc-gcc
3. 交叉编译
   基于 tag 1.46.0 创建分支 1.46.0

git checkout -b 1.46.0 1.46.0 --recurse-submodules -f
从 config.toml.example 复制一份 config.toml 文件，修改 config.toml 文件，指定要编译的 target，使能工具编译（默认不编），指定安装目录（相对路径，如果是系统的路径，安装时没有权限，会失败）及交叉编译用到的工具链：

# 修改
target = ["mips-unknown-linux-uclibc"]
extended = true
tools = ["cargo", "rls", "clippy", "rustfmt", "analysis", "src"]
prefix = "/mnt/f/wsl/usr/local"
sysconfdir = "etc"
docdir = "share/doc/rust"
bindir = "bin"
libdir = "lib"
mandir = "share/man"

# 添加（在 [target.x86_64-unknown-linux-gnu] 之后）
[target.mips-unknown-linux-uclibc]
cc = "mips-openwrt-linux-uclibc-gcc"
cxx = "mips-openwrt-linux-uclibc-c++"
ar = "mips-openwrt-linux-uclibc-ar"
linker = "mips-openwrt-linux-uclibc-gcc"
编译并安装：

~/wsl/rust$ ./x.py build && ./x.py install  
安装完成后库和可执行文件位于以下目录：

~/wsl/usr/local/lib
~/wsl/usr/local/bin
如果安装了 rustup，可以为其添加刚才编译好的工具链[3]：

$ rustup toolchain link my_toolchain ~/wsl/usr/local
效果是在 .rustup 目录下创建软链接。

添加之后可以看到对应的工具链：

$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/dell/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
my_toolchain

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.48.0 (7eac88abb 2020-11-16)
# 将编译好的工具链设置为默认工具链
$ rustup default my_toolchain
info: default toolchain set to 'my_toolchain'
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/dell/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
my_toolchain (default)

active toolchain
----------------

my_toolchain (default)
rustc 1.46.0-dev
如果编译时没有重新编译 cargo，而是使用预先安装好的，可能会存在和 rustc 版本不匹配的问题（cargo 传给 rustc 的某些选项，rustc 无法识别）。例如，cargo 版本号为 1.5，rustc 版本号为 1.43.0，cargo 传给 rustc 的选项 embed-bitcode 无法被 rustc 识别。编译时顺带编译了 cargo，添加 rustup 工具链并且将添加的编译链设置为默认工具链后，调用的 cargo 就是顺带编译出的 cargo，不存在版本不匹配的问题。

如果没有安装 rustup，将路径添加到环境变量（或者修改 ~/.profile 文件）即可：

$ export PATH=$PATH:~/wsl/usr/local/bin
$ export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:~/wsl/usr/local/lib
4. 测试
   编译 hello 工程：

$ cargo new hello
添加 hello/.cargo/config.toml 配置文件：

[target.mips-unknown-linux-uclibc]
linker = "mips-openwrt-linux-uclibc-gcc"
ar = "mips-openwrt-linux-uclibc-ar"
构建：

hello$ cargo build --target=mips-unknown-linux-uclibc
hello$ mips-openwrt-linux-strip target/mips-unknown-linux-uclibc/debug/hello
或者在 config.toml 中增加 build 选项：

[build]
target = "mips-unknown-linux-uclibc"
# target = "x86_64-unknown-linux-gnu"

[target.mips-unknown-linux-uclibc]
linker = "mips-openwrt-linux-uclibc-gcc"
ar = "mips-openwrt-linux-uclibc-ar"
构建：

hello$ cargo build
如果编译单个文件：

$ rustc hello.rs --target=mips-unknown-linux-uclibc -C linker=mips-openwrt-linux-uclibc-gcc -C ar=mips-openwrt-uclibc-ar
$ mips-openwrt-linux-strip hello
参考
[1] https://forge.rust-lang.org/release/platform-support.html

[2] https://openwrt.org/docs/guide-developer/crosscompile

[3] https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#creating-a-rustup-toolchain