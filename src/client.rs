use reqwest::{self};

use models;

pub struct Client {
    pub api_key: String,
}

impl Client {
    const BASE_URL: &'static str = "https://www.alphavantage.co/query/";
    
    pub fn make_request(&self, request: models::Request) -> Result<models::Response, reqwest::Error> {
        let url = request.build_url(&Client::BASE_URL, &self.api_key);
        Ok(request.call(&url)?)
    }
}
