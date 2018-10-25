extern crate avapi;
extern crate config;

use avapi::endpoints;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .unwrap()
        .merge(config::Environment::with_prefix("AVAPI"))
        .unwrap();
    let api_key = settings.get::<String>("api_key").unwrap();
    let client = avapi::client::Client { api_key: api_key };
    let request = endpoints::global_quote::GlobalQuoteRequest {
        symbol: String::from("GOOG"),
    };
    let res = client.make_request(request).unwrap();
    println!("{:#?}", res);
}
