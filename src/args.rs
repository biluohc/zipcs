use super::consts::*;
use app::{App, Opt, Flag};
use super::Loger;

use std::fs::read_dir;
use std::path::Path;
use std::fs::File;

#[derive(Debug)]
pub struct Config {
    charset: CharSet, //zip -cs/--charset   //utf-8
    outdir: String, //zipcs -o/--outdir   //./
    zips: Vec<String>, //zipcs ZipArchive0 ZipArchive1 ...
    task: Task, // UNZIP
}

impl Config {
    pub fn get() -> Result<Config, String> {
        let charsets = format!("Sets the charset Zipcs using{}",
                               CHARSETS.replace("_", "-").to_lowercase());
        let app = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR, EMAIL)
        .address(URL_NAME, URL)
        .about(ABOUT)
        .flag(Flag::new("list")
            .short("l")
            .long("list")
            .help("Only list files from archives"))
        .opt(Opt::new("log")
            .long("log")
            .short("log")
            .help("Print log for debug"))
        .opt(Opt::new("charset").short("cs").long("charset").help(&charsets))
        .opt(Opt::new("outdir").short("o").long("outdir").help("Sets Output directory"))
        .args_name("ZipArchives")
        .args_help("Sets the ZipArchives to unzip")
        // .args_must(true)
        .get();
        app.to_config()
    }
    fn new(cs: CharSet, od: String, zips: Vec<String>, task: Task) -> Self {
        Config {
            charset: cs,
            outdir: od,
            zips: zips,
            task: task,
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
    pub fn task(&self) -> &Task {
        &self.task
    }
}

#[derive(Debug,PartialEq)]
pub enum Task {
    LIST, // zipcs -l/--list
    UNZIP, // Extract files from archive with full paths
}

trait ToConfig {
    fn to_config(&self) -> Result<Config, String>;
}
impl<'app> ToConfig for App<'app> {
    fn to_config(&self) -> Result<Config, String> {
        let task = match self.get_flag("list") {
            Some(..) => Task::LIST,
            None => Task::UNZIP,
        };

        let charset_raw = self.get_opt("charset").unwrap_or_else(|| "utf-8".to_owned()).to_lowercase();
        dbln!("{}->{}@ToConfig()-charset_raw: {:?}\n",
              module_path!(),
              line!(),
              charset_raw);
        let charset = {
            if let Ok(o) = CharSet::new(charset_raw.as_str()) {
                o
            } else {
                return Err(format!("dont support the charset {:?}", charset_raw));
            }
        };

        let mut outdir = self.get_opt("outdir").unwrap_or_else(|| "./".to_owned());

        if Path::new(&outdir).is_file() {
            return Err(format!("outdir {:?} is a file.", outdir));
        } else if Path::new(&outdir).exists() {
            if let Err(e) = read_dir(&outdir) {
                return Err(format!("outdir {:?} is invalid {:?}.", outdir, e));
            }
        }

        //dir + name =dirname
        if !outdir.ends_with('/') {
            outdir.push('/');
        }
        assert!(outdir.ends_with('/'));

        let zips: Vec<String> = self.get_args().unwrap().values().cloned().collect();
        zips_path_valid(&zips)?;

        Ok(Config::new(charset, outdir, zips, task))
    }
}

fn zips_path_valid(zips: &[String]) -> Result<(), String> {
    if zips.is_empty() {
        return Err("no input ZipArchives.".to_owned());
    }
    for zip in zips {
        let path = Path::new(zip);
        if !path.exists() {
            return Err(format!("{:?} is not exists", path));
        } else if path.is_dir() {
            return Err(format!("{:?} is a directory.", path));
        } else if File::open(path).is_err() {
            return Err(format!("{:?} is invalid.", path));
        }
    }
    Ok(())
}
