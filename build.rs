extern crate chardet;
extern crate encoding;
extern crate time;

use chardet::{charset2encoding, detect};
use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;
use time::now_utc;

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command as Cmd;

/// `include!(concat!(env!("OUT_DIR"), "/zipcs.txt"));`
fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_path = out_dir.join("zipcs.txt");
    File::create(&out_path)
        .and_then(|mut f| f.write_all(fun().as_bytes()))
        .unwrap()
}

fn fun() -> String {
    let rustc = rustc_version()
        .map(|s| format!(" rustc{}", s.split(' ').nth(1).unwrap()))
        .unwrap_or_default();
    let git = commit_hash()
        .and_then(|s| branch_name().map(|b| format!("{}@{}{} ", s, b, rustc)))
        .unwrap_or_default();

    let version = format!("{} ({}{})", env!("CARGO_PKG_VERSION"), git, date_time());
    format!("pub const VERSION: &str = \"{}\";", version)
}

// date --help
fn date_time() -> String {
    now_utc()
        // .strftime("%Y-%m-%d/%H:%M:%SUTC")
        .strftime("%Y-%m-%dUTC")
        .map(|dt| dt.to_string())
        .unwrap_or_default()
}

// git describe --always --abbrev=10 --dirty=-modified
fn commit_hash() -> io::Result<String> {
    Cmd::new("git")
        .args(&["describe", "--always", "--abbrev=8", "--dirty=-modified"])
        .output()
        .map(|o| decode(&o.stdout))
        .map(|s| s.trim().to_string())
}

fn branch_name() -> io::Result<String> {
    Cmd::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|o| decode(o.stdout.as_slice()).trim().to_string())
}

fn rustc_version() -> io::Result<String> {
    Cmd::new("rustc")
        .arg("--version")
        .output()
        .map(|o| decode_utf8_unchecked(o.stdout).trim().to_string())
}

fn decode_utf8_unchecked(bytes: Vec<u8>) -> String {
    unsafe { String::from_utf8_unchecked(bytes) }
}

fn decode(bytes: &[u8]) -> String {
    encoding_from_whatwg_label(charset2encoding(&detect(bytes).0))
        .and_then(|code| code.decode(bytes, DecoderTrap::Strict).ok())
        .unwrap_or_else(|| String::from_utf8_lossy(bytes).into_owned())
}
