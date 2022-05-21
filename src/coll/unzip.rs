use super::consts::*;

use chardet::{charset2encoding, detect};
use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;
use filetime::{set_symlink_file_times, FileTime};
use zip::read::ZipArchive;
use zip::result::ZipError;
// https://docs.rs/filetime/ not follow symlink?

use std;
use std::error::Error;
use std::ffi::OsString;
use std::fs::read_dir;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Task {
    Chardet, // Detect the charset for File's name from ZipArchive
    List,    // zipcs -l/--list
    Unzip,   // Extract files from archive with full paths
}
impl Default for Task {
    fn default() -> Task {
        Task::Unzip
    }
}

#[derive(Debug, Default)]
pub struct Zips {
    pub charset: CharSet,         //zip -cs/--charset   //utf-8
    pub outdir: String,           //zipcs -o/--outdir   //./
    pub password: Option<String>, //zipcs -p/--password
    pub zips: Vec<String>,        //zipcs ZipArchive0 ZipArchive1 ...
    pub task: Task,               // UNZIP
}
impl Zips {
    pub fn check_fix(&mut self) -> Result<(), String> {
        let name = "ZipArchives";
        for zip in &self.zips {
            let path = Path::new(&zip);
            if !path.exists() {
                return Err(format!("Arguments({}): \"{:?}\" is not exists", name, path));
            } else if path.is_dir() {
                return Err(format!("Arguments({}): \"{:?}\" is a directory", name, path));
            }
            File::open(path).map_err(|e| format!("Arguments({}): \"{:?}\" is invalid({})", name, path, e))?;
        }
        Ok(())
    }
    pub fn call(self) {
        debug!("Config_zip: {:?}", self);

        for zip_arch_path in self.zips() {
            if let Err(e) = for_zip_arch_file(zip_arch_path, &self) {
                error!("{:?} -> {}", zip_arch_path, e);
            }
        }
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
    pub fn password(&self) -> Option<&str> {
        self.password.as_ref().map(|s| s.as_str())
    }
    pub fn task(&self) -> &Task {
        &self.task
    }
}

fn for_zip_arch_file(zip_arch_path: &str, config: &Zips) -> Result<(), ZipCSError> {
    let zip_arch_path_ = Path::new(zip_arch_path);
    let zip_arch = File::open(zip_arch_path)?;
    // Use BufReader read encrypt zip has error: corrupt deflate stream: https://github.com/zip-rs/zip/issues/280
    // let reader = BufReader::new(zip_arch);
    let mut zip_arch = ZipArchive::new(zip_arch)?;

    // LIST
    if *config.task() == Task::List {
        for i in 0..zip_arch.len() {
            let file = match zip_arch.by_index_raw(i) {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("{}_Error: {:?}${:?} ->{:?}", NAME, zip_arch_path, i, e);
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

    // Chardet
    if *config.task() == Task::Chardet {
        for i in 0..zip_arch.len() {
            let file = match zip_arch.by_index_raw(i) {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("{}_Error: {:?}${:?} ->{:?}", NAME, zip_arch_path, i, e);
                    continue;
                }
            };
            let charset = detect(file.name_raw());
            let name = encoding_from_whatwg_label(charset2encoding(&charset.0))
                .and_then(|enc| enc.decode(file.name_raw(), DecoderTrap::Strict).ok())
                .unwrap_or_else(|| file.name().to_owned());
            if name.ends_with('/') {
                println!("{} ${}-> {:?}", charset.0, i, name);
            } else {
                println!("{} ${}-> {:?}: {:?}", charset.0, i, name, file.size());
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
        let dir_item =
            read_dir(&outdir_path).map_err(|e| format!("Reading OutDir({}) occurs error: {}", outdir_path.display(), e))?;
        if dir_item.count() != 0 {
            return Err(format!("OutDir({}) is not empty!", outdir_path.display()).into());
        }
    } else if outdir_path.exists() && !outdir_path.is_dir() {
        return Err(format!("OutDir({}) is not a Dir!", outdir_path.display()).into());
    } else {
        create_dir_all(outdir_path)?;
    }

    macro_rules!  zip_arch_by_index {
        ($i: ident) => {
            if let Some(pw) = config.password() {
                match zip_arch.by_index_decrypt($i, pw.as_bytes()) {
                    Ok(Ok(o)) => Ok(o),
                    Err(e) => Err(e),
                    Ok(Err(e)) => return Err(ZipCSError::Desc(e.to_string())),
                }
            } else {
                zip_arch.by_index($i)
            }
        };
    }

    for i in 0..zip_arch.len() {
        let mut file = match zip_arch_by_index!(i) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{}_Error: {:?}${:?} ->{:?}", NAME, zip_arch_path, i, e);
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

        let path_display = path.as_path().display();
        // Get/Set m/atime
        match file.last_modified().to_time() {
            Err(e) => error!("{} last_modified().to_time() failed: {}", path_display, e),
            Ok(tm) => {
                let tm = FileTime::from_unix_time(tm.unix_timestamp(), tm.nanosecond());
                set_symlink_file_times(&path, tm, tm)
                    .map_err(|e| eprintln!("filetime::set_symlink_file_times({}, {:?}) failed: {}", path_display, tm, e))
                    .ok();
            }
        }

        // Get/Set permissions
        #[cfg(unix)]
        {
            use std::fs::{set_permissions, Permissions};
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                set_permissions(&path, Permissions::from_mode(mode))
                    .map_err(|e| eprintln!("fs::set_permissions({}, {:?}) occurs error: {}", path_display, mode, e))
                    .ok();
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
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ZipCSError::IO(e) => Some(e),
            ZipCSError::ZIP(ref e) => Some(e),
            _ => None,
        }
    }
}

use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for ZipCSError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZipCSError::IO(e) => e.fmt(f),
            ZipCSError::ZIP(e) => e.fmt(f),
            ZipCSError::Desc(e) => e.fmt(f),
        }
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
