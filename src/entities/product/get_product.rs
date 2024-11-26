use crate::entities::product::GetProductResponse;
use crate::error::PaddleError;
use crate::Client;

impl Client {
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

        let response = PaddleError::handle_response(
            self.client
                .get(url)
                .headers(self.default_headers()?)
                .send()
                .await?,
        )
        .await?
        .json::<GetProductResponse>()
        .await?;

        Ok(response)
    }
}
