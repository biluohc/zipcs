use super::consts::*;
use app::{App, Opt, OptValue, OptValueParse};
use super::Loger;

use std::default::Default;
use std::fs::read_dir;
use std::path::Path;
use std::fs::File;

#[derive(Debug,Default)]
pub struct Config {
    charset: CharSet, //zip -cs/--charset   //utf-8
    outdir: String, //zipcs -o/--outdir   //./
    zips: Vec<String>, //zipcs ZipArchive0 ZipArchive1 ...
    task: Task, // UNZIP
}

impl Config {
    pub fn get() -> Result<Self, String> {
        init!();
        let mut config = Self::default();
        let mut list = false;
        let mut log =Some(String::new());
        config.outdir.push_str("./");
        {
            let charsets = format!("Sets the charset Zipcs using({})",
                                   CHARSETS.replace("_", "").to_lowercase());
            let mut app = App::new(NAME)
                .version(VERSION)
                .author(AUTHOR, EMAIL)
                .addr(URL_NAME, URL)
                .desc(ABOUT)
                .opt(Opt::new("list", &mut list)
                         .short("l")
                         .long("list")
                         .help("Only list files from ZipArchives"))
                .opt(Opt::new("module_path", &mut log)
                         .long("log")
                         .short("log")
                         .help("Print log for debug"))
                .opt(Opt::new("charset", &mut config.charset)
                         .short("cs")
                         .long("charset")
                         .help(&charsets))
                .opt(Opt::new("outdir", &mut config.outdir)
                         .short("o")
                         .long("outdir")
                         .help("Sets Output directory"))
                .args("ZipArchives", &mut config.zips);
            app.parse();
        }
        if list == true {
            config.task = Task::LIST;
        }
        config.check()
    }
    fn check(mut self) -> Result<Self, String> {
        if Path::new(&self.outdir).is_file() {
            return Err(format!("outdir {:?} is a file.", self.outdir));
        } else if Path::new(&self.outdir).exists() {
            if let Err(e) = read_dir(&self.outdir) {
                return Err(format!("outdir {:?} is invalid {:?}.", self.outdir, e));
            }
        }
        //dir + name =dirname-> dir/ + name
        if !self.outdir.ends_with('/') {
            self.outdir.push('/');
        }
        assert!(self.outdir.ends_with('/'));

        zips_path_valid(&self.zips)?;
        Ok(self)
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
impl Default for Task {
    fn default() -> Task {
        Task::UNZIP
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

/// Custom OptValue by impl OptValueParse
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut CharSet {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::from(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        match CharSet::new(msg) {
            Err(_) => return Err(format!("OPTION({}) parse<CharSet> fails: \"{}\"", opt_name, msg)),
            Ok(o) => **self = o,
        }
        Ok(())
    }
    fn check(&self, _ : &str) -> Result<(),String> {
        Ok(())
    }
}