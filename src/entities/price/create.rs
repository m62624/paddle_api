use crate::entities::price::PriceResponse;
use crate::error::PaddleError;
use crate::Client;

use super::Price;

impl Client {
    /// Create a new price (POST).
    ///
    /// **to create a price, the following attributes are required** :
    /// - `description`: the description of the price.
    /// - `product_id`: the ID of the product.
    /// - `unit_price`: the unit price of the product.
    /// 
    /// If successful, your response includes a copy of the new price entity.
    pub async fn create_price(&self, price_data: Price) -> Result<PriceResponse, anyhow::Error> {
        let url = self.url.join("prices")?;

        let response = PaddleError::handle_response(
            self.client
                .post(url)
                .headers(self.default_headers()?)
                .json(&price_data)
                .send()
                .await?,
        )
        .await?
        .json()
        .await?;

        Ok(response)
    }
}
