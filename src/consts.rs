// crate's info
pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &'static str = "Wspsxing";
pub const EMAIL: &'static str = "biluohc@qq.com";
pub const DESC: &'static str = env!("CARGO_PKG_DESCRIPTION");

pub const URL_NAME: &'static str = "Repo";
pub const URL: &'static str = "https://github.com/biluohc/zipcs";

// copy form use@line16 ,but BIG5_2003 is swap by big5.
pub const CHARSETS: &'static str = "UTF_8, UTF_16BE, UTF_16LE, GBK, GB18030, HZ, BIG5...";

pub fn space_fix(msg: &str, msg_len_max: usize) -> String {
    let mut str = msg.to_owned();
    while str.len() < msg_len_max {
        str += " ";
    }
    str
}

// https://docs.rs/encoding/0.2.33/encoding/all/index.html
use encoding::all::*;
use encoding::{Encoding, DecoderTrap, EncoderTrap};
use std::default::Default;
use std::borrow::Cow;

impl Default for CharSet {
    fn default() -> CharSet {
        CharSet::UTF_8
    }
}

macro_rules! enum_create {
     ($($key: ident => $val: expr),+) => (
            #[allow(non_camel_case_types)]
            #[derive(Debug,Clone,PartialEq)]
            pub enum CharSet {
            $($key),+
            }
            impl CharSet {
                pub fn new(name :&str)->Result<Self,()> {
                    match name {
                        $($val => Ok(CharSet::$key)),+
                        , _=> Err(()),
                    }
                }
                pub fn decode(&self, bytes: &[u8]) -> Result<String, Cow<'static, str>> {
                    match *self {
                        $(CharSet::$key => ($key).decode(bytes, DecoderTrap::Strict)),+ ,
                    }
                }
                pub fn encode(&self, str: &str) -> Result<Vec<u8>, Cow<'static, str>> {
                    match *self {
                        $(CharSet::$key => ($key).encode(str, EncoderTrap::Strict)),+ ,
                    }
                }
                pub fn show() {
                    // println!("{:<16} => {}", "Input_String", "CharSet");
                    $( println!("{:<16} => {}", $val, stringify!($key)) ); + 
                }
            }
    );
}

enum_create!(
UTF_8 => "utf8",
UTF_16BE => "utf16be",
UTF_16LE => "utf16le",
GBK => "gbk",
GB18030 => "gb18030",
HZ => "hz",
BIG5_2003 => "big5",
// ERROR => "error",    
ASCII => "ascii",
IBM866 => "ibm866",
EUC_JP => "euc_jp",
ISO_2022_JP => "iso_2022_jp",
ISO_8859_1 => "iso_8859_1",
ISO_8859_2 => "iso_8859_2",
ISO_8859_3 => "iso_8859_3",
ISO_8859_4 => "iso_8859_4",
ISO_8859_5 => "iso_8859_5",
ISO_8859_6 => "iso_8859_6",
ISO_8859_7 => "iso_8859_7",
ISO_8859_8 => "iso_8859_8",
ISO_8859_10 => "iso_8859_10",
ISO_8859_13 => "iso_8859_13",
ISO_8859_14 => "iso_8859_14",
ISO_8859_15 => "iso_8859_15",
ISO_8859_16 => "iso_8859_16",
KOI8_R => "koi8_r",
KOI8_U => "koi8_u",
MAC_CYRILLIC => "mac_cyrillic",
MAC_ROMAN => "mac_roman",
WINDOWS_874 => "windows_874",
WINDOWS_949 => "windows_949",
WINDOWS_1250 => "windows_1250",
WINDOWS_1251 => "windows_1251",
WINDOWS_1252 => "windows_1252",
WINDOWS_1253 => "windows_1253",
WINDOWS_1254 => "windows_1254",
WINDOWS_1255 => "windows_1255",
WINDOWS_1256 => "windows_1256",
WINDOWS_1257 => "windows_1257",
WINDOWS_1258 => "windows_1258",
WINDOWS_31J => "windows_31j"
);

// encoding/all/index.html
// Get above map
// for l in s.lines() {
//     println!("{} => {:?},",l.trim(),l.trim().to_lowercase());
// }
