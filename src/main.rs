extern crate encoding;
extern crate urlparse;
extern crate requests;
extern crate zip;

extern crate app;
extern crate poolite;
#[macro_use]
extern crate stderr;
use stderr::Loger;

mod coll;
mod consts;
mod args;
use args::Config;

fn main() {
    Loger::init(module_path!());
    Config::parse();
}
