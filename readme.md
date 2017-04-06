# Unzip with charset setting written with Rust.

## Usage

```sh
 cargo install --git https://github.com/biluohc/zipcs
 zipcs --help
 ```

## Or
```sh
git clone https://github.com/biluohc/zipcs --depth 1 
cd zipcs 
cargo build --release

target/release/zipcs --help
```

![zipcs_help.jpg](https://github.com/biluohc/zipcs/blob/master/zipcs_help.jpg?raw=true)

默认使用 **utf8** 编码(可选字符集 `-h/--help` 可以看到)，依次解压每个zip文件到当前目录。  
使用 `-cs/--charset` 指定字符集,`-o/--outdir` 指定输出目录，`-l/--list` 仅列出Zip内容(不解压)。

![zipcs.jpg](https://github.com/biluohc/zipcs/blob/master/zipcs.jpg?raw=true)

## Ps
* 未做预防覆盖原有文件和目录的处理，后果概不负责。
* 未处理文件目录权限（即Zip原有的权限没了）。
* 所依赖的[zip-rs](https://github.com/mvdnes/zip-rs)库目前不支持加密,所以目前不支持密码。
