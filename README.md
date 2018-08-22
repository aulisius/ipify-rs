[![Crates.io](https://img.shields.io/crates/v/ipify_api.svg?style=plastic)](https://crates.io/crates/ipify_api/)
[![rust-stable](https://img.shields.io/badge/rust-stable-green.svg?style=plastic)](https://www.rust-lang.org/downloads.html)
[![crate-doc](https://img.shields.io/badge/docs-100%25-green.svg)](https://docs.rs/ipify_api/)

## ipify Client

This crate provides functionalities to retrieve your public IP. It uses `hyper` along with `ipify`. Works only on HTTP.

```rust
extern crate hyper;
extern crate ipify_api;
use hyper::rt::{self, Future};

fn main() {
    rt::run(
        ipify_api::get_ip("http://my-ipify-instance.rs")
            .map(|ip| println!("{}", ip))
            .map_err(|e| println!("{}", e)),
    );
}

```