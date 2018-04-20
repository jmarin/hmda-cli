extern crate clap;
extern crate hmda;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;

#[macro_use]
extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use hmda::model::fi::ts::transmittal_sheet::TransmittalSheet;
use hmda::model::fi::lar::loan_application_register::LoanApplicationRegister;

use hmda::api::status::hmda_api_status_url;

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
                .about("Transmittal Sheet")
                .subcommand(
                    SubCommand::with_name("generate").about("Generate Sample Transmittal Sheet"),
                )
                .subcommand(
                    SubCommand::with_name("parse")
                        .about("Parse Transmittal Sheet")
                        .arg(
                            Arg::with_name("TS")
                                .help("Transmittal Sheet value")
                                .index(1),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("lar")
                .about("Loan Application Register")
                .subcommand(
                    SubCommand::with_name("generate").about("Generate Loan Application Register"),
                )
                .subcommand(
                    SubCommand::with_name("parse")
                        .about("Parse Loan Application Register")
                        .arg(
                            Arg::with_name("LAR")
                                .help("Loan Application Register value")
                                .index(1),
                        ),
                ),
        )
        .get_matches();

    run(&matches);

    //if let Err(e) = run(&matches) {
    //    println!("An error has occured: {}", e);
    //    process::exit(1);
    //}

    //if let Result::Ok(response) = run(&matches) {
    //    //println!("{}", response);
    //}

    fn run(matches: &ArgMatches) -> Result<(), String> {
        match matches.subcommand() {
            ("status", Some(m)) => run_status(m),
            //("ts", Some(m)) => run_ts(m),
            //("lar", Some(m)) => run_lar(m),
            _ => Ok(println!("Unknown command")),
        }
    }

    fn run_status(matches: &ArgMatches) -> Result<(), String> {
        let maybe_hostname = matches.value_of("host");
        let hostname = match maybe_hostname {
            Some(host) => String::from(host),
            None => String::from("https://ffiec-api.cfpb.gov/public/"),
        };
        let service_status = hmda_api_status_url(&hostname).unwrap();
        let mut table = Table::new();
        table.add_row(row!["STATUS", "SERVICE", "TIME", "HOST"]);
        table.add_row(Row::new(vec![
            Cell::new(&service_status.status),
            Cell::new(&service_status.service),
            Cell::new(&service_status.time),
            Cell::new(&service_status.host),
        ]));
        Ok(table.printstd())
    }

    fn run_ts(matches: &ArgMatches) -> Result<String, String> {
        match matches.subcommand() {
            ("generate", Some(_)) => Ok(TransmittalSheet::ts_sample().to_string()),
            _ => Ok(String::from("")),
        }
    }

    fn run_lar(matches: &ArgMatches) -> Result<String, String> {
        match matches.subcommand() {
            ("generate", Some(_)) => Ok(LoanApplicationRegister::lar_sample().to_string()),
            _ => Ok(String::from("")),
        }
    }
}
