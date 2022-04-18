use std::{env, process};
use structopt::StructOpt;
use whois_rust::{WhoIs, WhoIsError, WhoIsLookupOptions};

#[derive(StructOpt, Debug)]
#[structopt(name = "wl", about = "WhoIs Lookup")]
struct Cli {
    /// Host or IP address to lookup
    #[structopt(name = "HOST")]
    host: String,
}

fn whois() -> Result<WhoIs, WhoIsError> {
    let json = r#"{
        "org": "whois.pir.org",
        "": "whois.ripe.net",
        "_": {
            "ip": {
                "host": "whois.arin.net",
                "query": "n + $addr\r\n"
            }
        }
    }"#;
    WhoIs::from_string(json)
}

fn main() {
    let cli = Cli::from_args();
    let whois = whois().unwrap();
    let result = whois.lookup(WhoIsLookupOptions::from_string(cli.host).unwrap()).unwrap();
    println!("{}", result);
    process::exit(0);
}
