extern crate clap;
extern crate hmda;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;
use hmda::uli;
use hmda::status;

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
            SubCommand::with_name("uli")
                .about("Create check digit and validate a ULI")
                .subcommand(
                    SubCommand::with_name("validate")
                        .arg(
                            Arg::with_name("ULI")
                                .help("sets the value for the ULI")
                                .required(true)
                                .index(1)
                                .help("ULI value"),
                        )
                        .help("usage: hmda uli validate <ULI>"),
                )
                .subcommand(
                    SubCommand::with_name("check-digit")
                        .arg(
                            Arg::with_name("LOAN_ID")
                                .required(true)
                                .index(1)
                                .help("Loan ID value"),
                        )
                        .help("usage: hmda uli check-digit <LOAN_ID>"),
                ),
        )
        .get_matches();

    //if let Some(matches) = matches.subcommand_matches("uli") {
    //    println!("Called ULI");
    //}

    //run(matches).unwrap();

    if let Err(e) = run(&matches) {
        println!("An error has occured: {}", e);
        process::exit(1);
    }

    fn run(matches: &ArgMatches) -> Result<(), String> {
        match matches.subcommand() {
            ("status", Some(m)) => run_status(m),
            ("uli", Some(m)) => run_uli(m),
            _ => Ok(()),
        }
    }

    fn run_status(matches: &ArgMatches) -> Result<(), String> {
        let maybe_hostname = matches.value_of("host");
        let hostname = match maybe_hostname {
            Some(host) => host,
            None => "https://ffiec-api.cfpb.gov/public/",
        };
        status::hmda_api_status(hostname)
    }

    fn run_uli(matches: &ArgMatches) -> Result<(), String> {
        match matches.subcommand() {
            ("validate", Some(m)) => uli::validate_uli(m),
            ("check-digit", Some(m)) => uli::check_digit(m),
            _ => Ok(()),
        }
    }
}
