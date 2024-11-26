use crate::entities::product::{GetProductResponse, ProductData};
use crate::Client;

impl<'a> Client<'a> {
    /// Update a product by its ID.
    ///
    /// If successful, your response includes a copy of the updated product entity.
    ///
    // https://developer.paddle.com/api-reference/products/update-product
    pub async fn update_product(
        &self,
        product_id: &str,
        product_data: ProductData,
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
