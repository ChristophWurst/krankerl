extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

mod models;

use models::{App, Category};
use futures::Stream;
use futures::future::Future;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::vec::Vec;
use tokio_core::reactor::Handle;

pub fn get_categories(
    handle: &Handle
) -> Box<Future<Item = Vec<Category>, Error = &'static str>> {
    let uri = "https://apps.nextcloud.com/api/v1/categories.json"
        .parse()
        .expect("to parse");
    let client = Client::configure()
        .connector(HttpsConnector::new(4, handle).unwrap())
        .build(handle);
    let work = client
        .get(uri)
        .and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let apps: Vec<Category> = serde_json::from_slice(&body).unwrap();
                Ok(apps)
            })
        })
        .map_err(|_| "whoops");

    Box::new(work)
}

pub fn get_apps_and_releases(
    handle: &Handle,
    version: &String,
) -> Box<Future<Item = Vec<App>, Error = &'static str>> {
    let uri = format!(
        "https://apps.nextcloud.com/api/v1/platform/{}/apps.json",
        version
    ).parse()
        .expect("to parse");
    let client = Client::configure()
        .connector(HttpsConnector::new(4, handle).unwrap())
        .build(handle);
    let work = client
        .get(uri)
        .and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let apps: Vec<App> = serde_json::from_slice(&body).unwrap();
                Ok(apps)
            })
        })
        .map_err(|_| "whoops");

    Box::new(work)
}
