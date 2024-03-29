[![Build status](https://travis-ci.org/biluohc/zipcs.svg?branch=master)](https://github.com/biluohc/zipcs)

## Useful tools collection.

### Usage

```sh
 cargo install --git https://github.com/biluohc/zipcs
 zcs -h
```

### Or

```sh
git clone https://github.com/biluohc/zipcs
cd zipcs
cargo build --release

./target/release/zcs --help
```
### Help

```sh
zcs 0.3.8 (b6127dc7@master rustc1.59.0 2022-04-13UTC)
Useful tools collection.
Wspsxing <biluohc@qq.com>
Repo: https://github.com/biluohc/zipcs

USAGE:
   zcs options 
   zcs <command> [args]

OPTIONS:
   -h, --help          Show the help message
   -V, --version       Show the version message

CAMMANDS:
   zip, z         Unzip with charset setting
   path, P        Paths decoding with charset setting
   file, f        Files encoding/decoding with charset setting
   ping, p        ping domains/ips
   chardet, c     Detect the charset for File(for reference)
   charset, C     Show all CharSet supported
   ip, i          Get ip address and it's location
   url, u         Urls decoding/encoding
```

### Binary

* [The Release Page](https://github.com/biluohc/zipcs/releases)

### Ps
* 所依赖的[zip-rs](https://github.com/mvdnes/zip-rs)库目前不支持 Multi-disk。
