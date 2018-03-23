extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate num_cpus;
extern crate tokio_core;

use std::io::{self, Write};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::future::Future;
use futures::Stream;

pub fn hmda_api_status(url: &str) -> Result<(), String> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let num_cpus = num_cpus::get();
    let client = Client::configure()
        .connector(HttpsConnector::new(num_cpus, &handle).unwrap())
        .build(&handle);

    let uri = url.parse().unwrap();

    let work = client.get(uri).and_then(|res| {
        res.body()
            .for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
    });
    Ok(core.run(work).unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn test_hmda_api_status() {}
}
