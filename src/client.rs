use reqwest::Client;
use std::collections::HashMap;
use std::env;

use crate::models::conversion::ConversionResponse;
use crate::models::rates::Rates;
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

        let request = self
            .client
            .get(&url)
            .header("Authorization", format!("Token {}", self.app_id));
        let response = request.send().await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub async fn latest(&self) -> Result<Rates, ApiError> {
        self.get("latest.json").await
    }
    pub async fn historical(&self, date: &str) -> Result<Rates, ApiError> {
        if !regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(date)
        {
            return Err(ApiError::InvalidDateFormat);
        }
        self.get::<Rates>(&format!("historical/{}.json", date))
            .await
    }

    pub async fn currencies(&self) -> Result<HashMap<String, String>, ApiError> {
        self.get("currencies.json").await
    }

    pub async fn convert(
        &self,
        value: i32,
        from: &str,
        to: &str,
    ) -> Result<ConversionResponse, ApiError> {
        let url = format!("convert/{value}/{from}/{to}");
        println!("{url}");
        self.get::<ConversionResponse>(&url).await
    }
}
