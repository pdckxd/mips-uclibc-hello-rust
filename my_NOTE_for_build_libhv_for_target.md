## Note

1. How to build with mipsel-unknown-linux-uclibc target
```
export CROSS_COMPILE=mipsel-linux-uclibc-
vim config.ini to enable OPENSSL support
export C_INCLUDE_PATH=$C_INCLUDE_PATH:/opt/rt-n56u/trunk/libs/libssl/openssl-1.1.1k/include/
export LDFLAGS="-L /opt/rt-n56u/trunk/stage/lib/"
./configure --with-openssl
```
