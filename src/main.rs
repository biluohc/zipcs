/*!
# Useful tools collection.

## Usage

```sh
 cargo install --git https://github.com/biluohc/zipcs 
 zcs -h
```

## Or

```sh
git clone https://github.com/biluohc/zipcs  
cd zipcs 
cargo build --release

./target/release/zcs --help
```
## Help

```sh
zcs 0.3.5 (ea183538@master@rustc1.23.0-nightly 2017-11-17UTC)
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
   ip, i          Get ip address
   url, u         Urls decoding/encoding
```

## Binary

* [The Release Page](https://github.com/biluohc/zipcs/releases)  

## Ps
* 所依赖的[zip-rs](https://github.com/mvdnes/zip-rs)库目前不支持加密,所以目前不支持密码。
*/

#[macro_use]
extern crate lazy_static;
extern crate encoding;
extern crate urlparse;
extern crate reqwest;
extern crate chardet;
extern crate filetime;
extern crate regex;
extern crate rayon;
extern crate zip;

#[macro_use]
extern crate stderr;
extern crate app;

mod coll;
pub mod consts;
mod args;
use args::Config;

fn main() {
    logger_init!();
    Config::parse();
}
