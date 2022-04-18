use std::{env, process};
use whois_rust::{WhoIs, WhoIsLookupOptions};

fn main() {
    let whois = WhoIs::from_path("/Users/ijohnson/servers.json").unwrap();
    let args = env::args();
    if args.len() < 2 {
        eprintln!("Usage: wl <target>");
        process::exit(1);
    }
    let target = env::args().last().unwrap();
    let result: String = whois.lookup(WhoIsLookupOptions::from_string(target).unwrap()).unwrap();
    println!("{}", result);
    process::exit(0);
}
