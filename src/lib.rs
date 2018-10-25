#[macro_use]
extern crate serde_derive;

extern crate reqwest;

pub mod client;
pub mod endpoints;

pub trait Request {
    type Response;

    fn build_url(&self, base_url: &str, api_key: &str) -> String;
    fn call(&self, url: &str) -> Result<Self::Response, reqwest::Error>;
}

pub trait Response {}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
