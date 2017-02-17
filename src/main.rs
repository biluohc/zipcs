#[macro_use]
extern crate stderr;

use std::fs::{File, create_dir_all};
use std::io::{copy, BufReader};
use std::process::exit;
use std::env::args;

extern crate encoding;
use encoding::{Encoding, DecoderTrap};
use encoding::all::{GB18030, BIG5_2003};

extern crate zip;
use zip::read::ZipArchive;
use zip::read::ZipFile;

fn main() {
    println!("Hello, Zipno !");
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        errln!("No input ZipArchives: exit()");
        exit(1);
    };

    for zip_arch_path in &args[1..] {
        if let Err(e) = for_zip_arch_file(zip_arch_path) {
            errln!("Err({:?})", e);
            exit(1);
        }
    }
}
fn for_zip_arch_file(zip_arch_path: &str) -> Result<(), ZipNoError> {
    let zip_arch = File::open(zip_arch_path)?;
    let reader = BufReader::new(zip_arch);
    let mut zip_arch = ZipArchive::new(reader)?;

    for i in 0..zip_arch.len() {
        let file = zip_arch.by_index(i)?;
        let name = {
            if let Ok(o) = GB18030.decode(file.name_raw(), DecoderTrap::Strict) {
                o
            } else if let Ok(o) = BIG5_2003.decode(file.name_raw(), DecoderTrap::Strict) {
                o
            } else {
                file.name().to_owned()
            }
        };

        if name.ends_with('/') {
            println!("${}->{:?}", i, name);
            create_dir_all(name)?;
        } else {
            println!("${}->{:?}: {}", i, name, file.size());
            unzipfile(file, name)?;
        }
    }
    Ok(())
}

#[inline]
fn unzipfile(mut zip: ZipFile, name: String) -> Result<(), ZipNoError> {
    let mut outfile = File::create(name)?;
    copy(&mut zip, &mut outfile)?;
    Ok(())
}

#[derive(Debug)]
enum ZipNoError {
    IO(std::io::Error),
    ZIP(zip::result::ZipError),
}

impl std::error::Error for ZipNoError {
    fn description(&self) -> &str {
        match *self {
            ZipNoError::IO(ref e) => e.description(),
            ZipNoError::ZIP(ref e) => e.description(),
        }
    }
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for ZipNoError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<std::io::Error> for ZipNoError {
    fn from(e: std::io::Error) -> Self {
        ZipNoError::IO(e)
    }
}

impl From<zip::result::ZipError> for ZipNoError {
    fn from(e: zip::result::ZipError) -> Self {
        ZipNoError::ZIP(e)
    }
}
