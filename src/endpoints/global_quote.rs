/// Request to the GLOBAL_QUOTE function
use reqwest;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GlobalQuoteRequest {
    pub symbol: String,
}

impl ::Request for GlobalQuoteRequest {
    type Response = GlobalQuoteResponse;

    fn build_url(&self, base_url: &str, api_key: &str) -> String {
        let function = "GLOBAL_QUOTE";
        let datatype = "json";
        let querystring = format!(
            "function={}&datatype={}&apikey={}&symbol={}",
            function, datatype, api_key, self.symbol
        );
        let url = format!("{}?{}", base_url, querystring);
        url
    }

    fn call(&self, url: &str) -> Result<Self::Response, reqwest::Error> {
        let res: Self::Response = reqwest::get(url)?.json()?;
        Ok(res)
    }
}

/// Response from the GLOBAL_QUOTE function
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GlobalQuoteResponse {
    #[serde(rename = "01. symbol")]
    pub symbol: String,
    #[serde(rename = "02. open")]
    pub open: String, // TODO: parse to f64
    #[serde(rename = "03. high")]
    pub high: String, // TODO: parse to f64
    #[serde(rename = "04. low")]
    pub low: String, // TODO: parse to f64
    #[serde(rename = "05. price")]
    pub price: String, // TODO: parse to f64
    #[serde(rename = "06. volume")]
    pub volume: String, // TODO: parse to u64
    #[serde(rename = "07. latest trading day")]
    pub latest_trading_day: String,
    #[serde(rename = "08. previous close")]
    pub previous_close: String, // TODO: parse to f64
    #[serde(rename = "09. change")]
    pub change: String, // TODO: parse to f64
    #[serde(rename = "10. change percent")]
    pub change_percent: String, // TODO: parse to f64
}

impl ::Response for GlobalQuoteResponse {}

// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
// pub enum Response {
//     // TODO: refactor this rename source of truth to endpoints/
//     #[serde(rename = "Global Quote")]
//     Quote(endpoints::global_quote::Response),
// }
