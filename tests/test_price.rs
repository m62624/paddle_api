mod config;

use config::*;

use paddle_api::entities::{
    price::{list::ListPricesParams, Price},
    EntityBaseGettersSetters, EntityStatus,
};
use paddle_api::Client;

#[tokio::test]
async fn test_get_price_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;
    let r = client.get_price(&config.price_id, None).await?;

    println!("Get price response: {:#?}", r);
    Ok(())
}

#[tokio::test]
#[should_panic]
async fn test_get_price_t_1() {
    let config = Config::new().unwrap();
    let client = Client::new(&config.url, &config.auth).unwrap();
    let _ = client
        .get_price("invalprice_id_price_id", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_list_prices_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;
    let r = client.get_list_prices(ListPricesParams::default()).await?;

    if r.data().is_empty() {
        panic!("No prices found");
    } else {
        println!("Get list prices response: {:#?}", r);
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_update_price_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;

    let price_id = &config.price_id;

    let r = client
        .update_price(price_id, Price::default().set_status(EntityStatus::Active))
        .await?;

    println!("Update price response (Active): {:#?}", r);

    let r = client
        .update_price(
            price_id,
            Price::default().set_status(EntityStatus::Archived),
        )
        .await?;

    println!("Update price response (Archived): {:#?}", r);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_create_price_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;

    let r = client.create_price(Price::default()).await?;

    println!("Create price response: {:#?}", r);

    Ok(())
}
