use percent_encoding::{percent_decode, percent_encode_byte, utf8_percent_encode, DEFAULT_ENCODE_SET};

#[derive(Debug, Default)]
pub struct Urls {
    pub encode_all_chars: bool,
    pub is_encode: bool,
    pub strs: Vec<String>,
}

impl Urls {
    pub fn call(self) {
        for str in &self.strs {
            let rest = match (self.is_encode, self.encode_all_chars) {
                (true, true) => Ok(encode_chars(str)),
                (true, false) => Ok(utf8_percent_encode(str, DEFAULT_ENCODE_SET).to_string()),
                _ => percent_decode(str.as_bytes()).decode_utf8().map(|s| s.into_owned()),
            };
            match rest {
                Ok(o) => {
                    println!("{}", o);
                }
                Err(o) => {
                    eprintln!("{:?}", o);
                }
            }
        }
    }
}

fn encode_chars(str: &str) -> String {
    let mut tmp = str
        .split('/')
        .filter(|c| !c.is_empty())
        .flat_map(|c| "/".chars().chain(c.bytes().flat_map(|cc| percent_encode_byte(cc).chars())))
        .collect::<String>();
    if !str.starts_with('/') {
        tmp.remove(0);
    }
    tmp
}
