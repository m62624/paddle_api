use crate::entities::{price::PriceResponse, BaseListParamsGettersSetters};
use crate::error::PaddleError;
use crate::Client;

use super::list::ListPricesParams;

impl Client {
    /// Get a single price by its ID (GET).
    pub async fn get_price<T, I>(
        &self,
        id: &str,
        include: Option<T>,
    ) -> Result<PriceResponse, anyhow::Error>
    where
        T: IntoIterator<Item = I>,
        I: Into<String>,
    {
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
