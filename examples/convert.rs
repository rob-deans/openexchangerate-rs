use openexchangerate_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() {
    let app_id = env::var("OPEN_EXCHANGE_RATE_APP_ID").expect("Missing env var!");
    let client = ApiClient::new("https://openexchangerates.org/api", &app_id);
    // client
    match client.convert(100, "USD", "EUR").await {
        Ok(response) => println!("Conversion rate: {}", response.value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
