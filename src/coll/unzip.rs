use super::consts::*;

use stderr::Loger;
use poolite::{Pool, IntoPool};
use zip::result::ZipError;
use zip::read::ZipArchive;

use std::fs::{File, create_dir_all};
use std::io::{copy, BufReader};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::prelude::*;
use std::fs::read_dir;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::io;
use std;

#[derive(Debug,PartialEq)]
pub enum Task {
    LIST, // zipcs -l/--list
    UNZIP, // Extract files from archive with full paths
}
impl Default for Task {
    fn default() -> Task {
        Task::UNZIP
    }
}

#[derive(Debug)]
pub struct Zips {
    pub charset: CharSet, //zip -cs/--charset   //utf-8
    pub outdir: String, //zipcs -o/--outdir   //./
    pub zips: Vec<String>, //zipcs ZipArchive0 ZipArchive1 ...
    pub task: Task, // UNZIP
}
impl Zips {
    pub fn check_fix(&mut self) -> Result<(), String> {
        if Path::new(&self.outdir).is_file() {
            return Err(format!("outdir {:?} is a file.", self.outdir));
        } else if Path::new(&self.outdir).exists() {
            read_dir(&self.outdir)
                .map_err(|e| format!("outdir {:?} is invalid {:?}.", self.outdir, e))?;
        }
        // dir + name =dirname-> dir/ + name
        if !self.outdir.ends_with('/') {
            self.outdir.push('/');
        }
        assert!(self.outdir.ends_with('/'));
        let name = "ZipArchives";
        for zip in &self.zips {
            let path = Path::new(&zip);
            if !path.exists() {
                return Err(format!("Arguments({}): \"{:?}\" is not exists", name, path));
            } else if path.is_dir() {
                return Err(format!("Arguments({}): \"{:?}\" is a directory", name, path));
            }
            File::open(path)
                .map_err(|_| format!("Arguments({}): \"{:?}\" is invalid", name, path))?;
        }
        Ok(())
    }
    pub fn call(self) -> Result<(), String> {
        dbln!("Config_zip: {:?}", self);
        let config = Rc::from(self);
        dbln!("Config_zip: {:?}", config);
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
    pub fn charset(&self) -> &CharSet {
        &self.charset
    }
    pub fn outdir(&self) -> &String {
        &self.outdir
    }
    pub fn zips(&self) -> &[String] {
        self.zips.as_slice()
    }
    pub fn task(&self) -> &Task {
        &self.task
    }
}
impl Default for Zips {
    fn default() -> Self {
        Zips {
            charset: CharSet::default(),
            outdir: "./".to_string(),
            zips: Vec::new(),
            task: Task::default(),
        }
    }
}

fn for_zip_arch_file(pool: &Pool, zip_arch_path: &str, config: Rc<Zips>) -> Result<(), ZipCSError> {
    let zip_arch = File::open(zip_arch_path)?;
    let reader = BufReader::new(zip_arch);
    let mut zip_arch = ZipArchive::new(reader)?;

    let mut map: HashMap<usize, String> = HashMap::new();
    if *config.task() == Task::UNZIP && !Path::new(config.outdir()).exists() {
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
            if let Ok(o) = config.charset().decode(file.name_raw()) {
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
            if *config.task() == Task::UNZIP {
                create_dir_all(name)?;
            }
        } else {
            println!("${}-> {:?}: {:?}", i, name, file.size());
            if *config.task() == Task::UNZIP {
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
    if *config.task() == Task::LIST {
        return Ok(());
    }

    let zip_arch_mutex = Arc::new(Mutex::new(zip_arch));
    for (i, name) in &map {
        let zip_arch_mutex = zip_arch_mutex.clone();
        let name = name.to_string();
        let i = *i;
        pool.push(move || unzipfile_matchres(zip_arch_mutex, i, &name));
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
fn unzipfile_matchres<R: Read + io::Seek>(zip_arch: Arc<Mutex<ZipArchive<R>>>, i: usize, name: &str) {
    if let Err(e) = unzipfile(zip_arch, i, name) {
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
    ZIP(ZipError),
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

impl From<ZipError> for ZipCSError {
    fn from(e: ZipError) -> Self {
        ZipCSError::ZIP(e)
    }
}
