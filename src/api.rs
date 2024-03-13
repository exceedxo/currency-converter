use reqwest::get;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::Result;
use thiserror::Error;
use crate::config::read_config;

// The API for the exchange rate service is provided by https://v6.exchangerate-api.com.
// I know reading the API key every time is not optimal, but i think it's good enough for this project.

const BASE_API_URL: &str = "https://v6.exchangerate-api.com/v6";

#[derive(Error, Debug)]
enum RequestError {
    #[error("Unsupported currency.")]
    UnsupportedCurrency,
    #[error("Malformed request.")]
    MalformedRequest,
    #[error("Invalid API key.")]
    InvalidApiKey,
    #[error("Inactive account.")]
    InactiveAccount,
    #[error("Quota reached.")]
    QuotaReached
}

#[derive(Serialize, Deserialize)]
pub struct ApiMultirateResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: Value
}

#[derive(Serialize, Deserialize)]
pub struct ApiRateResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ApiConversionResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
    pub conversion_result: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub result: String,
    #[serde(rename = "error-type")]
    pub error_type: String,
}

pub async fn get_all_exchange_rates(base_currency: &str) -> Result<ApiMultirateResponse> {
    let api_key = read_config()?.api_key;
    let url = format!("{}/{}/latest/{}", BASE_API_URL, api_key, base_currency);
    let response = get(&url).await?;
    let status = response.status();
    if !status.is_success() {
        let error = response.json::<ErrorResponse>().await?;
        match error.error_type.as_str() {
            "unsupported-code" => return Err(RequestError::UnsupportedCurrency.into()),
            "malformed-request" => return Err(RequestError::MalformedRequest.into()),
            "invalid-key" => return Err(RequestError::InvalidApiKey.into()),
            "inactive-account" => return Err(RequestError::InactiveAccount.into()),
            "quota-reached" => return Err(RequestError::QuotaReached.into()),
            _ => return Err(RequestError::MalformedRequest.into())
        }
    }
    let exchange_rate_response = response.json::<ApiMultirateResponse>().await?;
    Ok(exchange_rate_response)
}

pub async fn get_exchange_rate(from: &str, to: &str) -> Result<ApiRateResponse> {
    let api_key = read_config()?.api_key;
    let url = format!("{}/{}/pair/{}/{}", BASE_API_URL, api_key, from, to);
    let response = get(&url).await?;
    let status = response.status();
    if !status.is_success() {
        let error = response.json::<ErrorResponse>().await?;
        match error.error_type.as_str() {
            "unsupported-code" => return Err(RequestError::UnsupportedCurrency.into()),
            "malformed-request" => return Err(RequestError::MalformedRequest.into()),
            "invalid-key" => return Err(RequestError::InvalidApiKey.into()),
            "inactive-account" => return Err(RequestError::InactiveAccount.into()),
            "quota-reached" => return Err(RequestError::QuotaReached.into()),
            _ => return Err(RequestError::MalformedRequest.into())
        }
    }
    let exchange_rate_response = response.json::<ApiRateResponse>().await?;
    Ok(exchange_rate_response)
}

pub async fn convert(from: &str, to: &str, amount: f64) -> Result<ApiConversionResponse> {
    let api_key = read_config()?.api_key;
    let url = format!("{}/{}/pair/{}/{}/{}", BASE_API_URL, api_key, from, to, amount);
    let response = get(&url).await?;
    let status = response.status();
    if !status.is_success() {
        let error = response.json::<ErrorResponse>().await?;
        match error.error_type.as_str() {
            "unsupported-code" => return Err(RequestError::UnsupportedCurrency.into()),
            "malformed-request" => return Err(RequestError::MalformedRequest.into()),
            "invalid-key" => return Err(RequestError::InvalidApiKey.into()),
            "inactive-account" => return Err(RequestError::InactiveAccount.into()),
            "quota-reached" => return Err(RequestError::QuotaReached.into()),
            _ => return Err(RequestError::MalformedRequest.into())
        }
    }
    let exchange_rate_response = response.json::<ApiConversionResponse>().await?;
    Ok(exchange_rate_response)
}

