extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

mod models;

use models::{App, Category, Release};
use futures::Stream;
use futures::future::Future;
use hyper::{Client, Method, Request};
use hyper::header::{Authorization, ContentLength, ContentType};
use hyper_tls::HttpsConnector;
use std::vec::Vec;
use tokio_core::reactor::Handle;

pub fn get_categories(handle: &Handle) -> Box<Future<Item = Vec<Category>, Error = &'static str>> {
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

fn get_app_signature(_: &String) -> Result<String, ()> {
    Ok("xxx".to_owned())
}

pub fn release_app(
    handle: &Handle,
    app_id: &String,
    url: &String,
    is_nightly: bool,
    api_token: &String,
) -> Box<Future<Item = (), Error = &'static str>> {
    let uri = "https://apps.nextcloud.com/api/v1/apps/releases"
        .parse()
        .expect("to parse");
    let release = Release {
        download: url.to_owned(),
        signature: get_app_signature(app_id).unwrap(),
        nightly: is_nightly,
    };
    let release_json = serde_json::to_string(&release).unwrap();
    println!("{}", release_json);
    let client = Client::configure()
        .connector(HttpsConnector::new(4, handle).unwrap())
        .build(handle);
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut()
        .set(Authorization(format!("Token {}", api_token)));
    req.headers_mut().set(ContentType::json());
    req.headers_mut()
        .set(ContentLength(release_json.len() as u64));
    req.set_body(release_json);
    let work = client
        .request(req)
        .and_then(|res| {
            println!("Status: {}", res.status());
            Ok(())
        })
        .map_err(|_| "whoops");

    Box::new(work)
}
