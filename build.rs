extern crate encoding;
extern crate chardet;
extern crate time;

use encoding::label::encoding_from_whatwg_label;
use chardet::{detect, charset2encoding};
use encoding::DecoderTrap;
use time::now_utc;

use std::process::Command as Cmd;
use std::io::{self, Write};
use std::path::PathBuf;
use std::fs::File;
use std::env;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_path = out_dir.join("zipcs.txt");
    let file = File::create(&out_path);
    file.and_then(|mut f| f.write_all(fun().as_bytes()))
        .unwrap()
}

fn fun() -> String {
    let rustc = rustc_version()
        .map(|s| format!("@rustc{}", s.split(' ').nth(1).unwrap()))
        .unwrap_or_default();
    let git = commit_hash()
        .map(|s| (&s[0..8]).to_string())
        .and_then(|s| branch_name().map(|b| format!("{}@{}{} ", s, b, rustc)))
        .unwrap_or_default();

    let version = format!("{} ({}{})", env!("CARGO_PKG_VERSION"), git, datetime());
    format!("pub const VERSION: &'static str = \"{}\";", version)
}

fn datetime() -> String {
    now_utc()
        // .strftime("%Y-%m-%d/%I:%M:%SUTC")
        .strftime("%Y-%m-%dUTC")
        .map(|dt| dt.to_string())
        .unwrap_or_default()
}

fn commit_hash() -> io::Result<String> {
    Cmd::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|o| o.stdout)
        .map(|bytes| decode(bytes.as_slice()).trim().to_string())
}

fn branch_name() -> io::Result<String> {
    Cmd::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|o| o.stdout)
        .map(|bytes| decode(bytes.as_slice()).trim().to_string())
}

fn rustc_version() -> io::Result<String> {
    Cmd::new("rustc")
        .arg("--version")
        .output()
        .map(|o| o.stdout)
        .map(|bytes| decode(bytes.as_slice()).trim().to_string())
}

fn decode(msg: &[u8]) -> String {
    let result = detect(msg);
    if let Some(code) = encoding_from_whatwg_label(charset2encoding(&result.0)) {
        if let Ok(str) = code.decode(msg, DecoderTrap::Strict) {
            return str;
        }
    }
    String::from_utf8_lossy(msg).into_owned().to_owned()
}
