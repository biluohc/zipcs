use rayon::prelude::*;
use encoding::DecoderTrap;
use chardet::{detect,charset2encoding};
use encoding::label::encoding_from_whatwg_label;

use super::consts::space_fix;
use std::process::Command as Cmd;
use std::process::Output;
use std::error::Error;

#[derive(Debug)]
pub struct Pings {
    pub _6: bool,
    pub only_line: bool,
    pub hosts: Vec<String>,
    pub count: u64,
}
impl Pings {
    pub fn call(self) {
        dbstln!("{:?}",self);
        let host_len_max = self.hosts.as_slice().iter().max_by_key(|p|p.len()).unwrap().len();
        // sleep sort
        self.hosts.par_iter()
         .for_each(|host| ping(host, &self,host_len_max))
    }
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

fn ping(host:&str, config: &Pings,  host_len_max: usize) {
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

    let output = Cmd::new(ping)
        .args(&args[..])
        .output()
        .map_err(|e| panic!("exec ping fails: {}", e.description()))
        .unwrap();
    if output.status.success() && !output.stdout.is_empty() {
        printf(&output, config.only_line, host, host_len_max);
    } else if !output.stderr.is_empty() {
        printf_err(&output, host, host_len_max);
    }
}

fn printf(msg: &Output, only_line: bool, host: &str, host_len_max: usize) {
    debug_assert!(!msg.stdout.is_empty());
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
    let result = detect(msg);
    if let Some(code) =  encoding_from_whatwg_label(charset2encoding(&result.0)) {
       if let Ok(str)= code.decode(msg, DecoderTrap::Strict){
           return str;
       }
    }
    String::from_utf8_lossy(msg).into_owned().to_owned()
}
