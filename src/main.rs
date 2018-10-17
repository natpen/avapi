extern crate config;
extern crate avapi;

use avapi::{endpoints, models};

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).unwrap()
        .merge(config::Environment::with_prefix("AVAPI")).unwrap();
    let api_key = settings.get::<String>("api_key").unwrap();
    let client = avapi::client::Client {
        api_key: api_key,
    };
    let request =  models::Request::Quote(endpoints::global_quote::Request {
        symbol: String::from("GOOG"),
    });
    let res = client.make_request(request).unwrap();
    println!("{:#?}", res);
}
