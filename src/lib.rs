mod entities;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as RClient;
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
    pub fn new(url: &str, auth: &'a str) -> Result<Self, anyhow::Error> {
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
}
