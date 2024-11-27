use crate::entities::price::PriceResponse;
use crate::entities::BaseListParamsGettersSetters;
use crate::error::PaddleError;
use crate::Client;

use super::list::ListPricesParams;

impl Client {
    /// Get a single price by its ID (GET).
    pub async fn get_price(
        &self,
        id: &str,
        include: Option<Vec<String>>,
    ) -> Result<PriceResponse, anyhow::Error> {
        let mut url = self.url.join(&format!("prices/{}", id))?;

        // query
        if let Some(include) = include {
            url.set_query(Some(&serde_qs::to_string(
                &ListPricesParams::default().set_include(include),
            )?));
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
