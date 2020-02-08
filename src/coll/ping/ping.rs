use chardet::{charset2encoding, detect};
use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;
use futures::{future::ready, stream::futures_unordered::FuturesUnordered, FutureExt, StreamExt};
use tokio::process::Command;

use crate::consts::{basic_runtime, space_fix};
use std::process::Output;

#[derive(Debug)]
pub struct Pings {
    pub v6: bool,
    pub only_line: bool,
    pub hosts: Vec<String>,
    pub count: u64,
}
impl Pings {
    pub fn check_fix(&mut self) -> Result<(), String> {
        let mut vs = Vec::new();
        for arg in &self.hosts {
            let addr = if self.v6 { RE6.find(arg) } else { RE.find(arg) };
            if addr.is_none() {
                return Err(format!("ARG is't contains reachable domain/ip: {:?} ", arg));
            }
            vs.push(addr.unwrap().to_string())
        }
        debug_assert!(!vs.is_empty());
        self.hosts = vs;
        Ok(())
    }
    pub fn call(self) {
        debug!("{:?}", self);

        let mut rt = basic_runtime();

        let host_len_max = self.hosts.as_slice().iter().max_by_key(|p| p.len()).unwrap().len();

        // sleep sort
        let futs = FuturesUnordered::new();

        let only_line = self.only_line;
        for host in self.hosts.clone() {
            let fut = ping(host, &self, move |output: Output, host: String| {
                callback(output, host, only_line, host_len_max)
            });

            futs.push(fut);
        }

        rt.block_on(futs.for_each(|_| ready(())))
    }
}

impl Default for Pings {
    fn default() -> Self {
        Pings {
            v6: false,
            only_line: false,
            hosts: Vec::new(),
            count: 3,
        }
    }
}

async fn ping<F>(host: String, config: &Pings, callback: F)
where
    F: Fn(Output, String),
{
    let count_str = format!("{}", config.count);
    let mut args = Vec::new();
    let mut ping = "ping";
    // -6
    if config.v6 {
        if cfg!(unix) {
            ping = "ping6";
        } else {
            args.push("-6");
        }
    }
    // -count
    if cfg!(unix) {
        args.push("-c");
    } else {
        args.push("-n");
    };
    args.push(&count_str);

    // host
    args.push(&host);

    let mut cmd = Command::new(ping);
    cmd.args(&args[..]);
    cmd.output()
        .map(|res| match res {
            Ok(output) => callback(output, host),
            Err(e) => error!("Running ping command failed: {:?}", e),
        })
        .await
}

fn callback(output: Output, host: String, only_line: bool, host_len_max: usize) {
    if output.status.success() && !output.stdout.is_empty() {
        printf0(&output, only_line, &host, host_len_max);
    } else if !output.status.success() && !output.stdout.is_empty() {
        printf1(&output, only_line, &host, host_len_max);
    } else if !output.stderr.is_empty() {
        printf_err(&output, &host, host_len_max);
    } else {
        error!(
            "ping {:?} -> code: {:?}, stdout.is_empty: {}, stderr.is_empty: {}",
            host,
            output.status,
            output.stdout.is_empty(),
            output.stderr.is_empty()
        );
    }
}

fn printf0(msg: &Output, only_line: bool, host: &str, host_len_max: usize) {
    let msg = decode(&msg.stdout[..]);
    let msg = msg.trim();
    // -l/--only-line
    if !only_line {
        println!("{}\n", msg);
        return;
    }

    let vs: Vec<String> = msg.lines().map(|s| s.trim().to_string()).collect();
    debug!("{:?}", msg);

    #[cfg(unix)]
    assert!(!vs.len() > 2);
    #[cfg(unix)]
    println!(
        "{}: {} -> {}",
        space_fix(host, host_len_max),
        vs[vs.len() - 1],
        vs[vs.len() - 2]
    );

    #[cfg(windows)]
    assert!(!vs.len() > 3);
    #[cfg(windows)]
    println!(
        "{}: {} -> {}",
        space_fix(host, host_len_max),
        vs[vs.len() - 1],
        vs[vs.len() - 3]
    );
}

// ping fuck.co -c 3                                                                                                              (%1)
// 3 packets transmitted, 0 received, 100% packet loss, time 2016ms
fn printf1(msg: &Output, only_line: bool, host: &str, host_len_max: usize) {
    let msg = decode(&msg.stdout[..]);
    let msg = msg.trim();
    // -l/--only-line
    if !only_line {
        println!("{}\n", msg);
        return;
    }

    let vs: Vec<String> = msg.lines().map(|s| s.trim().to_string()).collect();
    debug!("{:?}", msg);

    #[cfg(unix)]
    assert!(!vs.len() > 2);
    #[cfg(unix)]
    println!("{}: -> {}", space_fix(host, host_len_max), vs[vs.len() - 1]);

    #[cfg(windows)]
    assert!(!vs.len() > 3);
    #[cfg(windows)]
    println!("{}: -> {}", space_fix(host, host_len_max), vs[vs.len() - 1]);
}
fn printf_err(msg: &Output, host: &str, host_len_max: usize) {
    let msg = decode(&msg.stderr[..]);
    let vs: Vec<String> = msg.trim().lines().map(|s| s.trim().to_string()).collect();
    assert!(!vs.is_empty());

    eprintln!("{}: {}", space_fix(host, host_len_max), vs[vs.len() - 1]);
}

// #[cfg(unix)]
// fn decode(msg: &[u8]) -> String {
//     String::from_utf8_lossy(msg).into_owned()
// }

// #[cfg(windows)]
fn decode(bytes: &[u8]) -> String {
    encoding_from_whatwg_label(charset2encoding(&detect(bytes).0))
        .and_then(|code| code.decode(bytes, DecoderTrap::Strict).ok())
        .unwrap_or_else(|| String::from_utf8_lossy(bytes).into_owned())
}
