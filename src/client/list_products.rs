use super::Client;
use super::PaddleError;
use crate::entities::product::ProductResponse;
use serde::Deserialize;

// https://developer.paddle.com/api-reference/products/list-products#query-parameters
#[derive(Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsParams {
    after: Option<String>,
    id: Option<Vec<String>>,
    include: Option<Vec<String>>,
    order_by: Option<String>,
    per_page: Option<u32>,
    status: Option<Vec<String>>,
    tax_category: Option<Vec<String>>,
    #[serde(rename = "type")]
    p_type: Option<String>,
}

// https://developer.paddle.com/api-reference/products/list-products#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsResponse {
    data: Vec<ProductResponse>,
    meta: Meta,
}

// https://developer.paddle.com/api-reference/products/list-products#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Meta {
    request_id: String,
    pagination: serde_json::Value,
}

impl ListProductsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn id(mut self, id: Vec<String>) -> Self {
        self.id = Some(id);
        self
    }

    pub fn include(mut self, include: Vec<String>) -> Self {
        self.include = Some(include);
        self
    }

    pub fn order_by(mut self, order_by: String) -> Self {
        self.order_by = Some(order_by);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn status(mut self, status: Vec<String>) -> Self {
        self.status = Some(status);
        self
    }

    pub fn tax_category(mut self, tax_category: Vec<String>) -> Self {
        self.tax_category = Some(tax_category);
        self
    }
    pub fn p_type(mut self, p_type: String) -> Self {
        self.p_type = Some(p_type);
        self
    }
}

impl ListProductsResponse {
    pub fn data(&self) -> &Vec<ProductResponse> {
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

    pub fn pagination(&self) -> &serde_json::Value {
        &self.pagination
    }
}

impl<'a> Client<'a> {
    /// List all products (GET)
    ///
    /// ### Arguments
    /// `params` - list products parameters
    // https://developer.paddle.com/api-reference/products/list-products
    pub async fn list_products(
        &self,
        params: ListProductsParams,
    ) -> Result<ListProductsResponse, PaddleError> {
        let mut url = self.url.join("products")?;

        if let Some(after) = params.after {
            url.query_pairs_mut().append_pair("after", &after);
        }
        if let Some(id) = params.id {
            url.query_pairs_mut().append_pair("id", &id.join(","));
        }
        if let Some(include) = params.include {
            url.query_pairs_mut()
                .append_pair("include", &include.join(","));
        }
        if let Some(order_by) = params.order_by {
            url.query_pairs_mut().append_pair("order_by", &order_by);
        }
        if let Some(per_page) = params.per_page {
            url.query_pairs_mut()
                .append_pair("per_page", &per_page.to_string());
        }
        if let Some(status) = params.status {
            url.query_pairs_mut()
                .append_pair("status", &status.join(","));
        }
        if let Some(tax_category) = params.tax_category {
            url.query_pairs_mut()
                .append_pair("tax_category", &tax_category.join(","));
        }
        if let Some(p_type) = params.p_type {
            url.query_pairs_mut().append_pair("type", &p_type);
        }

        Ok(self
            .client
            .get(url)
            .headers(self.default_headers()?)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
