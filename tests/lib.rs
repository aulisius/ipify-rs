extern crate hyper;
extern crate ipify_api;
use hyper::rt::{self, Future};

#[test]
fn async_ip() {
    rt::run(
        ipify_api::get_ip("")
            .map(|ip| println!("{}", ip))
            .map_err(|e| println!("{}", e)),
    );
}
