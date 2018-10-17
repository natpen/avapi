use reqwest::{self};

use endpoints;

pub enum Request {
    Quote(endpoints::global_quote::Request),
}

impl Request {
    pub fn call(&self, url: &str) -> Result<Response, reqwest::Error> {
        // TODO: inherit persistent reqwest client from mod client
        let res: Response = reqwest::get(url)?.json()?;
        Ok(res)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Response {
    // TODO: refactor this rename source of truth to endpoints/
    #[serde(rename = "Global Quote")]
    Quote(endpoints::global_quote::Response),
}
