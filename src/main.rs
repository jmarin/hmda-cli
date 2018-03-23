extern crate clap;
extern crate hmda;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;
use hmda::uli;

fn main() {
    let matches = App::new("HMDA Platform CLI")
        .version("0.1.0")
        .author("Juan Marin Otero <juan.marin.otero@gmail.com>")
        .about("Command Line Interface client for the HMDA Platform")
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .help("Sets verbosity level"),
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

    if let Err(e) = run(matches) {
        println!("An error has occured: {}", e);
        process::exit(1);
    }

    fn run(matches: ArgMatches) -> Result<(), String> {
        match matches.subcommand() {
            ("validate", Some(m)) => uli::validate(m),
            _ => Ok(()),
        }
    }
}
