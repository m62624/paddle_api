use super::*;
use crate::entities::product::ProductData;
use crate::list_products::ListProductsParams;
use std::env;

/// Environment variables:
/// `PADDLE_API_URL`
/// `PADDLE_API_AUTH`
/// `PADDLE_PRODUCT_ID`
pub struct Config {
    pub url: String,
    pub auth: String,
    product_id: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            url: env::var("PADDLE_API_URL")?,
            auth: env::var("PADDLE_API_AUTH")?,
            product_id: None,
        })
    }

    pub fn set_product_id(mut self, product_id: String) -> Self {
        self.product_id = Some(product_id);
        self
    }

    pub fn product_id(&self) -> Option<&str> {
        self.product_id.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod client {
        use super::*;

        #[test]
        fn new_t_0() -> Result<(), Box<dyn std::error::Error>> {
            let config = Config::new()?;
            Client::new(&config.url, &config.auth)?;
            Ok(())
        }

        #[test]
        #[should_panic]
        fn new_f_0() {
            let config = Config::new().unwrap();
            let mut client =
                Client::new(r###"https:::\/sandbox-api.paddle.com"###, &config.auth).unwrap();
            client.set_paddle_version("1");
        }

        #[tokio::test]
        async fn test_authentication_t_0() -> Result<(), Box<dyn std::error::Error>> {
            let config = Config::new()?;
            let client = Client::new(&config.url, &config.auth)?;
            client.test_authentication().await?;
            Ok(())
        }

        #[tokio::test]
        #[should_panic]
        async fn test_authentication_f_0() {
            let config = Config::new().unwrap();
            let client = Client::new(&config.url, "invalid").unwrap();
            client.test_authentication().await.unwrap();
        }

        #[tokio::test]
        async fn list_products_t_0() -> Result<(), Box<dyn std::error::Error>> {
            let config = Config::new()?;
            let client = Client::new(&config.url, &config.auth)?;
            let x = client.list_products(ListProductsParams::default()).await?;
            println!("{:#?}", x);
            Ok(())
        }

        #[tokio::test]
        async fn get_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
            let config = Config::new()?;
            // let config = Config::new()?.set_product_id("some id".to_string());

            let client = Client::new(&config.url, &config.auth)?;

            if let Some(product_id) = config.product_id() {
                client.get_product(product_id, None).await?;
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_create_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
            let config = Config::new()?;
            let client = Client::new(&config.url, &config.auth)?;

            let product_data = ProductData::new(
                "AeroEdit Student".to_string(),
                "standard".to_string(),
                "custom".to_string(),
            )
            .description("Essential tools for student pilots to manage flight logs, analyze performance, and plan routes, and ensure compliance. Valid student pilot certificate from the FAA required.".to_string())
            .image_url("https://paddle.s3.amazonaws.com/user/165798/bT1XUOJAQhOUxGs83cbk_pro.png".to_string())
            .custom_data(serde_json::json!({"key": "value"}));

            client.create_product(product_data).await?;
            Ok(())
        }
    }
}
