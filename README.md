# Avapi

A Rust-based wrapper for the Alpha Vantage API for financial data. 

## Introduction

`avapi` is a Rust Crate for consuming the Alpha Vantage API as a client. `avapi` is based on [Reqwest](https://docs.rs/reqwest).

## Installation & Configuration

In your `Cargo.toml`, add the following lines:

```toml
[dependencies]
avapi = "0.1"
```
### Example Usage

```rust,no_run
extern crate avapi;

use avapi::{endpoints, models};

fn main() {
    let api_key = String::from("<YOUR AV KEY>");
    let client = avapi::client::Client {
        api_key: api_key,
    };
    let request =  models::Request::Quote(endpoints::global_quote::Request {
        symbol: String::from("GOOG"),
    });
    let res = client.make_request(request).unwrap();
    println!("{:#?}", res);
}
```

## Requirements

This crate has been tested with Stable Rust versions 1.29.2 and above.

## License

MIT
