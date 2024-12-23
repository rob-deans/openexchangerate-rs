use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Http error")]
    HttpError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing env var OPEN_EXCHANGE_RATE_BASE_URL")]
    MissingBaseUrl,
    #[error("missing env var OPEN_EXCHANGE_RATE_APP_ID")]
    MissingAppId,
}
