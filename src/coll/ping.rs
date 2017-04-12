use duct::cmd;
use poolite::{IntoPool, Pool};
use stderr::Loger;

use std::net::ToSocketAddrs;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct Pings {
    pub _6: bool,
    pub only_line: bool,
    pub hosts: Vec<String>,
    pub count: u64,
}
impl Pings {
    pub fn call(self) {
        // sleep_sort
        let pool = Pool::new().min(self.hosts.len()).run().into_pool();
        let config = Arc::from(self);
        let host_len_max = {
            let mut len = 0;
            for str in &config.hosts {
                if str.len() > len {
                    len = str.len();
                }
            }
            len
        };
        for idx in 0..config.hosts.len() {
            let config = config.clone();
            pool.push(move || ping(config, idx, host_len_max));
        }
        pool.join();
    }
    pub fn check_fix(&mut self) -> Result<(), String> {
        let mut vs = Vec::new();
        for arg in &self.hosts {
            let mut arg_tmp = Vec::new();
            arg.split('/')
                .filter(|s| !s.trim().is_empty())
                .map(|s| if (s, 0).to_socket_addrs().is_ok() {
                         arg_tmp.push(s.to_string());
                     } else {
                         s.split(':')
                             .filter(|ss| !ss.trim().is_empty())
                             .map(|ss| if (ss, 0).to_socket_addrs().is_ok() {
                                      arg_tmp.push(ss.to_string());
                                  })
                             .count();
                     })
                .count();
            if arg_tmp.is_empty() {
                return Err(format!("ARG is't contains reachable domain/ip: {} ", arg));
            }
            vs.extend(arg_tmp);
        }
        assert!(!vs.is_empty());
        self.hosts = vs;
        Ok(())
    }
}

impl Default for Pings {
    fn default() -> Self {
        Pings {
            _6: false,
            only_line: false,
            hosts: Vec::new(),
            count: 3,
        }
    }
}

fn ping(config: Arc<Pings>, idx: usize, host_len_max: usize) {
    let count_str = format!("{}", config.count);
    let host = &config.hosts[idx];
    let mut args = Vec::new();
    // -6
    if config._6 {
        args.push("-6");
    }
    // -count
    if cfg!(unix) {
        args.push("-c");
    } else {
        args.push("-n");
    };
    args.push(&count_str);
    // host
    args.push(host);

    match cmd("ping", &args[..]).read() {
        Ok(o) => {
            if config.only_line {
                printf(&o, host, host_len_max);
            } else {
                println!("{}\n", o);
            }
        }
        Err(e) => {
            dbstln!("{:?}", e);
            errln!("{}", e.description());
        }
    }
}

fn printf(msg: &str, host: &str, host_len_max: usize) {
    let vs: Vec<String> = msg.lines().map(|s| s.to_string()).collect();
    dbstln!("{:?}", msg);
    let space_fix = |host: &str| {
        let mut str = host.to_owned();
        while str.len() < host_len_max {
            str += " ";
        }
        str
    };
    println!("{}:  {} -> {}",
             space_fix(host),
             vs[vs.len() - 1],
             vs[vs.len() - 2]);
}
