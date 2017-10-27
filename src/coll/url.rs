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
            let rest = if self.is_encode && self.is_plus {
                quote_plus(str, b"")
            } else if self.is_encode {
                quote(str, b"")
            } else if self.is_plus {
                unquote_plus(str)
            } else {
                unquote(str)
            };
            match rest {
                Ok(o) => {
                    println!("{}", o);
                }
                Err(o) => {
                    errln!("{:?}", o);
                }
            }

        }

    }
}
