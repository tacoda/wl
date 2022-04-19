use std;
use structopt::StructOpt;
use exitcode;
use whois_rust::{WhoIs, WhoIsError, WhoIsLookupOptions};

#[derive(StructOpt)]
#[structopt(name = "wl", about = "WhoIs Lookup Tool")]
struct Cli {
    /// Host or IP address to lookup
    #[structopt(name = "HOST")]
    host: String,
}

fn code(error: WhoIsError) -> i32 {
    match error {
        WhoIsError::SerdeJsonError(_) => exitcode::CONFIG,
        WhoIsError::IOError(_) => exitcode::IOERR,
        WhoIsError::HostError(_) => exitcode::CONFIG,
        _ => exitcode::USAGE,
    }
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

fn run_app() -> Result<String, WhoIsError> {
    let cli = Cli::from_args();
    let whois = whois().unwrap();
    whois.lookup(WhoIsLookupOptions::from_string(cli.host).unwrap())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(result) => {
            println!("{}", result);
            exitcode::OK
        },
        Err(err) => {
            eprintln!("error: {}", err);
            code(err)
        }
    });
}
