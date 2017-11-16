use encoding::DecoderTrap;
use chardet::{detect,charset2encoding};
use encoding::label::encoding_from_whatwg_label;

use super::consts::space_fix;
use std::process::Command as Cmd;
use std::process::Output;
use std::time::Duration;
use std::process::{Stdio, Child};
use std::thread::sleep;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct Pings {
    pub _6: bool,
    pub only_line: bool,
    pub hosts: Vec<String>,
    pub count: u64,
}
impl Pings {
    pub fn check_fix(&mut self) -> Result<(), String> {
        let mut vs = Vec::new();
        for arg in &self.hosts {
            let addr =  if self._6 {
                RE6.find(arg)
            } else {
                RE.find(arg)
            };
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
        dbstln!("{:?}",self);
        let host_len_max = self.hosts.as_slice().iter().max_by_key(|p|p.len()).unwrap().len();
        // sleep sort
        let mut childs = self.hosts.as_slice().iter()
          .map(|host|ping(host, &self)
          .map_err(|e| eprintln!("exec ping fails: {}", e.description())))
          .filter_map(|r| r.ok())
          .collect::<Vec<_>>();

        while !childs.is_empty() {
            let mut wait = true; 
            let mut idx =0;

            while idx+1 <= childs.len() {
                match childs[idx].0.try_wait() {             
                        Ok(None) => {
                            idx+=1;
                            continue;
                        }
                        _=> {
                            wait = false;
                            let (r, host) = childs.remove(idx);
                            callback(r, host, self.only_line,  host_len_max);
                        }
                }
            }
            if  wait && !childs.is_empty() {
                // 1ms=1000_000ns
                sleep(Duration::new(0, 100_000));
            }
        }
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

fn ping<'a>(host:&'a str, config: &Pings)-> io::Result<(Child, &'a str)> {
    let count_str = format!("{}", config.count);
    let mut args = Vec::new();
    let mut ping = "ping";
    // -6
    if config._6 {
        if cfg!(unix){
            ping = "ping6";
        } else {
        args.push("-6");}
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

    Cmd::new(ping).args(&args[..]).stderr(Stdio::piped()).stdout(Stdio::piped()).spawn().map(|r|(r, host))
}

fn callback(child: Child, host: &str,  only_line: bool,  host_len_max: usize) {
    let output = child.wait_with_output().map_err(|e|eprintln!("wait ping fails: {}", e.description()));

    if let Ok(output) =output {
        if output.status.success() && !output.stdout.is_empty() {
            printf0(&output, only_line, host, host_len_max);
        }else if !output.status.success() && !output.stdout.is_empty() {
            printf1(&output, only_line, host, host_len_max);
        }else if !output.stderr.is_empty() {
            printf_err(&output, host, host_len_max);
        } else {
            eprintln!("ping {:?} -> code: {:?}, stdout.is_empty: {}, stderr.is_empty: {}", host, output.status,output.stdout.is_empty(), output.stderr.is_empty());
        }
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
    dbstln!("{:?}", msg);

    #[cfg(unix)]    
    assert!(!vs.len()>2);   
    #[cfg(unix)]
    println!("{}: {} -> {}",
             space_fix(host, host_len_max),
             vs[vs.len() - 1],
             vs[vs.len() - 2]);

    #[cfg(windows)]
    assert!(!vs.len()>3);   
    #[cfg(windows)]
    println!("{}: {} -> {}",
             space_fix(host, host_len_max),
             vs[vs.len() - 1],
             vs[vs.len() - 3]);
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
    dbstln!("{:?}", msg);

    #[cfg(unix)]    
    assert!(!vs.len()>2);   
    #[cfg(unix)]
    println!("{}: -> {}",
             space_fix(host, host_len_max),
             vs[vs.len() - 1]);

    #[cfg(windows)]
    assert!(!vs.len()>3);   
    #[cfg(windows)]
    println!("{}: -> {}",
             space_fix(host, host_len_max),
             vs[vs.len() - 1]);
}
fn printf_err(msg: &Output, host: &str, host_len_max: usize) {
    let msg = decode(&msg.stderr[..]);
    let vs: Vec<String> = msg.trim()
        .lines()
        .map(|s| s.trim().to_string())
        .collect();
    assert!(!vs.is_empty());

    errln!("{}: {}", space_fix(host, host_len_max), vs[vs.len() - 1]);
}

// #[cfg(unix)]
// fn decode(msg: &[u8]) -> String {
//     String::from_utf8_lossy(msg).into_owned().to_owned()
// }

// #[cfg(windows)]
fn decode(msg: &[u8]) -> String {
    encoding_from_whatwg_label(charset2encoding(&detect(msg).0))
        .and_then(|code| code.decode(msg, DecoderTrap::Strict).ok())
        .unwrap_or(String::from_utf8_lossy(msg).into_owned().to_owned())
}

