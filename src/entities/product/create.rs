use crate::entities::product::ProductResponse;
use crate::error::PaddleError;
use crate::Client;

use super::Product;

impl Client {
    /// Create a new product (POST).
    ///
    /// **to create a product, the following attributes are required** :
    /// - `name`: the name of the product.
    /// - `tax_category`: the tax category of the product.
    ///
    /// If successful, your response includes a copy of the new product entity.
    ///
    /// [Official document](https://developer.paddle.com/api-reference/products/create-product)
    pub async fn create_product(
        &self,
        product_data: Product,
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
