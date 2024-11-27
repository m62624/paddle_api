use crate::entities::product::{list::ListProductsParams, ProductResponse};
use crate::entities::BaseListParamsGettersSetters;
use crate::error::PaddleError;
use crate::Client;

impl Client {
    /// Get a single product by its ID (GET).
    ///
    /// [Official document](https://developer.paddle.com/api-reference/products/get-product)
    pub async fn get_product<T, I>(
        &self,
        id: &str,
        include: Option<T>,
    ) -> Result<ProductResponse, anyhow::Error>
    where
        T: IntoIterator<Item = I>,
        I: Into<String>,
    {
        let mut url = self.url.join(&format!("products/{}", id))?;

        // query
        if let Some(include) = include {
            url.set_query(Some(&serde_qs::to_string(
                &ListProductsParams::default().set_include(include),
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
