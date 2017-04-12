use requests::Request;

static IPS: [&'static str; 3] = ["http://ip.cn/",
                                 "http://myip.ipip.net/",
                                 "http://ipinfo.io/"];
// curl ip.cn -v
static UA: &'static str = "curl/7.52.1";

pub fn call() {
    let mut req = Request::new();
    req.user_agent(UA);
    for str in &IPS {
        if let Err(e) = curl(str, &req) {
            errln!("{}", e);
        }
    }
}

fn curl(url: &str, req: &Request) -> Result<(), String> {
    let resp = req.get(url)
        .map_err(|e| format!("{:?} Request GET fails: {}", url, e))?;
    let str = resp.text()
        .ok_or_else(|| format!("{:?} text GET fails", url))?;
    println!("{}\n{}", url, str);
    Ok(())
}