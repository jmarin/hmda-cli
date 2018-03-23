extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use status::tokio_core::reactor::Core;

pub fn hmda_api_status(url: &str) -> Result<(), String> {
    println!("API STATUS");
    Ok(())
}
