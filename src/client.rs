use reqwest::{self};

use models;

pub struct Client {
    pub api_key: String,
}

impl Client {
    const BASE_URL: &'static str = "https://www.alphavantage.co/query/";
    
    pub fn make_request(&self, request: models::Request) -> Result<models::Response, reqwest::Error> {
        let url: String;
        // TODO: make this automatically downcast, or use traits, or something, so
        // we don't need care about request's type here. see also "trait objects"
        // and polymorphism.
        match &request {
            &models::Request::Quote(ref q) => {
                url = q.get_url(&Client::BASE_URL, &self.api_key);
            },
        }
        Ok(request.call(&url)?)
    }
}
