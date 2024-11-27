mod config;

use config::*;

use paddle_api::Client;

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
    let client = Client::new(&config.url, "invalid_auth").unwrap();
    client.test_authentication().await.unwrap();
}
