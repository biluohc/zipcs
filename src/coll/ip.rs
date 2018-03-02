use reqwest::{header, Client};
use rayon::prelude::*;

use std::error::Error;
use std::io::Read;

static HOSTS: &'static [&str] = &[
    "http://ip.cn/",
    "http://myip.ipip.net/",
    "http://ipinfo.io/",
];
// curl ip.cn -v
static UA: &'static str = "curl/7.52.1";

pub fn call() {
    let mut headers = header::Headers::new();
    headers.set(header::UserAgent::new(UA.to_string()));
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
        .map_err(|e| format!("Send Request fails: {}", e.description()))
        .and_then(|mut resp| {
            resp.read_to_string(&mut body)
                .map_err(|e| format!("Read response's body to string fails: {}", e.description()))
        })?;
    println!("{}\n{}", url, body);
    Ok(())
}
