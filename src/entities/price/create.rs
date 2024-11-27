use crate::entities::price::PriceResponse;
use crate::error::PaddleError;
use crate::Client;

use super::CreatePriceRequest;

impl Client {
    /// Create a new price (POST).
    ///
    /// If successful, your response includes a copy of the new price entity.
    pub async fn create_price(
        &self,
        price_data: CreatePriceRequest,
    ) -> Result<PriceResponse, anyhow::Error> {
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
