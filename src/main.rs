extern crate clap;
extern crate hmda;

#[macro_use]
extern crate serde_derive;

//#[macro_use]
extern crate serde_json;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;
use hmda::status;

#[derive(Serialize, Deserialize, Debug)]
struct ServiceStatus {
    status: String,
    service: String,
    time: String,
    host: String,
}

fn main() {
    let matches = App::new("HMDA Platform CLI")
        .version("0.1.0")
        .author("Juan Marin Otero <juan.marin.otero@gmail.com>")
        .about("Command Line Interface client for the HMDA Platform")
        .subcommand(
            SubCommand::with_name("status")
                .about("HMDA API Health Check")
                .arg(
                    Arg::with_name("host")
                        .takes_value(true)
                        .index(1)
                        .help("sets the HMDA API host"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ts")
                .about("HMDA Transmittal Sheet")
                .subcommand(SubCommand::with_name("generate"))
                .about("Generates Sample Transmittal Sheet"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        println!("An error has occured: {}", e);
        process::exit(1);
    }

    if let Result::Ok(response) = run(&matches) {
        let deserialized: ServiceStatus = serde_json::from_str(&response).unwrap();
        println!("{:?}", deserialized);
    }

    fn run(matches: &ArgMatches) -> Result<String, String> {
        match matches.subcommand() {
            ("status", Some(m)) => run_status(m),
            ("ts", Some(m)) => match m.subcommand {
                //("generate", Some(m)) => run_ts_generate(m),
                _ => Ok(String::new()),
            },
            _ => Ok(String::new()),
        }
    }

    fn run_status(matches: &ArgMatches) -> Result<String, String> {
        let maybe_hostname = matches.value_of("host");
        let hostname = match maybe_hostname {
            Some(host) => host,
            None => "https://ffiec-api.cfpb.gov/public/",
        };
        status::hmda_api_status2(hostname)
    }

    //fn run_ts_generate(matches: &ArgMatches) -> Result<String, String> {

    //}
}
