use crate::entities::price::PriceResponse;
use crate::error::PaddleError;
use crate::Client;

use super::Price;

impl Client {
    /// Update the price of a product (POST).
    ///
    /// If successful, your response includes a copy of the updated price entity.
    ///
    /// [Official document](https://developer.paddle.com/api-reference/prices/update-price)
    pub async fn update_price(
        &self,
        price_id: &str,
        price_data: Price,
    ) -> Result<PriceResponse, anyhow::Error> {
        let url = self.url.join(&format!("prices/{}", price_id))?;

        let response = PaddleError::handle_response(
            self.client
                .patch(url)
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
