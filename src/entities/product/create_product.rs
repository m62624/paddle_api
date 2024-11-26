use crate::entities::product::{GetProductResponse, ProductData};
use crate::Client;

impl<'a> Client<'a> {
    /// Create a new product.
    /// 
    /// If successful, your response includes a copy of the new product entity.
    /// 
    // https://developer.paddle.com/api-reference/products/create-product
    pub async fn create_product(
        &self,
        product_data: ProductData,
    ) -> Result<GetProductResponse, crate::error::PaddleError> {
        let url = self.url.join("products")?;

        let response = self
            .client
            .post(url)
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
