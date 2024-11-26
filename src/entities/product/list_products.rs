use crate::entities::product::ProductResponse;
use crate::error::PaddleError;
use crate::Client;

use serde::Deserialize;

use super::ProductStatus;
use super::ProductTaxCategory;
use super::ProductType;

// https://developer.paddle.com/api-reference/products/list-products#query-parameters
#[derive(Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsParams {
    after: Option<String>,
    id: Option<Vec<String>>,
    include: Option<Vec<String>>,
    order_by: Option<String>,
    per_page: Option<u32>,
    status: Option<Vec<ProductStatus>>,
    tax_category: Option<Vec<ProductTaxCategory>>,
    #[serde(rename = "type")]
    p_type: Option<ProductType>,
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

    pub fn after(&self) -> Option<&str> {
        self.after.as_deref()
    }

    pub fn set_after<T: Into<String>>(mut self, after: T) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn id(&self) -> Option<&Vec<String>> {
        self.id.as_ref()
    }

    pub fn set_id<T: Into<String>>(mut self, id: Vec<T>) -> Self {
        self.id = Some(id.into_iter().map(Into::into).collect());
        self
    }

    pub fn include(&self) -> Option<&Vec<String>> {
        self.include.as_ref()
    }

    pub fn set_include<T: Into<String>>(mut self, include: Vec<T>) -> Self {
        self.include = Some(include.into_iter().map(Into::into).collect());
        self
    }

    pub fn order_by(&self) -> Option<&str> {
        self.order_by.as_deref()
    }

    pub fn set_order_by<T: Into<String>>(mut self, order_by: T) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn per_page(&self) -> Option<u32> {
        self.per_page
    }

    pub fn set_per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn status(&self) -> Option<&Vec<ProductStatus>> {
        self.status.as_ref()
    }

    pub fn set_status(mut self, status: Vec<ProductStatus>) -> Self {
        self.status = Some(status);
        self
    }

    pub fn tax_category(&self) -> Option<&Vec<ProductTaxCategory>> {
        self.tax_category.as_ref()
    }

    pub fn set_tax_category(mut self, tax_category: Vec<ProductTaxCategory>) -> Self {
        self.tax_category = Some(tax_category);
        self
    }

    pub fn p_type(&self) -> Option<&ProductType> {
        self.p_type.as_ref()
    }

    pub fn set_p_type(mut self, p_type: ProductType) -> Self {
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

impl Client {
    /// List all products (GET)
    ///
    /// ### Arguments
    /// `params` - list products parameters
    // https://developer.paddle.com/api-reference/products/list-products
    pub async fn get_list_products(
        &self,
        params: ListProductsParams,
    ) -> Result<ListProductsResponse, anyhow::Error> {
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
            url.query_pairs_mut().append_pair(
                "status",
                &status
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(tax_category) = params.tax_category {
            url.query_pairs_mut().append_pair(
                "tax_category",
                &tax_category
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(p_type) = params.p_type {
            url.query_pairs_mut()
                .append_pair("type", &p_type.to_string());
        }

        Ok(PaddleError::handle_response(
            self.client
                .get(url)
                .headers(self.default_headers()?)
                .send()
                .await?,
        )
        .await?
        .json()
        .await?)
    }
}
