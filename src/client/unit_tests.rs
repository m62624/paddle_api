use super::*;
use crate::list_products::ListProductsParams;
use std::env;

/// Environment variables:
/// `PADDLE_API_URL`
/// `PADDLE_API_AUTH`
pub struct Config {
    pub url: String,
    pub auth: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            url: env::var("PADDLE_API_URL")?,
            auth: env::var("PADDLE_API_AUTH")?,
        })
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
            let client = Client::new(&config.url, &config.auth)?;
            let x = client
                .get_product("", None)
                .await?;
            println!("{:#?}", x);
            Ok(())
        }
    }
}
