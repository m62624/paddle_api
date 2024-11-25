use crate::entities::product::ProductResponse;
use serde::Deserialize;

use super::Client;

// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct GetProductResponse {
    data: ProductResponse,
    meta: Meta,
}

// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Meta {
    request_id: String,
}

impl GetProductResponse {
    pub fn data(&self) -> &ProductResponse {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl Meta {
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
}

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
