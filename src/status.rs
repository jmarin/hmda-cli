extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate num_cpus;
extern crate serde_json;
extern crate tokio_core;

use std::io;
use std::str;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::future::Future;
use futures::Stream;
use serde_json::Value;

pub fn hmda_api_status2(url: &str) -> Result<String, String> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let num_cpus = num_cpus::get();
    let client = Client::configure()
        .connector(HttpsConnector::new(num_cpus, &handle).unwrap())
        .build(&handle);
    let mut s = String::new();
    let uri = url.parse().unwrap();
    {
        let work = client.get(uri).and_then(|res| {
            res.body().for_each(|chunk| {
                s.push_str(str::from_utf8(&*chunk).unwrap());
                futures::future::ok(())
            })
        });
        core.run(work).unwrap();
    }
    Ok(s)
}

pub fn hmda_api_status(url: &str) -> Result<(), String> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let num_cpus = num_cpus::get();
    let client = Client::configure()
        .connector(HttpsConnector::new(num_cpus, &handle).unwrap())
        .build(&handle);

    let uri = url.parse().unwrap();

    let work_verbose = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value =
                serde_json::from_slice(&body).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            println!("Status for service {} is {}", v["service"], v["status"]);
            Ok(())
        })
    });

    //let work = client.get(uri).and_then(|res| {
    //    res.body()
    //        .for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
    //});
    core.run(work_verbose).unwrap();
    Ok(())
}
