use super::Client;
use crate::entities::product::GetProductResponse;

impl<'a> Client<'a> {
    /// Get a single product by its ID.
    ///
    /// https://developer.paddle.com/api-reference/product-api/products/getproduct
    pub async fn get_product(
        &self,
        id: &str,
        include: Option<Vec<String>>,
    ) -> Result<GetProductResponse, crate::error::PaddleError> {
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
