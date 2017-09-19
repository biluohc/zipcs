use rayon::prelude::*;
use chardet::detect;

use super::consts::space_fix;
use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

#[derive(Debug, Default)]
pub struct CharDet {
    pub files: Vec<String>,
}

impl CharDet {
    pub fn call(self) {
        dbln!("{:?}",self);
        let max_len= self.files.as_slice().iter().max_by_key(|p|p.len()).unwrap().len();
        // println!("{}{:3}CharSet{:13}Rate{:8}Info", space_fix("File",max_len), "", "", "");
        self.files.par_iter()
         .for_each(|file| 
         match chardet(&file) {
             Ok(o)=> {
                 let (charset, rate, info) = o ; 
                 // "WINDOWS_1258".len() = 12 -> 12+6 = 18
                 println!("{}: {}  {:.4}{:6}{}", space_fix(file,max_len), space_fix(&charset, 18), rate, "", info);
             }
             Err(e)=> {
                 errln!("{}: {:?}", space_fix(file,max_len), e)
             }
         }
          )
    }
    pub fn check(&self) -> Result<(), String> {
        for path in &self.files {
            let path=Path::new(path);
            if !path.exists() {
                return Err(format!("Args(File): {:?} is not exists", path));
            }
            if !path.is_file() {
                return Err(format!("Args(File): {:?} is not a file", path));
            }
        }
        Ok(())
    }
}

fn chardet(f :&str)->io::Result<(String, f32, String)> {
    let mut file = File::open(f)?;
    let mut bytes = Vec::default();
    let mut buf = [0u8];
    loop {
        let len = file.read(&mut buf)?;
        if len != 0 {
            bytes.push(buf[0]);
        } else {
            break;
        }
    }
    Ok(detect(bytes.as_slice()))
}