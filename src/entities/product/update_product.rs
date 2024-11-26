use crate::entities::product::GetProductResponse;
use crate::Client;

use super::UpdateProductRequest;

impl Client {
    /// Update a product by its ID (PATCH).
    ///
    /// If successful, your response includes a copy of the updated product entity.
    ///
    /// [Official document](https://developer.paddle.com/api-reference/products/update-product)
    pub async fn update_product(
        &self,
        product_id: &str,
        product_data: UpdateProductRequest,
    ) -> Result<GetProductResponse, anyhow::Error> {
        let url = self.url.join(&format!("products/{}", product_id))?;

        let response = self
            .client
            .patch(url)
            .headers(self.default_headers()?)
            .json(&product_data)
            .send()
            .await?
            .error_for_status()?
            .json::<GetProductResponse>()
            .await?;

        Ok(response)
    }
}
