use super::consts::*;

use zip::result::ZipError;
use zip::read::ZipArchive;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use std::fs::{File, create_dir_all, Permissions, set_permissions};
use std::io::{copy, BufReader};
use std::ffi::OsString;
use std::error::Error;
use std::fs::read_dir;
use std::path::Path;
use std;

#[derive(Debug, PartialEq)]
pub enum Task {
    LIST, // zipcs -l/--list
    UNZIP, // Extract files from archive with full paths
}
impl Default for Task {
    fn default() -> Task {
        Task::UNZIP
    }
}

#[derive(Debug, Default)]
pub struct Zips {
    pub charset: CharSet, //zip -cs/--charset   //utf-8
    pub outdir: String, //zipcs -o/--outdir   //./
    pub zips: Vec<String>, //zipcs ZipArchive0 ZipArchive1 ...
    pub task: Task, // UNZIP
}
impl Zips {
    pub fn check_fix(&mut self) -> Result<(), String> {
        let name = "ZipArchives";
        for zip in &self.zips {
            let path = Path::new(&zip);
            if !path.exists() {
                return Err(format!("Arguments({}): \"{:?}\" is not exists", name, path));
            } else if path.is_dir() {
                return Err(format!(
                    "Arguments({}): \"{:?}\" is a directory",
                    name,
                    path
                ));
            }
            File::open(path).map_err(|e| {
                format!(
                    "Arguments({}): \"{:?}\" is invalid({})",
                    name,
                    path,
                    e.description()
                )
            })?;
        }
        Ok(())
    }
    pub fn call(self) -> Result<(), String> {
        dbln!("Config_zip: {:?}", self);

        for zip_arch_path in self.zips() {
            if let Err(e) = for_zip_arch_file(zip_arch_path, &self) {
                return Err(format!("{:?} -> {:?}", zip_arch_path, e));
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

fn for_zip_arch_file(zip_arch_path: &str, config: &Zips) -> Result<(), ZipCSError> {
    let zip_arch_path_ = Path::new(zip_arch_path);
    let zip_arch = File::open(zip_arch_path)?;
    let reader = BufReader::new(zip_arch);
    let mut zip_arch = ZipArchive::new(reader)?;

    // LIST
    if *config.task() == Task::LIST {
        for i in 0..zip_arch.len() {
            let mut file = match zip_arch.by_index(i) {
                Ok(o) => o,
                Err(e) => {
                    errln!("{}_Error: {:?}${:?} ->{:?}", NAME, zip_arch_path, i, e);
                    continue;
                }
            };
            let name = {
                if let Ok(o) = config.charset().decode(file.name_raw()) {
                    o
                } else {
                    file.name().to_owned()
                }
            };
            if name.ends_with('/') {
                println!("${}-> {:?}", i, name);
            } else {
                println!("${}-> {:?}: {:?}", i, name, file.size());
            }
        }
        return Ok(());
    }

    // UNZIP
    // Get ouddir
    let outdir = if config.outdir.is_empty() {
        zip_arch_path_
            .file_stem()
            .ok_or("ZipArchive's stem name is None")?
            .to_os_string()
    } else {
        OsString::from(config.outdir())
    };

    // Check and create oudir
    let outdir_path = Path::new(&outdir);
    if outdir_path.exists() && outdir_path.is_dir() {
        if read_dir(&outdir_path)
            .map_err(|e| {
                format!(
                    "Reading OutDir({}) occurs error: {}",
                    outdir_path.display(),
                    e.description()
                )
            })?
            .count() != 0
        {
            Err(format!("OutDir({}) is not empty!", outdir_path.display()))?;
        }
    } else if outdir_path.exists() && !outdir_path.is_dir() {
        Err(format!("OutDir({}) is not a Dir!", outdir_path.display()))?;
    } else {
        create_dir_all(outdir_path)?;
    }

    for i in 0..zip_arch.len() {
        let mut file = match zip_arch.by_index(i) {
            Ok(o) => o,
            Err(e) => {
                errln!("{}_Error: {:?}${:?} ->{:?}", NAME, zip_arch_path, i, e);
                continue;
            }
        };
        // Get name
        let name = {
            if let Ok(o) = config.charset().decode(file.name_raw()) {
                o
            } else {
                file.name().to_owned()
            }
        };

        // Get outpath, use PathBuf.push() to concat
        let mut path = outdir_path.to_path_buf();
        path.push(&name);

        // create dir/file
        if name.ends_with('/') {
            println!("${}-> {:?}", i, path.as_path());
            create_dir_all(&path)?;
        } else {
            println!("${}-> {:?}: {:?}", i, path.as_path(), file.size());
            if let Some(p) = path.parent() {
                if !p.exists() {
                    create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&path)?;
            copy(&mut file, &mut outfile)?;
        }

        // Get/Set permissions
        #[allow(unused_must_use)]
        #[cfg(unix)]
        {
            if let Some(mode) = file.unix_mode() {
                set_permissions(&path, Permissions::from_mode(mode)).map_err(|e| {
                    eprintln!(
                        "fs::set_permissions({}, {:?}) occurs error: {}",
                        path.as_path().display(),
                        mode,
                        e.description()
                    )
                });
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
enum ZipCSError {
    IO(std::io::Error),
    ZIP(ZipError),
    Desc(String),
}

impl std::error::Error for ZipCSError {
    fn description(&self) -> &str {
        match *self {
            ZipCSError::IO(ref e) => e.description(),
            ZipCSError::ZIP(ref e) => e.description(),
            ZipCSError::Desc(ref e) => e.as_str(),
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

impl From<String> for ZipCSError {
    fn from(e: String) -> Self {
        ZipCSError::Desc(e)
    }
}
impl<'a> From<&'a str> for ZipCSError {
    fn from(e: &str) -> Self {
        ZipCSError::Desc(e.to_owned())
    }
}
impl From<ZipError> for ZipCSError {
    fn from(e: ZipError) -> Self {
        ZipCSError::ZIP(e)
    }
}
