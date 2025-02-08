use openexchangerate_rs::rates::Rates;
use openexchangerate_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::from_env().unwrap();
    // client
    match client.get::<Rates>("latest.json").await {
        Ok(data) => println!("{:?}", data.rates),
        Err(error) => println!("Error: {error}"),
    };
}
