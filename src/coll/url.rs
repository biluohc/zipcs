use urlparse::{quote, quote_plus, unquote, unquote_plus};


#[derive(Debug, Default)]
pub struct Urls {
    pub is_plus: bool,
    pub is_encode: bool,
    pub strs: Vec<String>,
}

impl Urls {
    pub fn call(self) {
        for str in &self.strs {
            let rest = match (self.is_encode, self.is_plus) {
                (true, true) => quote_plus(str, b""),
                (true, false) => quote(str, b""),
                (false, true) => unquote_plus(str),
                (false, false) => unquote(str),
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
