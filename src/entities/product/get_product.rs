use crate::entities::product::GetProductResponse;
use crate::Client;

impl<'a> Client<'a> {
    /// Get a single product by its ID (GET).
    ///
    /// [Official document](https://developer.paddle.com/api-reference/products/get-product)
    pub async fn get_product(
        &self,
        id: &str,
        include: Option<Vec<String>>,
    ) -> Result<GetProductResponse, anyhow::Error> {
        let mut url = self.url.join(&format!("products/{}", id))?;

        // query
        if let Some(include) = include {
            url.query_pairs_mut()
                .extend_pairs(include.iter().map(|item| ("include", item)));
        }

        let response = self
            .client
            .get(url)
            .headers(self.default_headers()?)
            .send()
            .await?
            .error_for_status()?
            .json::<GetProductResponse>()
            .await?;

        Ok(response)
    }
}
