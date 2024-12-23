use openexchangerate_rs::latest::Latest;
use openexchangerate_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::from_env().unwrap();
    // client
    match client.get::<Latest>("latest.json").await {
        Ok(data) => println!("{:?}", data.rates),
        Err(error) => println!("Error: {error}"),
    };
}
