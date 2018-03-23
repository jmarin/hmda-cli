extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::io::{self, Write};
use status::hyper::Client;
use status::hyper_tls::HttpsConnector;
use status::tokio_core::reactor::Core;
use status::futures::future::Future;
use status::futures::Stream;

pub fn hmda_api_status(url: &str) -> Result<(), String> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
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
