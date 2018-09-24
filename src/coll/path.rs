use super::consts::*;

use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::rename;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf};
use std::process::exit;

#[derive(Debug)]
pub struct Paths {
    pub depth: Option<usize>,
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
        debug!("Config_path: {:?}", self);
        let depth = self.depth;
        for str in &self.strs {
            if let Err(e) = path_recurse(PathBuf::from(str), depth, &self) {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Paths {
            depth: None,
            store: false,
            link: false,
            strs: Vec::new(),
            charset: CharSet::default(),
        }
    }
}

fn path_recurse(mut path: PathBuf, mut depth: Option<usize>, config: &Paths) -> Result<(), String> {
    let components_last = path.components().last().unwrap().as_os_str().to_os_string();
    let components_last = Component::Normal(components_last.as_os_str());
    match components_last {
        Component::Normal(os_str) => match decode(os_str, &config.charset) {
            Ok(file_name) => {
                let mut path_new = path.clone();
                assert!(path_new.pop());
                path_new.push(&file_name);
                println!("{:?}", path_new);
                if config.store && ne(&file_name, os_str) {
                    rename(&path, &path_new).map_err(|e| format!("rename fails: {}: {:?}", e.description(), path))?;
                    path = path_new;
                }
            }
            Err(_) => {
                eprintln!("decode failed by {:?}: {:?} ", config.charset, path);
            }
        },
        _ => {
            println!("{:?}", path);
        }
    }

    // -d/--depth
    if !path.as_path().is_dir() || depth.as_ref() == Some(&0) {
        return Ok(());
    }

    // -l/--link
    if !config.link {
        let metadata = path
            .as_path()
            .symlink_metadata()
            .map_err(|e| format!("{:?} read without symlink fails: {}", path, e))?;
        if !metadata.is_dir() {
            return Ok(());
        }
    }
    depth = depth.map(|d| d - 1);

    for entry in path
        .as_path()
        .read_dir()
        .map_err(|e| format!("{:?} read fails: {}", path, e.description()))?
    {
        let entry = entry.map_err(|ref e| format!("{:?}'s entry read fails: {}", path, e.description()))?;
        debug!("{:?}", entry.path());
        path_recurse(entry.path(), depth, config)?;
    }
    Ok(())
}

#[cfg(unix)]
fn decode(path: &OsStr, cs: &CharSet) -> Result<String, Cow<'static, str>> {
    cs.decode(path.as_bytes())
}
#[cfg(windows)]
fn decode(path: &OsStr, cs: &CharSet) -> Result<String, Cow<'static, str>> {
    cs.decode(path.to_string_lossy().as_bytes())
}

// no-equal
#[cfg(unix)]
fn ne(str: &str, path: &OsStr) -> bool {
    str.as_bytes() != path.as_bytes()
}

#[cfg(windows)]
fn ne(str: &str, path: &OsStr) -> bool {
    str.as_bytes() != path.to_string_lossy().as_bytes()
}
