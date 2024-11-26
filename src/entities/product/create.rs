use crate::entities::product::ProductResponse;
use crate::error::PaddleError;
use crate::Client;

use super::CreateProductRequest;

impl Client {
    /// Create a new product (POST).
    ///
    /// If successful, your response includes a copy of the new product entity.
    ///
    /// [Official document](https://developer.paddle.com/api-reference/products/create-product)
    pub async fn create_product(
        &self,
        product_data: CreateProductRequest,
    ) -> Result<ProductResponse, anyhow::Error> {
        let url = self.url.join("products")?;

        let response = PaddleError::handle_response(
            self.client
                .post(url)
                .headers(self.default_headers()?)
                .json(&product_data)
                .send()
                .await?,
        )
        .await?
        .json()
        .await?;

        Ok(response)
    }
}
