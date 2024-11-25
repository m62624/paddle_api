pub mod check_auth;
pub mod create_product;
pub mod get_product;
pub mod list_products;
pub mod update_product;

#[cfg(test)]
mod unit_tests;

use super::error::PaddleError;
use reqwest::Client as RClient;
use serde_json::Value;
use url::Url;

/// Paddle API client
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Client<'a> {
    client: RClient,
    auth: &'a str,
    url: Url,
    paddle_version: Option<String>,
}

impl<'a> Client<'a> {
    /// Create a new Paddle API client
    ///
    /// ### Arguments
    /// `url` - the base URL for the Paddle API\
    /// `auth` - use Bearer authentication when making requests to the Paddle API
    // https://developer.paddle.com/api-reference/about/authentication
    pub fn new(url: &str, auth: &'a str) -> Result<Self, PaddleError> {
        Ok(Self {
            client: RClient::new(),
            auth: auth.into(),
            url: Url::parse(url.into())?,
            paddle_version: None,
        })
    }

    /*
    curl `URL`
    -H "Authorization: Bearer `token`"
    -H "Paddle-Version: 1"
    */
    /// Set the Paddle API version (only numbers)
    pub fn set_paddle_version(&mut self, version: &str) {
        self.paddle_version = Some(version.to_string());
    }

    /// Default headers for Paddle API requests
    fn default_headers(&self) -> Result<reqwest::header::HeaderMap, PaddleError> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_str("application/json")?,
        );

        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.auth))?,
        );

        if let Some(version) = &self.paddle_version {
            headers.insert(
                "Paddle-Version",
                reqwest::header::HeaderValue::from_str(version)?,
            );
        }

        Ok(headers)
    }
}
