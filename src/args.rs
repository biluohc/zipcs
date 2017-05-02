use super::consts::*;
use coll::*;

use app::{App, Cmd, Opt, OptValue, OptValueParse};
use stderr::Loger;

#[derive(Debug,Default)]
pub struct Config {
    zip: Zips,
    ping: Pings,
    url: Urls,
    path: Paths,
    file: Files,
}

impl Config {
    pub fn parse() {
        let mut config = Self::default();
        let mut list = false;
        let mut log = Some(String::new());
        let charsets = format!("Sets the charset Zipcs using({})",
                               CHARSETS.replace("_", "").to_lowercase());
        let helper = {
            App::new(NAME)
                .version(VERSION)
                .author(AUTHOR, EMAIL)
                .addr(URL_NAME, URL)
                .desc(DESC)
                .cmd(Cmd::new("zip")
                         .desc("Unzip with charset setting")
                         .opt(Opt::new("list", &mut list)
                                  .short("l")
                                  .long("list")
                                  .help("Only list files from ZipArchives"))
                         .opt(Opt::new("module_path", &mut log)
                                  .long("log")
                                  .short("log")
                                  .help("Print log for debug"))
                         .opt(Opt::new("charset", &mut config.zip.charset)
                                  .short("cs")
                                  .long("charset")
                                  .help(&charsets))
                         .opt(Opt::new("outdir", &mut config.zip.outdir)
                                  .short("o")
                                  .long("outdir")
                                  .help("Sets Output directory"))
                         .args("ZipArchives", &mut config.zip.zips)
                         .args_help("ZipArchives need to unzip"))
                .cmd(Cmd::new("ping")
                         .desc("ping domains/ips")
                         .opt(Opt::new("count", &mut config.ping.count)
                                  .short("c")
                                  .long("count")
                                  .help("stop after sending count ECHO_REQUEST packets"))
                         .opt(Opt::new("_6", &mut config.ping._6)
                                  .short("6")
                                  .help("use IPV6"))
                         .opt(Opt::new("only-line", &mut config.ping.only_line)
                                  .short("l")
                                  .long("only-line")
                                  .help("print result only-line"))
                         .args("Hosts/IPs", &mut config.ping.hosts)
                         .args_help("Hosts or IPs need to ping"))
                .cmd(Cmd::new("url")
                         .desc("Urls decoding/encoding")
                         .opt(Opt::new("encode", &mut config.url.is_encode)
                                  .short("e")
                                  .long("encode")
                                  .help("encode(default is decode)"))
                         .opt(Opt::new("plus", &mut config.url.is_plus)
                                  .short("p")
                                  .long("plus")
                                  .help("replaces ' ' with '+'"))
                         .args("Urls", &mut config.url.strs)
                         .args_help("Urls need to decode/encode"))
                .cmd(Cmd::new("path")
                         .desc("Path decoding with charset setting")
                         .opt(Opt::new("charset", &mut config.path.charset)
                                  .short("cs")
                                  .long("charset")
                                  .help(&charsets))
                         .opt(Opt::new("depth", &mut config.path.depth)
                                  .short("d")
                                  .long("depth")
                                  .help("decode paths recursively depth"))
                         .opt(Opt::new("store", &mut config.path.store)
                                  .short("s")
                                  .long("store")
                                  .help("store result by rename"))
                         .opt(Opt::new("link", &mut config.path.link)
                                  .short("l")
                                  .long("link")
                                  .help("follow symbolic links"))
                         .args("Paths", &mut config.path.strs)
                         .args_help("Paths need to decode"))
                .cmd(Cmd::new("file")
                         .desc("File encoding/decoding with charset setting")
                         .opt(Opt::new("charset", &mut config.file.charset)
                                  .short("cs")
                                  .long("charset")
                                  .help(&charsets))
                         .opt(Opt::new("charset_out", &mut config.file.charset_out)
                                  .short("co")
                                  .long("charset-out")
                                  .help("charset output(encode) using"))
                         .opt(Opt::new("store", &mut config.file.store)
                                  .short("s")
                                  .long("store")
                                  .help("store result by rewrite"))
                         .args("Files", &mut config.file.strs)
                         .args_help("Files need to encode/decode"))
                .cmd(Cmd::new("ip").desc("Get ip address"))
                .parse_args()
        };
        if list {
            config.zip.task = Task::LIST;
        }
        if let Err(e) = config.check_fix_call(helper.current_cmd_str()) {
            helper.help_cmd_err_exit(helper.current_cmd_ref(), e, 1);
        }
    }
    fn check_fix_call(mut self, cmd: Option<&str>) -> Result<(), String> {
        dbstln!("Config: {:?}: {:?}", cmd, self);
        match cmd {
            Some("zip") => {
                self.zip.check_fix()?;
                self.zip.call()?;
            }
            Some("ping") => {
                self.ping.check_fix()?;
                self.ping.call();
            }
            Some("url") => {
                self.url.call();
            }
            Some("path") => {
                self.path.check_fix()?;
                self.path.call();
            }
            Some("file") => {
                self.file.check_fix()?;
                self.file.call();
            }
            Some("ip") => {
                call();
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

/// Custom `OptValue` by impl `OptValueParse`
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut CharSet {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Some("utf8".to_owned())
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        match CharSet::new(msg) {
            Err(_) => return Err(format!("OPTION({}) parse<CharSet> fails: \"{}\"", opt_name, msg)),
            Ok(o) => **self = o,
        }
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}
