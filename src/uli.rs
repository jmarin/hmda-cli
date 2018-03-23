extern crate clap;

use self::clap::ArgMatches;

pub fn validate_uli(matches: &ArgMatches) -> Result<(), String> {
    let uli = matches.value_of("ULI").unwrap();
    println!("ULI: {}", uli);
    Ok(())
}

pub fn check_digit(matches: &ArgMatches) -> Result<(), String> {
    let loan_id = matches.value_of("LOAN_ID").unwrap();
    println!("LOAN ID: {}", loan_id);
    Ok(())
}
