extern crate app;
#[macro_use]
extern crate stderr;
use stderr::Loger;

extern crate poolite;
use poolite::{Pool, IntoPool};

extern crate encoding;
extern crate zip;
use zip::read::ZipArchive;
// use zip::read::ZipFile;

use std::fs::{File, create_dir_all};
use std::io::{copy, BufReader};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::prelude::*;
use std::process::exit;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::io;

mod consts;
use consts::*;
mod args;
use args::{Config, Task};

fn main() {
    init!();
    if let Err(e) = fun() {
        errln!("{}_Error: {}", NAME, &e);
        assert_ne!("", e.trim());
        exit(1);
    };
}
fn fun() -> Result<(), String> {
    let config = Rc::from(Config::get()?);
    dbln!("Config::get(): {:?}", config);
    let pool = match *config.task() {
        Task::LIST => Pool::new(),
        Task::UNZIP => Pool::new().min(0).run().into_pool(),
    };
    for zip_arch_path in config.zips() {
        let config = config.clone();
        if let Err(e) = for_zip_arch_file(&pool, zip_arch_path, config) {
            return Err(format!("{:?}->{:?}", zip_arch_path, e));
        }
    }
    Ok(())
}

fn for_zip_arch_file(pool: &Pool, zip_arch_path: &str, config: Rc<Config>) -> Result<(), ZipCSError> {
    let zip_arch = File::open(zip_arch_path)?;
    let reader = BufReader::new(zip_arch);
    let mut zip_arch = ZipArchive::new(reader)?;

    let mut map: HashMap<usize, String> = HashMap::new();
    if config.task() == &Task::UNZIP && !Path::new(config.outdir()).exists() {
        create_dir_all(config.outdir())?;
    }
    for i in 0..zip_arch.len() {
        let file = match zip_arch.by_index(i) {
            Ok(o) => o,
            Err(e) => {
                errln!("{}_Error: {:?}${:?} ->{:?}", NAME, zip_arch_path, i, e);
                continue;
            }
        };
        let mut name = {
            if let Ok(o) = config.charset().u8slice_to_string(file.name_raw()) {
                o
            } else {
                file.name().to_owned()
            }
        };

        name = match *config.task() {
            Task::LIST => name,
            Task::UNZIP => config.outdir().to_string() + &name,
        };

        if name.ends_with('/') {
            println!("${}-> {:?}", i, name);
            if config.task() == &Task::UNZIP {
                create_dir_all(name)?;
            }
        } else {
            println!("${}-> {:?}: {:?}", i, name, file.size());
            if config.task() == &Task::UNZIP {
                {
                    let path = Path::new(&name);
                    if let Some(p) = path.parent() {
                        if !p.exists() {
                            create_dir_all(&p)?;
                        }
                    }
                }
                map.insert(i, name);
            }
        }
    }
    if config.task() == &Task::LIST {
        return Ok(());
    }

    let zip_arch_mutex = Arc::new(Mutex::new(zip_arch));
    for (i, name) in &map {
        let zip_arch_mutex = zip_arch_mutex.clone();
        let name = name.to_string();
        let i = *i;
        pool.push(move || unzipfile_matchres(zip_arch_mutex, i, name));
    }

    loop {
        thread::sleep(Duration::from_millis(100)); //wait for the pool 100ms.
        if pool.is_empty() {
            break;
        }
    }
    Ok(())
}

#[inline]
fn unzipfile_matchres<R: Read + io::Seek>(zip_arch: Arc<Mutex<ZipArchive<R>>>, i: usize, name: String) {
    if let Err(e) = unzipfile(zip_arch, i, &name) {
        errln!("Unzip `${}->{}` fails: {:?}", i, name, e);
    }
}

#[inline]
fn unzipfile<R: Read + io::Seek>(zip_arch: Arc<Mutex<ZipArchive<R>>>, i: usize, name: &str) -> Result<(), ZipCSError> {
    let mut zip_arch = zip_arch.lock().unwrap();
    let mut zip = zip_arch.by_index(i)?;
    let mut outfile = File::create(name)?;
    copy(&mut zip, &mut outfile)?;
    Ok(())
}

#[derive(Debug)]
enum ZipCSError {
    IO(std::io::Error),
    ZIP(zip::result::ZipError),
}

impl std::error::Error for ZipCSError {
    fn description(&self) -> &str {
        match *self {
            ZipCSError::IO(ref e) => e.description(),
            ZipCSError::ZIP(ref e) => e.description(),
        }
    }
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for ZipCSError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<std::io::Error> for ZipCSError {
    fn from(e: std::io::Error) -> Self {
        ZipCSError::IO(e)
    }
}

impl From<zip::result::ZipError> for ZipCSError {
    fn from(e: zip::result::ZipError) -> Self {
        ZipCSError::ZIP(e)
    }
}
