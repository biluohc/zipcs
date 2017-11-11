include!("ping.rs");


use regex::{self, Regex};

#[derive(Debug, Default)]
pub struct RegexList(pub Vec<Regex>);

impl RegexList {
    pub fn new<I, S>(res: I) -> Result<Self, regex::Error>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut rl = Self::default();
        for re in res {
            rl.0.push(Regex::new(re.as_ref())?);
        }
        Ok(rl)
    }
    pub fn find<'a>(&self, msg: &'a str) -> Option<&'a str> {
        // ::1应该是最短的吧?
        let addrs = msg.split('/')
            .filter(|s| s.len() >= 3)
            .collect::<Vec<&str>>();
        debugln!("{:?}", addrs);

        for addr in addrs {
            for re in self.0.as_slice().iter() {
                if let Some(mat) = re.find(addr) {
                    let rest = &addr[mat.start()..mat.end()];
                    debugln!("{:?}\n{:?} -> {:?}\n", re, msg, rest);
                    return Some(rest);
                }
            }
        }
        None
    }
    #[allow(dead_code)] // for test
    pub fn find_by_re<'a>(&self, msg: &'a str, idx: usize) -> Option<&'a str> {
        let addrs = msg.split('/')
            .filter(|s| s.len() >= 3)
            .collect::<Vec<&str>>();
        debugln!("{:?}", addrs);

        for addr in addrs {
            let rest = self.0[idx].find(addr).map(
                |mat| &addr[mat.start()..mat.end()],
            );
            if rest.is_some() {
                return rest;
            }
        }
        None
    }
}

lazy_static! {
        pub static ref RE: RegexList = RegexList::new(
            vec![
                // Ipv4
                r#"((2[0-4]\d|25[0-4]|[01]?\d\d?)\.){3}(2[0-4]\d|25[0-4]|[01]?\d\d?)"#,
                // domainName
                r#"[a-zA-Z0-9][-a-zA-Z0-9]{0,62}(\.[a-zA-Z0-9][-a-zA-Z0-9]{0,62})+\.?"#,
                r#"localhost"#,
            ]
        ).unwrap();
    // localhost 如果管前后的字符的话太麻烦了, 不能方便的写成一个 ...
    // let re = Regex::new(r#"[(.+://)](?P<localhost>(localhost))([:/].*)?"#).unwrap();
    // println!("{:?}",re.captures("http://localhost").and_then(|c|c.name("localhost")));

       pub static ref RE6: RegexList = RegexList::new(
        //  https://stackoverflow.com/questions/53497/regular-expression-that-matches-valid-ipv6-addresses#
            vec![
                // Ipv6                
                r#"fe80:(:[0-9a-fA-F]{0,4}){0,4}"#,                    // fe80::215:ff:fec0:284e/64
                r#"([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}"#,           // 1:2:3:4:5:6:7:8
                r#"([0-9a-fA-F]{1,4}:){1,7}:"#,                        // 1::              1:2:3:4:5:6:7::
                r#"([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}"#,        // 1::8             1:2:3:4:5:6::8  1:2:3:4:5:6::8
                r#"([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}"#, // 1::7:8           1:2:3:4:5::7:8  1:2:3:4:5::8
                r#"([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}"#, // 1::6:7:8         1:2:3:4::6:7:8  1:2:3:4::8
                r#"([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}"#, // 1::5:6:7:8       1:2:3::5:6:7:8  1:2:3::8
                r#"([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}"#, // 1::4:5:6:7:8     1:2::4:5:6:7:8  1:2::8
                r#"[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})"#,      // 1::3:4:5:6:7:8   1::3:4:5:6:7:8  1::8 
                r#":((:[0-9a-fA-F]{1,4}){1,7}|:)"#,                    // ::2:3:4:5:6:7:8  ::2:3:4:5:6:7:8 ::8       ::

                // fe80::7:8%eth0   fe80::7:8%1     (link-local IPv6 addresses with zone index)
                // r#"fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}"#,

                // ::255.255.255.255   ::ffff:255.255.255.255  ::ffff:0:255.255.255.255  (IPv4-mapped IPv6 addresses and IPv4-translated addresses)
                r#"::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])"#,
                
                // 2001:db8:3:4::192.0.2.33  64:ff9b::192.0.2.33 (IPv4-Embedded IPv6 Address)
                r#"([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])"#,
                
                // domainName
                r#"[a-zA-Z0-9][-a-zA-Z0-9]{0,62}(\.[a-zA-Z0-9][-a-zA-Z0-9]{0,62})+\.?"#,
                r#"localhost"#,
            ]
        ).unwrap();
    }
