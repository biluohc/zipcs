use rayon::prelude::*;
use reqwest::{header, Client};

use std::error::Error;
use std::io::Read;

static HOSTS: &'static [&str] = &[
    "http://ip.cn/",
    "http://myip.ipip.net/",
    "http://ipinfo.io/",
];
// curl ip.cn -v
static UA: &'static str = "curl/7.54.0";
static ACCEPT: &'static str = "Accept: */*";

pub fn call() {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, UA.parse().unwrap());
    headers.insert(header::ACCEPT, ACCEPT.parse().unwrap());
    let client = Client::builder().default_headers(headers).build().unwrap();

    HOSTS.par_iter().for_each(|host| {
        if let Err(e) = curl(host, &client) {
            eprintln!("{}: {}", host, e);
        }
    })
}

fn curl(url: &str, client: &Client) -> Result<(), String> {
    let mut body = String::new();
    client
        .get(url)
        .send()
        .map_err(|e| format!("Send Request failed: {}", e.description()))
        .and_then(|mut resp| {
            resp.read_to_string(&mut body)
                .map_err(|e| format!("Read response's body to string failed: {}", e.description()))
        })?;
    println!("{}\n{}", url, body);
    Ok(())
}
