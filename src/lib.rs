pub mod entities;
pub mod error;

use crate::error::PaddleError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as RClient;
use serde_json::Value;
use url::Url;

/// Paddle API client
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Client {
    client: RClient,
    auth: String,
    url: Url,
    paddle_version: Option<String>,
}

impl Client {
    /// Create a new Paddle API client
    ///
    /// ### Arguments
    /// `url` - the base URL for the Paddle API\
    /// `auth` - use Bearer authentication when making requests to the Paddle API
    // https://developer.paddle.com/api-reference/about/authentication
    pub fn new<T: Into<String>>(url: &str, auth: T) -> Result<Self, anyhow::Error> {
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
    fn default_headers(&self) -> Result<HeaderMap, anyhow::Error> {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.auth))?,
        );

        if let Some(version) = &self.paddle_version {
            headers.insert("Paddle-Version", HeaderValue::from_str(version)?);
        }

        Ok(headers)
    }

    /// You can test your authentication credentials
    /// by making a request to the Paddle API.
    ///
    /// `403 Forbidden` will be returned if the authentication fails.
    ///
    /// [Official document](https://developer.paddle.com/api-reference/about/authentication#test-authentication)
    pub async fn test_authentication(&self) -> Result<(), anyhow::Error> {
        Self::include_data_meta(
            PaddleError::handle_response(
                self.client
                    .get(self.url.join("event-types")?)
                    .headers(self.default_headers()?)
                    .send()
                    .await?,
            )
            .await?
            .text()
            .await?,
        )
    }

    // Response
    //
    // If successful, you should get
    // a response that includes a `data` array and a `meta` object.
    //
    // https://developer.paddle.com/api-reference/about/authentication#test-authentication-response
    fn include_data_meta(body: String) -> Result<(), anyhow::Error> {
        let v: Value = serde_json::from_str(&body)?;

        if let Some(data) = v.get("data") {
            if let Some(array) = data.as_array() {
                if array.is_empty() {
                    return Err(anyhow::anyhow!("Data is empty"));
                }
            } else {
                return Err(anyhow::anyhow!("Data is not a collection"));
            }
        } else {
            return Err(anyhow::anyhow!("Data is missing"));
        }

        if v.get("meta").is_none() {
            return Err(anyhow::anyhow!("Meta is missing"));
        }

        Ok(())
    }
}
