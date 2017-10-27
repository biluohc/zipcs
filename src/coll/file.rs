use super::consts::*;
use std::io::{Read, Write};
use std::process::exit;
use std::path::Path;
use std::fs::File;

#[derive(Debug, Default)]
pub struct Files {
    pub store: bool,
    pub strs: Vec<String>,
    pub charset: CharSet,
    pub charset_out: CharSet,
}

impl Files {
    pub fn check_fix(&mut self) -> Result<(), String> {
        for str in &self.strs {
            if !Path::new(str).is_file() {
                return Err(format!("File isn't exits {:?}", str));
            }
        }
        Ok(())
    }
    pub fn call(self) {
        dbstln!("Config_file_: {:?}", self);

        for str in &self.strs {
            if let Err(e) = file_handle(str, &self) {
                errln!("{}", e);
                exit(1);
            }
        }
    }
}

fn file_handle(file_name: &str, config: &Files) -> Result<(), String> {
    let mut file = File::open(file_name).map_err(|e| {
        format!("{:?} open fails: {}", file_name, e)
    })?;
    let mut bytes = Vec::new();
    let _ = file.read_to_end(&mut bytes).map_err(|e| {
        format!("{:?} read fails: {}", file_name, e)
    })?;
    let read_result = config.charset.decode(&bytes[..]);
    let str = {
        if config.charset != CharSet::UTF_8 && read_result.is_ok() {
            read_result.unwrap()
        } else {
            String::from_utf8_lossy(&bytes[..]).into_owned()
        }
    };
    if config.charset != config.charset_out {
        if let Ok(bs) = config.charset_out.encode(&str) {
            if config.store {
                let mut file = File::create(file_name).map_err(|e| {
                    format!("{:?} create fails: {}", file_name, e)
                })?;
                file.write_all(&bs[..]).map_err(|e| {
                    format!("{:?} write fails: {}", file_name, e)
                })?;
                file.flush().map_err(|e| {
                    format!("{:?} flush fails: {}", file_name, e)
                })?;
                println!("{:?} rewrite success", file_name);
            } else {
                println!("{:?}: \n{}\n", file_name, String::from_utf8_lossy(&bs[..]));
            }
        } else {
            return Err(format!("{:?} encode fails", file_name));
        }
    } else {
        println!("{:?}: \n{}\n", file_name, str);
    }
    Ok(())
}
