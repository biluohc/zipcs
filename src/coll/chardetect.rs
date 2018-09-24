use chardet::detect;
use rayon::prelude::*;

use super::consts::space_fix;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

#[derive(Debug, Default)]
pub struct CharDet {
    pub files: Vec<String>,
}

impl CharDet {
    pub fn call(self) {
        debug!("{:?}", self);
        let max_len = self.files.as_slice().iter().max_by_key(|p| p.len()).unwrap().len();
        // println!("{}{:3}CharSet{:13}Rate{:8}Info", space_fix("File",max_len), "", "", "");
        self.files.par_iter().for_each(|file| match chardet(file) {
            Ok(o) => {
                let (mut charset, rate, info) = o;
                // "WINDOWS_1258".len() = 12 -> 12+6 = 18
                if charset.is_empty() {
                    charset = "Binary".to_owned();
                }
                println!(
                    "{}: {}  {:.4}{:6}{}",
                    space_fix(file, max_len),
                    space_fix(&charset, 18),
                    rate,
                    "",
                    info
                );
            }
            Err(e) => eprintln!("{}: {:?}", space_fix(file, max_len), e),
        })
    }
    pub fn check(&self) -> Result<(), String> {
        for path in &self.files {
            let path = Path::new(path);
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

fn chardet(f: &str) -> io::Result<(String, f32, String)> {
    let mut file = BufReader::new(File::open(f)?);
    let mut bytes = Vec::default();
    file.read_to_end(&mut bytes)?;
    Ok(detect(bytes.as_slice()))
}
