# openexchangerate-rs

This Rust library wraps the Open Exchange Rates API, allowing you to easily convert currencies and fetch exchange rates.

## Usage Example

Add the following to your `Cargo.toml`:

```toml
[dependencies]
openexchangerate_rs = "0.1"
tokio = { version = "1.42.0", features = ["full"] }
```

Create a new file `main.rs` with the following content:

```rust
use openexchangerate_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() {
    let app_id = env::var("OPEN_EXCHANGE_RATE_APP_ID").expect("Missing env var!");
    let client = ApiClient::new("https://openexchangerates.org/api", &app_id);

    match client.convert(100, "USD", "EUR").await {
        Ok(response) => println!("Conversion rate: {}", response.value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Run the example:

```bash
export OPEN_EXCHANGE_RATE_APP_ID=your_api_key_here
cargo run
```

Or run examples under `examples/`

```bash
cargo run --example currencies
```

## Getting an API Key

To get an API key, sign up at [Open Exchange Rates](https://openexchangerates.org/signup) and retrieve your app ID from the dashboard.
