use crate::entities::product::ProductResponse;
use crate::error::PaddleError;
use crate::Client;

use super::Product;

impl Client {
    /// Update a product by its ID (PATCH).
    ///
    /// If successful, your response includes a copy of the updated product entity.
    ///
    /// [Official document](https://developer.paddle.com/api-reference/products/update-product)
    pub async fn update_product(
        &self,
        product_id: &str,
        product_data: Product,
    ) -> Result<ProductResponse, anyhow::Error> {
        let url = self.url.join(&format!("products/{}", product_id))?;

        let response = PaddleError::handle_response(
            self.client
                .patch(url)
                .headers(self.default_headers()?)
                .json(&product_data)
                .send()
                .await?,
        )
        .await?
        .json::<ProductResponse>()
        .await?;

        Ok(response)
    }
}
