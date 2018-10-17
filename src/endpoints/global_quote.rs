/// Request to the GLOBAL_QUOTE function
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Request {
    pub symbol: String,
}

impl Request {
    pub fn get_url(&self, base_url: &str, api_key: &str) -> String {
        let function = "GLOBAL_QUOTE";
        let datatype = "json";
        let querystring = format!("function={}&datatype={}&apikey={}&symbol={}", function, datatype, api_key, self.symbol);
        let url = format!("{}?{}", base_url, querystring);
        url
    }
}

/// Response from the GLOBAL_QUOTE function
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Response {
    #[serde(rename = "01. symbol")]
    pub symbol: String,
    #[serde(rename = "02. open")]
    pub open: String, // f64,
    #[serde(rename = "03. high")]
    pub high: String, // f64,
    #[serde(rename = "04. low")]
    pub low: String, // f64,
    #[serde(rename = "05. price")]
    pub price: String, // f64,
    #[serde(rename = "06. volume")]
    pub volume: String, // u64,
    #[serde(rename = "07. latest trading day")]
    pub latest_trading_day: String,
    #[serde(rename = "08. previous close")]
    pub previous_close: String, // f64,
    #[serde(rename = "09. change")]
    pub change: String, // f64,
    #[serde(rename = "10. change percent")]
    pub change_percent: String, // f64,
}