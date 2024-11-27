use crate::entities::price::PriceResponse;
use crate::entities::product::Product;
use crate::error::PaddleError;
use crate::Client;

impl Client {
    /// Get a single price by its ID (GET).
    pub async fn get_price(
        &self,
        id: &str,
        include: Option<Vec<Product>>,
    ) -> Result<PriceResponse, anyhow::Error> {
        let mut url = self.url.join(&format!("prices/{}", id))?;

        if let Some(include) = include {
            url.query_pairs_mut().extend_pairs(
                include
                    .iter()
                    .map(|item| ("include", serde_json::to_string(&item).unwrap_or_default())),
            );
        }

        let response = PaddleError::handle_response(
            self.client
                .get(url)
                .headers(self.default_headers()?)
                .send()
                .await?,
        )
        .await?
        .json()
        .await?;

        Ok(response)
    }
}
