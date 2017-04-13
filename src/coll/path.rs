use stderr::Loger;

use super::consts::*;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::process::exit;
use std::error::Error;
use std::borrow::Cow;
use std::sync::Arc;
use std::fs::rename;
use std::u32;

#[derive(Debug)]
pub struct Paths {
    pub depth: u32,
    pub store: bool,
    pub link: bool,
    pub strs: Vec<String>,
    pub charset: CharSet,
}
impl Paths {
    pub fn check_fix(&mut self) -> Result<(), String> {
        for str in &self.strs {
            if !Path::new(str).exists() {
                return Err(format!("Path isn't exits {:?}", str));
            }
        }
        Ok(())
    }
    pub fn call(self) {
        dbstln!("Config_path: {:?}", self);
        let config = Arc::from(self);
        let depth = config.depth as i64;
        for str in &config.strs {
            if let Err(e) = path_recurse(Path::new(str).to_owned().into_os_string(),
                                         depth,
                                         config.clone()) {
                errln!("{}", e);
                exit(1);
            }
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Paths {
            depth: u32::MAX,
            store: false,
            link: false,
            strs: Vec::new(),
            charset: CharSet::default(),
        }
    }
}

fn path_recurse(path: OsString, mut depth: i64, config: Arc<Paths>) -> Result<(), String> {
    let path_decode_result = decode(&path, &config.charset);
    if config.charset != CharSet::UTF_8 && path_decode_result.is_ok() {
        let str = path_decode_result.unwrap();
        let ne = ne(&str, &path);
        if config.store && ne {
            rename(&path, &str)
                .map_err(|e| format!("{:?} rename fails: {}", path, e.description()))?;
            println!("{:?} -> {:?}", path, str);
        } else {
            println!("{:?} : {:?}", path, str);
        }
    } else {
        println!("{:?}", path);
    }

    // -d/--depth
    let path = PathBuf::from(path);
    if !path.as_path().is_dir() || depth < 1 {
        return Ok(());
    }

    // -l/--link
    if !config.link {
        let metadata = path.as_path()
            .symlink_metadata()
            .map_err(|e| format!("{:?} read without symlink fails: {}", path, e))?;
        if !metadata.is_dir() {
            return Ok(());
        }
    }
    depth -= 1;

    for entry in path.as_path()
            .read_dir()
            .map_err(|e| format!("{:?} read fails: {}", path, e.description()))? {
        let entry = entry
            .map_err(|ref e| format!("{:?}'s entry read fails: {}", path, e.description()))?;
        dbstln!("{:?}", entry.path());
        path_recurse(entry.path().into_os_string(), depth, config.clone())?;
    }
    Ok(())
}

#[cfg(unix)]
fn decode(path: &OsString, cs: &CharSet) -> Result<String, Cow<'static, str>> {
    cs.decode(path.as_bytes())
}
#[cfg(windows)]
fn decode(path: &OsString, cs: &CharSet) -> Result<String, Cow<'static, str>> {
    cs.decode(path.to_string_lossy().as_bytes())
}

#[cfg(unix)]
fn ne(str: &str, path: &OsString) -> bool {
    str.as_bytes() != path.as_bytes()
}

#[cfg(windows)]
fn ne(str: &str, path: &OsString) -> bool {
    str.as_bytes() != path.to_string_lossy().as_bytes()
}