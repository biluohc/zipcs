[![Build status](https://travis-ci.org/biluohc/zipcs.svg?branch=master)](https://github.com/biluohc/zipcs)

## Useful tools collection.

### Usage

```sh
 cargo +nightly install --git https://github.com/biluohc/zipcs
 zipcs -h
```

### Or

```sh
git clone https://github.com/biluohc/zipcs
cd zipcs
cargo +nightly build --release

./target/release/zipcs --help
```
### Help

```sh
zipcs 0.3.2
Useful tools collection
Wspsxing <biluohc@qq.com>
Repository: https://github.com/biluohc/zipcs

USAGE:
   zipcs options
   zipcs <command> [args]

OPTIONS:
   -h, --help          Show the help message
   -V, --version       Show the version message

CAMMANDS:
   zip, z      Unzip with charset setting
   path, P     Paths decoding with charset setting
   file, f     Files encoding/decoding with charset setting
   ping, p     ping domains/ips
   ip, i       Get ip address
   url, u      Urls decoding/encoding
```

### Binary

* [The Release Page](https://github.com/biluohc/zipcs/releases)

### Ps
* 未做预防覆盖原有文件和目录的处理，后果概不负责。
* 未处理文件目录权限（即Zip原有的权限没了）。
* 所依赖的[zip-rs](https://github.com/mvdnes/zip-rs)库目前不支持加密,所以目前不支持密码。
