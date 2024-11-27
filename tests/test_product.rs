mod config;

use config::*;

use paddle_api::entities::{
    product::{list::ListProductsParams, Product, ProductTaxCategory},
    BaseListParamsGettersSetters, EntityBaseGettersSetters, EntityStatus, EntityType,
};
use paddle_api::Client;

mod tests_get_product {
    use super::*;

    #[tokio::test]
    async fn t_0() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client.get_product(&config.product_id, None).await?;

        println!("Get product response: {:#?}", r);
        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn t_1() {
        let config = Config::new().unwrap();
        let client = Client::new(&config.url, &config.auth).unwrap();
        let _ = client.get_product("invalid_id", None).await.unwrap();
    }

    #[tokio::test]
    async fn t_2() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_product(
                &config.product_id,
                Some(vec!["prices".to_string(), "X".to_string()]),
            )
            .await?;

        println!("Get product response: {:#?}", r);

        assert!(r.data().prices().is_some());
        Ok(())
    }

    #[tokio::test]
    async fn t_3() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client.get_product(&config.product_id, None).await?;

        println!("Get product response: {:#?}", r);

        assert!(r.data().prices().is_none());
        Ok(())
    }
}

mod tests_get_list_products {
    use super::*;

    /*
    set_tax_category(tax_category) +
    .set_after(after) +
    .set_id(id) +
    .set_include(include) +
    .set_order_by(order_by) +
    .set_p_type(p_type) +
    .set_status(status) +
    .set_per_page(per_page) +
     */

    #[tokio::test]
    async fn t_0() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(
                ListProductsParams::default().set_tax_category(vec![ProductTaxCategory::Standard]),
            )
            .await?;

        println!("Get list products response: {:#?}", r);

        assert_eq!(r.data().len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn t_1() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(ListProductsParams::default())
            .await?;

        let uri_with_after = r.meta().pagination.as_ref().unwrap().next.as_str();

        let after = uri_with_after.split("after=").last().unwrap();

        let r = client
            .get_list_products(ListProductsParams::default().set_after(after))
            .await?;

        println!("Get list products response: {:#?}", r);

        assert_eq!(r.data().len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn t_2() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(
                ListProductsParams::default()
                    .set_id(vec![
                        "pro_01j7k6xbf7jctjpnx0pz20th7s".to_string(),
                        "pro_01jcgjpcbfpr4wa8f6zmxwqbng".to_string(),
                    ])
                    .set_include(vec!["prices".to_string(), "X".to_string()]),
            )
            .await?;

        println!("Get list products response: {:#?}", r);

        assert_eq!(r.data().len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn t_3() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(
                ListProductsParams::default()
                    .set_include(vec!["prices".to_string(), "X".to_string()]),
            )
            .await?;

        println!("Get list products response: {:#?}", r);

        assert!(!r.data().is_empty() && r.data().len() > 2);

        Ok(())
    }

    #[tokio::test]
    async fn t_4() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(
                ListProductsParams::default()
                    .set_status(vec![EntityStatus::Archived, EntityStatus::Active])
                    .set_order_by("status[ASC]"),
            )
            .await?;

        if let Some(p) = r.data().first() {
            assert_eq!(p.product().status(), Some(EntityStatus::Active).as_ref());
        } else {
            panic!("No product found");
        }

        let r = client
            .get_list_products(
                ListProductsParams::default()
                    .set_status(vec![EntityStatus::Archived, EntityStatus::Active])
                    .set_order_by("status[DESC]"),
            )
            .await?;

        if let Some(p) = r.data().first() {
            assert_eq!(p.product().status(), Some(EntityStatus::Archived).as_ref());
        } else {
            panic!("No product found");
        }

        Ok(())
    }

    #[tokio::test]
    async fn t_5() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(ListProductsParams::default().set_p_type(EntityType::Custom))
            .await?;

        println!("Get list products response: {:#?}", r);

        assert!(!r.data().is_empty() && r.data().len() >= 2);

        Ok(())
    }

    #[tokio::test]
    async fn t_6() -> Result<(), Box<dyn std::error::Error>> {
        let config = CONFIG.clone();
        let client = Client::new(&config.url, &config.auth)?;
        let r = client
            .get_list_products(ListProductsParams::default().set_per_page(1))
            .await?;

        println!("Get list products response: {:#?}", r);

        assert!(r.data().len() == 1);

        Ok(())
    }
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
