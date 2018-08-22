//! The `ipify_api` crate provides functionalities to retrieve your
//! public IP. This crate builds on **rust-stable**.
//!
//! It uses `hyper` along with `ipify`.
//! To use this, add the following to your `Cargo.toml`.
//!
//!```text
//! [dependencies]
//! ipify_api = "1.0.0"
//!```
//!
//! and add this to your crate root:
//!
//!```
//! extern crate ipify;
//!```
extern crate hyper;
use hyper::rt::{Future, Stream};
use hyper::Client;

/// This function takes an ipify endpoint to query. Works only on HTTP.
/// If an empty string is given, it uses the default (http://api.ipify.org)
///
///```
/// # #[macro_use] extern crate restructure;
/// # extern crate hyper;
// extern crate ipify_api;
/// # use hyper::rt::{self, Future};
///
/// # fn main() {
/// # rt::run(
///     get_ip("http://my-custom-ipify-instance.rs")
///         .map(|ip| println!("{}", ip))
///         .map_err(|e| println!("{}", e));
/// # )
/// # }
///```
pub fn get_ip(url: &str) -> impl Future<Item = String, Error = hyper::Error> {
    Client::new()
        .get(match url.parse() {
            Ok(v) => v,
            Err(_) => "http://api.ipify.org".parse().unwrap(),
        })
        .and_then(|res| res.into_body().concat2())
        .map(|val| val.into_bytes().to_vec())
        .map(|ip| String::from_utf8(ip).unwrap())
}
