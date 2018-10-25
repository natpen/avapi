use reqwest;

pub struct Client {
    pub api_key: String,
}

impl Client {
    const BASE_URL: &'static str = "https://www.alphavantage.co/query/";

    pub fn make_request<T>(&self, request: T) -> Result<T::Response, reqwest::Error>
    where
        T: ::Request,
    {
        let url = request.build_url(&Client::BASE_URL, &self.api_key);
        Ok(request.call(&url)?)
    }
}
