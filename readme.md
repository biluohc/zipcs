[![Build status](https://travis-ci.org/biluohc/zipcs.svg?branch=master)](https://github.com/biluohc/zipcs)

# Useful tools collection.

## Usage

```sh
 cargo +nightly install --git https://github.com/biluohc/zipcs 
 zipcs --help
 ```

## Or
```sh
git clone https://github.com/biluohc/zipcs  
cd zipcs 
cargo +nightly build --release

./target/release/zipcs --help
```
## Help
```sh
 D/cache zipcs -h
INFO:
  Zipcs - 0.x.x
  Useful tools collection

USAGE:
  Zipcs [global options] command [command options] [arguments...]

GLOBAL OPTIONS:
   -h, --help       show the help message
   -v, --version    show the version message

COMMANDS:
    file    File encoding/decoding with charset setting
    ip      Get ip address
    path    Path decoding with charset setting
    ping    ping domains/ips
    url     Urls decoding/encoding
    zip     Unzip with charset setting

ARGS:
   <Files>          Files need to encode/decode
   <Paths>          Paths need to decode
   <Hosts/IPs>      Hosts or IPs need to ping
   <Urls>           Urls need to decode/encode
   <ZipArchives>    ZipArchives need to unzip
```

## Binary

* [The Release Page](https://github.com/biluohc/zipcs/releases)  

## Ps
* 未做预防覆盖原有文件和目录的处理，后果概不负责。
* 未处理文件目录权限（即Zip原有的权限没了）。
* 所依赖的[zip-rs](https://github.com/mvdnes/zip-rs)库目前不支持加密,所以目前不支持密码。
