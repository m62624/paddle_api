use crate::entities::price::Price;
use crate::entities::Meta;
use crate::error::PaddleError;
use crate::Client;

use serde::Deserialize;

use super::EntityStatus;
use super::Product;
use super::ProductTaxCategory;
use crate::entities::EntityType;

// https://developer.paddle.com/api-reference/products/list-products#query-parameters
#[derive(Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsParams {
    after: Option<String>,
    id: Option<Vec<String>>,
    include: Option<Vec<Price>>,
    order_by: Option<String>,
    per_page: Option<i32>,
    status: Option<Vec<EntityStatus>>,
    tax_category: Option<Vec<ProductTaxCategory>>,
    #[serde(rename = "type")]
    p_type: Option<EntityType>,
}

#[derive(Deserialize)]
// https://developer.paddle.com/api-reference/products/list-products#response
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsResponse {
    data: Vec<ProductResponseFromList>,
    meta: Meta,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponseFromList {
    #[serde(flatten)]
    product: Product,
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

    pub fn include(&self) -> Option<&Vec<Price>> {
        self.include.as_ref()
    }

    pub fn set_include(mut self, include: Vec<Price>) -> Self {
        self.include = Some(include);
        self
    }

    pub fn order_by(&self) -> Option<&str> {
        self.order_by.as_deref()
    }

    pub fn set_order_by<T: Into<String>>(mut self, order_by: T) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn per_page(&self) -> Option<i32> {
        self.per_page
    }

    pub fn set_per_page<T: Into<i32>>(mut self, per_page: T) -> Self{
        self.per_page = Some(per_page.into());
        self
    }

    pub fn status(&self) -> Option<&Vec<EntityStatus>> {
        self.status.as_ref()
    }

    pub fn set_status(mut self, status: Vec<EntityStatus>) -> Self {
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

    pub fn p_type(&self) -> Option<&EntityType> {
        self.p_type.as_ref()
    }

    pub fn set_p_type(mut self, p_type: EntityType) -> Self {
        self.p_type = Some(p_type);
        self
    }
}


impl ListProductsResponse {
    pub fn data(&self) -> &Vec<ProductResponseFromList> {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl ProductResponseFromList {
    pub fn product(&self) -> &Product {
        &self.product
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
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|p| serde_json::to_string(p).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join(","),
            );
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

impl From<ListProductsResponse> for (Vec<ProductResponseFromList>, Meta) {
    fn from(r: ListProductsResponse) -> Self {
        (r.data, r.meta)
    }
}

impl From<ProductResponseFromList> for Product {
    fn from(p: ProductResponseFromList) -> Self {
        p.product
    }
}