use futures::{future::ready, stream::futures_unordered::FuturesUnordered, StreamExt, TryFutureExt};
use reqwest::{header, Client};

use crate::consts::basic_runtime;
use std::{net::IpAddr, time::Duration};

static UA: &str = "curl/7.82.0";
static UA_CHROME: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36";
static ACCEPT: &str = "Accept: */*";
static ACCEPT_JSON: &str = "Accept: application/json";

type Host<'a> = (&'a str, Option<&'a str>, &'a str, &'a str);
static HOSTS: &[Host] = &[
    // https://www.ipip.net
    ("https://myip.ipip.net", None, UA, ACCEPT),
    // https://ipinfo.io/developers
    // curl ipinfo.io
    // curl ipinfo.io/8.8.8.8
    ("https://ipinfo.io", Some("/"), UA, ACCEPT_JSON),
    // https://ip.sb/api
    // curl https://api.ip.sb/geoip
    // curl https://api.ip.sb/geoip/185.222.222.222
    ("https://api.ip.sb/geoip", Some("/"), UA_CHROME, ACCEPT_JSON),
    ("https://checkip.amazonaws.com", None, UA, ACCEPT_JSON),
];

pub type Ips = Vec<IpAddr>;

pub fn call(ips: Ips) {
    let mut rt = basic_runtime();

    let client = Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("reqwest Client build failed");

    let futs = FuturesUnordered::new();

    if ips.is_empty() {
        HOSTS.iter().for_each(|host| futs.push(curl(*host, None, &client)));
    } else {
        let hosts = HOSTS.iter().filter(|h| h.1.is_some());
        for ip in ips {
            hosts.clone().for_each(|host| futs.push(curl(*host, Some(ip), &client)));
        }
    }

    rt.block_on(futs.for_each(|_| ready(())))
}

async fn curl((url, custom_ip_mode, ua, accept): Host<'static>, ip: Option<IpAddr>, client: &Client) {
    let mut url = std::borrow::Cow::Borrowed(url);
    if let Some(ip) = &ip {
        let custum_ip_str = custom_ip_mode.unwrap();
        url = format!("{}{}{}", url, custum_ip_str, ip).into();
    }

    let res = client
        .get(url.as_ref())
        .header(header::USER_AGENT, ua)
        .header(header::ACCEPT, accept)
        .send()
        .map_err(|e| format!("Send request failed: {}", e))
        .and_then(|resp| {
            resp.text()
                .map_err(|e| format!("Read response's Body to string failed: {}", e))
        })
        .await;

    match res {
        Ok(body) => println!("{}\n{}", url, body),
        Err(e) => eprintln!("{}: {}", url, e),
    }
}
