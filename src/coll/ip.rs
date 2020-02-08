use futures::{future::ready, stream::futures_unordered::FuturesUnordered, StreamExt, TryFutureExt};
use reqwest::{header, Client};

use crate::consts::basic_runtime;
use std::time::Duration;

static HOSTS: &[&str] = &["https://ip.cn/", "https://myip.ipip.net/", "https://ipinfo.io/"];
// curl https://ip.cn -v
static UA: &str = "curl/7.54.0";
static ACCEPT: &str = "Accept: */*";

pub fn call() {
    let mut rt = basic_runtime();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, UA.parse().unwrap());
    headers.insert(header::ACCEPT, ACCEPT.parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .use_rustls_tls()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("reqwest Client build failed");

    let futs = FuturesUnordered::new();
    HOSTS.iter().for_each(|host| futs.push(curl(host, &client)));

    rt.block_on(futs.for_each(|_| ready(())))
}

async fn curl(url: &str, client: &Client) {
    let res = client
        .get(url)
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
