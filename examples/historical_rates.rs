use openexchangerate_rs::error::ApiError;
use openexchangerate_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() {
    let app_id = env::var("OPEN_EXCHANGE_RATE_APP_ID").expect("Missing env var!");
    let client = ApiClient::new("https://openexchangerates.org/api", &app_id);
    // client
    match client.historical("2025-01-01").await {
        Ok(data) => println!("{:?}", data.rates),
        Err(ApiError::HttpError(inner)) => println!("Error: {inner}"),
        Err(ApiError::InvalidDateFormat) => println!("Invalid date format"),
    };
}
