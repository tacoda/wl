use std::env;
use whois_rust::{WhoIs, WhoIsLookupOptions};

fn main() {
    let whois = WhoIs::from_path("/Users/ijohnson/servers.json").unwrap();
    let target = env::args().last().unwrap();
    let result: String = whois.lookup(WhoIsLookupOptions::from_string(target).unwrap()).unwrap();
    println!("{}", result);
}
