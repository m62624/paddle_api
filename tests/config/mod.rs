use once_cell::sync::Lazy;
use std::{env, sync::Arc};

/// Environment variables:
/// `PADDLE_API_URL`
/// `PADDLE_API_AUTH`
/// `PADDLE_PRODUCT_ID`
/// `PADDLE_PRICE_ID`

pub struct Config {
    pub url: String,
    pub auth: String,
    #[allow(dead_code)]
    pub product_id: String,
    #[allow(dead_code)]
    pub price_id: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv()?;
        Ok(Self {
            url: env::var("PADDLE_API_URL")?,
            auth: env::var("PADDLE_API_AUTH")?,
            product_id: env::var("PADDLE_PRODUCT_ID")?,
            price_id: env::var("PADDLE_PRICE_ID")?,
        })
    }
}

pub static CONFIG: Lazy<Arc<Config>> =
    Lazy::new(|| Arc::new(Config::new().expect("Failed to load config")));
