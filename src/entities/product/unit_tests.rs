use super::list_products::ListProductsParams;
use crate::entities::product::*;

use crate::Client;
use once_cell::sync::Lazy;
use std::{env, sync::Arc};

/// Environment variables:
/// `PADDLE_API_URL`
/// `PADDLE_API_AUTH`
/// `PADDLE_PRODUCT_ID`

pub struct Config {
    pub url: String,
    pub auth: String,
    pub product_id: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            url: env::var("PADDLE_API_URL")?,
            auth: env::var("PADDLE_API_AUTH")?,
            product_id: env::var("PADDLE_PRODUCT_ID")?,
        })
    }
}

pub static CONFIG: Lazy<Arc<Config>> =
    Lazy::new(|| Arc::new(Config::new().expect("Failed to load config")));

#[tokio::test]
async fn test_auth_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;
    client.test_authentication().await?;
    Ok(())
}

#[tokio::test]
#[should_panic]
async fn test_auth_t_1() {
    let config = Config::new().unwrap();
    let _client = Client::new(&config.url, "invalid_auth").unwrap();
}

#[tokio::test]
async fn test_get_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;
    client.get_product(&config.product_id, None).await?;
    Ok(())
}

#[tokio::test]
#[should_panic]
async fn test_get_product_t_1() {
    let config = Config::new().unwrap();
    let client = Client::new(&config.url, &config.auth).unwrap();
    let _ = client
        .get_product("invalid_product_id", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_list_products_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;
    let r = client
        .get_list_products(ListProductsParams::default())
        .await?;

    if r.data().is_empty() {
        panic!("No products found");
    }

    Ok(())
}

#[tokio::test]
async fn test_update_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;

    let product_id = &config.product_id;
    let name = "AeroEdit Student";

    client
        .update_product(
            product_id,
            UpdateProductRequest::new(ProductData::new(name).set_status(ProductStatus::Active)),
        )
        .await?;

    client
        .update_product(
            product_id,
            UpdateProductRequest::new(ProductData::new(name).set_status(ProductStatus::Archived)),
        )
        .await?;

    Ok(())
}
