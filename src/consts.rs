// crate's info
pub const NAME: &'static str = "Zipcs";
pub const VERSION: &'static str = "0.2.2";

pub const AUTHOR: &'static str = "Wspsxing";
pub const EMAIL: &'static str = "biluohc@qq.com";
pub const ABOUT: &'static str = "Unzip with charset setting.";
pub const URL_NAME: &'static str = "Repository";
pub const URL: &'static str = "https://github.com/biluohc/zipcs";

// copy form use@line16 ,but BIG5_2003 is swap by big5.
pub const CHARSETS: &'static str = "UTF_8, UTF_16BE, UTF_16LE, GBK, GB18030, HZ, BIG5";

// charset.downcase() to handle
// https://docs.rs/encoding/0.2.33/encoding/all/index.html
use encoding::all::{UTF_8, UTF_16BE, UTF_16LE, GBK, GB18030, HZ, BIG5_2003};
use encoding::{Encoding, DecoderTrap};
use std::default::Default;
use std::borrow::Cow;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CharSet {
    UTF_8,
    UTF_16BE,
    UTF_16LE,
    GBK,
    GB18030,
    HZ,
    BIG5_2003,
}

impl CharSet {
    pub fn new(name: &str) -> Result<Self, ()> {
        let cs = match name {
            "utf8" => CharSet::UTF_8,
            "utf16be" => CharSet::UTF_16BE,
            "utf16le" => CharSet::UTF_16LE,
            "gbk" => CharSet::GBK,
            "gb18030" => CharSet::GB18030,
            "hz" => CharSet::HZ,
            "big5" => CharSet::BIG5_2003,
            _ => return Err(()),
        };
        Ok(cs)
    }
    pub fn u8slice_to_string(&self, u8slice: &[u8]) -> Result<String, Cow<'static, str>> {
        match *self {
            CharSet::UTF_8 => UTF_8.decode(u8slice, DecoderTrap::Strict),
            CharSet::UTF_16BE => UTF_16BE.decode(u8slice, DecoderTrap::Strict),
            CharSet::UTF_16LE => UTF_16LE.decode(u8slice, DecoderTrap::Strict),
            CharSet::GBK => GBK.decode(u8slice, DecoderTrap::Strict),
            CharSet::GB18030 => GB18030.decode(u8slice, DecoderTrap::Strict),
            CharSet::HZ => HZ.decode(u8slice, DecoderTrap::Strict),
            CharSet::BIG5_2003 => BIG5_2003.decode(u8slice, DecoderTrap::Strict),
        }
    }
}

impl Default for CharSet {
    fn default() -> CharSet {
        CharSet::UTF_8
    }
}