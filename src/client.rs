use reqwest::Client;
use std::env;

use crate::{error::ConfigError, ApiError};

pub struct ApiClient {
    base_url: String,
    app_id: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: &str, app_id: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
            app_id: app_id.to_string(),
            client: Client::new(),
        }
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        let base_url =
            env::var("OPEN_EXCHANGE_RATE_BASE_URL").map_err(|_| ConfigError::MissingBaseUrl)?;
        let app_id =
            env::var("OPEN_EXCHANGE_RATE_APP_ID").map_err(|_| ConfigError::MissingAppId)?;

        Ok(ApiClient {
            base_url: base_url.to_string(),
            app_id: app_id.to_string(),
            client: Client::new(),
        })
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T, ApiError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let request = self.client.get(&url).query(&[("app_id", &self.app_id)]);
        let response = request.send().await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }
}
