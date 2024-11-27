mod config;

use config::*;

use paddle_api::entities::{
    product::{list::ListProductsParams, Product, ProductTaxCategory},
    EntityBaseGettersSetters, EntityStatus,
};
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

#[tokio::test]
async fn test_get_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;
    let r = client.get_product(&config.product_id, None).await?;

    println!("Get product response: {:#?}", r);
    Ok(())
}

#[tokio::test]
#[should_panic]
async fn test_get_product_t_1() {
    let config = Config::new().unwrap();
    let client = Client::new(&config.url, &config.auth).unwrap();
    let _ = client.get_product("invalid_id", None).await.unwrap();
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
    } else {
        println!("Get list products response: {:#?}", r);
    }

    Ok(())
}

#[tokio::test]
async fn test_update_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;

    let id = &config.product_id;

    let r = client
        .update_product(id, Product::default().set_status(EntityStatus::Active))
        .await?;

    println!("Update product response (Active): {:#?}", r);

    let r = client
        .update_product(id, Product::default().set_status(EntityStatus::Archived))
        .await?;

    println!("Update product response (Archived): {:#?}", r);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_create_product_t_0() -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG.clone();
    let client = Client::new(&config.url, &config.auth)?;

    let r = client
        .create_product(Product::default().set_tax_category(ProductTaxCategory::Standard))
        .await?;

    println!("Create product response: {:#?}", r);

    Ok(())
}
