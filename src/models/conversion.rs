use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConversionResponse {
    pub disclaimer: String,
    pub license: String,
    pub request: Request,
    pub meta: Meta,
    #[serde(rename(deserialize = "response"))]
    pub value: f64,
}

#[derive(Deserialize, Debug)]
pub struct Request {
    pub query: String,
    pub amount: f64,
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub timestamp: u32,
    pub rate: f64,
}
