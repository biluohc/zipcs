use requests::Request;
use rayon::prelude::*;

static HOSTS: &'static [&str] = &[
    "http://ip.cn/",
    "http://myip.ipip.net/",
    "http://ipinfo.io/",
];
// curl ip.cn -v
static UA: &'static str = "curl/7.52.1";

pub fn call() {
    let mut req = Request::new();
    req.user_agent(UA);
    let req = ReqOnlyRead::new(req);

    HOSTS.par_iter().for_each(
        |host| if let Err(e) = curl(host, &req) {
            errln!("{}", e);
        },
    )
}

fn curl(url: &str, req: &ReqOnlyRead) -> Result<(), String> {
    let req = req.as_ref();
    let resp = req.get(url).map_err(|e| {
        format!("{:?} Request GET fails: {}", url, e)
    })?;
    let str = resp.text().ok_or_else(
        || format!("{:?} text GET fails", url),
    )?;
    println!("{}\n{}", url, str);
    Ok(())
}

#[derive(Debug)]
struct ReqOnlyRead(Request);

use std::marker::Sync;
unsafe impl Sync for ReqOnlyRead {}

impl ReqOnlyRead {
    fn new(req: Request) -> Self {
        ReqOnlyRead(req)
    }
}

impl AsRef<Request> for ReqOnlyRead {
    fn as_ref(&self)-> &Request {
        &self.0
    }
}