extern crate duct;
extern crate app;
#[macro_use]
extern crate stderr;
use stderr::Loger;
extern crate poolite;
extern crate encoding;
extern crate zip;
extern crate urlparse;
extern crate requests;

mod coll;
mod consts;
mod args;
use args::Config;

fn main() {
    init!();
    Config::parse();
}
