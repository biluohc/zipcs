#[macro_use]
extern crate stderr;
extern crate poolite;
use poolite::Pool;

use std::fs::{File, create_dir_all};
use std::io::{copy, BufReader};
use std::time::Duration;
use std::env::args;
use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

extern crate encoding;
use encoding::{Encoding, DecoderTrap};
use encoding::all::{GB18030, BIG5_2003};

extern crate zip;
use zip::read::ZipArchive;
// use zip::read::ZipFile;

fn main() {
    println!("Hello, Zipno !");
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        errln!("No input ZipArchives: exit()");
        exit(1);
    };

    for zip_arch_path in &args[1..] {
        if let Err(e) = for_zip_arch_file(&zip_arch_path) {
            errln!("Err({:?})", e);
            exit(1);
        }
    }
}
fn for_zip_arch_file(zip_arch_path: &str) -> Result<(), ZipNoError> {
    let zip_arch = File::open(zip_arch_path)?;
    let reader = BufReader::new(zip_arch);
    let mut zip_arch = ZipArchive::new(reader)?;

    let mut map: HashMap<usize, String> = HashMap::new();
    let pool = Pool::new().max(Pool::num_cpus() + 1).run().unwrap();
    for i in 0..zip_arch.len() {
        let file = zip_arch.by_index(i).unwrap();
        let name = {
            if let Ok(o) = GB18030.decode(file.name_raw(), DecoderTrap::Strict) {
                o
            } else if let Ok(o) = BIG5_2003.decode(file.name_raw(), DecoderTrap::Strict) {
                o
            } else {
                file.name().to_owned()
            }
        };

        println!("${}->{:?}: {}", i, name, file.size());

        if name.ends_with("/") {
            create_dir_all(name)?;
        } else {
            map.insert(i, name);
        }
    }
    let zip_arch_mutex = Arc::new(Mutex::new(zip_arch));
    for (i, name) in map.iter() {
        let zip_arch_mutex = zip_arch_mutex.clone();
        let name = name.to_string();
        let i=i.clone();
        pool.spawn(Box::new(move|| unzipfile_matchres(zip_arch_mutex, i, name)));
    }

    loop {
        thread::sleep(Duration::from_millis(100)); //wait for the pool 100ms.
        if pool.is_empty() {
            break;
        }
    }
    Ok(())
}

fn unzipfile_matchres<R: Read + io::Seek>(zip_arch: Arc<Mutex<ZipArchive<R>>>, i: usize, name: String) {
    if let Err(e) = unzipfile(zip_arch, i, &name) {
        errln!("Unzip `${}->{}` fails: {:?}", i, name, e);
    }
}

#[inline]
fn unzipfile<R: Read + io::Seek>(zip_arch: Arc<Mutex<ZipArchive<R>>>, i: usize, name: &String) -> Result<(), ZipNoError> {
    let mut zip_arch = zip_arch.lock().unwrap();
    let mut zip = zip_arch.by_index(i)?;
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
