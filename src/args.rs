use super::consts::*;
use coll::*;

use app::{App, Cmd, Opt, Args, args, OptValue, OptValueParse, OptTypo};

#[derive(Debug,Default)]
pub struct Config {
    zip: Zips,
    ping: Pings,
    url: Urls,
    path: Paths,
    file: Files,
    chardet: CharDet,
}

impl Config {
    pub fn parse() {
        let mut config = Self::default();
        let mut list = false;
        let charsets = format!("Sets the charset Zipcs using({})\nYou can see all CharSet by `zipcs charset`",
                               CHARSETS.replace("_", "").to_lowercase());
        let helper = {
            App::new(NAME)
                .version(VERSION)
                .author(AUTHOR, EMAIL)
                .addr(URL_NAME, URL)
                .desc(DESC)
                .cmd(Cmd::new("zip")
                         .short("z")
                         .sort_key("1")
                         .desc("Unzip with charset setting")
                         .opt(Opt::new("list", &mut list)
                                  .short('l')
                                  .long("list")
                                  .help("Only list files from ZipArchives"))
                         .opt(Opt::new("charset", &mut config.zip.charset)
                                  .short('c')
                                  .long("charset")
                                  .help(&charsets))
                         .opt(Opt::new("outdir", &mut config.zip.outdir)
                                  .short('o')
                                  .long("outdir")
                                  .help("Sets Output directory"))
                         .args(Args::new("ZipArchive", &mut config.zip.zips).help("ZipArchive need to unzip")))
                .cmd(Cmd::new("path")
                         .short("P")
                         .sort_key("2")
                         .desc("Paths decoding with charset setting")
                         .opt(Opt::new("charset", &mut config.path.charset)
                                  .short('c')
                                  .long("charset")
                                  .help(&charsets))
                         .opt(Opt::new("depth", &mut config.path.depth)
                                  .optional()
                                  .short('d')
                                  .long("depth")
                                  .help("decode paths recursively depth(default without limit)"))
                         .opt(Opt::new("store", &mut config.path.store)
                                  .short('s')
                                  .long("store")
                                  .help("store result by rename"))
                         .opt(Opt::new("link", &mut config.path.link)
                                  .short('l')
                                  .long("link")
                                  .help("follow symbolic links"))
                         .args(Args::new("Path", &mut config.path.strs).help("Path need to decode")))
                .cmd(Cmd::new("file")
                         .short("f")
                         .sort_key("3")
                         .desc("Files encoding/decoding with charset setting")
                         .opt(Opt::new("charset", &mut config.file.charset)
                                  .short('c')
                                  .long("charset")
                                  .help(&charsets))
                         .opt(Opt::new("charset_out", &mut config.file.charset_out)
                                  .short('C')
                                  .long("charset-out")
                                  .help("charset output(encode) using"))
                         .opt(Opt::new("store", &mut config.file.store)
                                  .short('s')
                                  .long("store")
                                  .help("store result by rewrite"))
                         .args(Args::new("File", &mut config.file.strs).help("File need to encode/decode")))
                .cmd(Cmd::new("ping")
                         .short("p")
                         .sort_key("4")
                         .desc("ping domains/ips")
                         .opt(Opt::new("count", &mut config.ping.count)
                                  .short('c')
                                  .long("count")
                                  .help("stop after sending count ECHO_REQUEST packets"))
                         .opt(Opt::new("_6", &mut config.ping._6)
                                  .short('6')
                                  .help("use IPV6"))
                         .opt(Opt::new("only-line", &mut config.ping.only_line)
                                  .short('l')
                                  .long("only-line")
                                  .help("print result only-line"))
                         .args(Args::new("Host/IP", &mut config.ping.hosts).help("Host or IP need to ping")))
                .cmd(Cmd::new("chardet").short("c").sort_key("5").desc("Detect the charset for File(for reference)")
                .args(args(
                    "File",
                    &mut config.chardet.files,
                    "The file need to detect charset",
                )))
                .cmd(Cmd::new("charset").short("C").sort_key("50").desc("Show all CharSet supported")
                 )
                .cmd(Cmd::new("ip")
                         .short("i")
                         .sort_key("6")
                         .desc("Get ip address"))
                .cmd(Cmd::new("url")
                         .short("u")
                         .sort_key("7")
                         .desc("Urls decoding/encoding")
                         .opt(Opt::new("encode", &mut config.url.is_encode)
                                  .short('e')
                                  .long("encode")
                                  .help("encode(default is decode)"))
                         .opt(Opt::new("plus", &mut config.url.is_plus)
                                  .short('p')
                                  .long("plus")
                                  .help("replaces ' ' with '+'"))
                         .args(Args::new("Url", &mut config.url.strs).help("Url need to decode/encode")))
                .parse_args()
        };
        if list {
            config.zip.task = Task::LIST;
        }
        if *helper.args_len() == 0 {
            helper.help_exit(0);
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
            Some("chardet")=> {
                self.chardet.check()?;
                 self.chardet.call();
            }
            Some("charset") => {
                CharSet::show();
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
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
            match CharSet::new(msg) {
                Err(_) => {
                    Err(format!(
                        "OPTION(<{}>) parse<CharSet> fails: \"{}\"",
                        opt_name,
                        msg
                    ))?;
                }
                Ok(o) => **self = o,}
        } else if typo.is_single() {
            Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name,
                msg
            ))?;
        }
        Ok(())
    }
    /// env::arg could is `""`
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 &&self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}
