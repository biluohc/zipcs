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
zcs 0.3.6 (594c5ca7@master rustc1.25.0-nightly 2018-03-02UTC)
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
extern crate chardet;
extern crate chrono;
extern crate encoding;
extern crate filetime;
extern crate futures;
extern crate percent_encoding;
extern crate rayon;
extern crate regex;
extern crate reqwest;
extern crate tokio;
extern crate tokio_process;
extern crate zip;

extern crate app;
#[macro_use]
extern crate nonblock_logger;
#[macro_use]
extern crate lazy_static;

pub mod args;
pub mod coll;
pub mod consts;
pub mod logger;

use args::Config;

fn main() {
    let _handle = logger::logger_init(1);
    Config::parse();
}
