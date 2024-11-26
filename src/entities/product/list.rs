use crate::entities::price::Price;
use crate::entities::BaseListParams;
use crate::entities::BaseListParamsGettersSetters;
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
    #[serde(flatten)]
    base: BaseListParams,
    include: Option<Vec<Price>>,   
    tax_category: Option<Vec<ProductTaxCategory>>,
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

impl BaseListParamsGettersSetters for ListProductsParams{
    fn after(&self) -> Option<&str> {
        self.base.after.as_deref()
    }

    fn set_after<T: Into<String>>(self, after: T) -> Self {
       Self{
           base: BaseListParams{
               after: Some(after.into()),
               ..self.base
           },
           ..self
       }
    }

    fn id(&self) -> Option<&Vec<String>> {
        self.base.id.as_ref()
    }

    fn set_id<T: Into<Vec<String>>>(self, id: T) -> Self {
        Self{
            base: BaseListParams{
                id: Some(id.into()),
                ..self.base
            },
            ..self
        }
    }

    fn order_by(&self) -> Option<&str> {
        self.base.order_by.as_deref()
    }

    fn set_order_by<T: Into<String>>(self, order_by: T) -> Self {
        Self{
            base: BaseListParams{
                order_by: Some(order_by.into()),
                ..self.base
            },
            ..self
        }
    }

    fn per_page(&self) -> Option<i32> {
        self.base.per_page
    }

    fn set_per_page<T: Into<i32>>(self, per_page: T) -> Self {
        Self{
            base: BaseListParams{
                per_page: Some(per_page.into()),
                ..self.base
            },
            ..self
        }
    }

    fn status(&self) -> Option<&Vec<EntityStatus>> {
        self.base.status.as_ref()
    }
    
    fn set_status(self, status: Vec<EntityStatus>) -> Self {
         Self{
            base: BaseListParams{
                status: Some(status),
                ..self.base
            },
            ..self
    }
}
    
    fn p_type(&self) -> Option<&EntityType> {
        self.base.p_type.as_ref()
    }
    
    fn set_p_type(self, p_type: EntityType) -> Self {
        Self{
            base: BaseListParams{
                p_type: Some(p_type),
                ..self.base
            },
            ..self
        }
    }

    
}

impl ListProductsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include(&self) -> Option<&Vec<Price>> {
        self.include.as_ref()
    }

    pub fn set_include(mut self, include: Vec<Price>) -> Self {
        self.include = Some(include);
        self
    }

    pub fn tax_category(&self) -> Option<&Vec<ProductTaxCategory>> {
        self.tax_category.as_ref()
    }

    pub fn set_tax_category(mut self, tax_category: Vec<ProductTaxCategory>) -> Self {
        self.tax_category = Some(tax_category);
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

        if let Some(after) = params.after() {
            url.query_pairs_mut().append_pair("after", &after);
        }
        if let Some(id) = params.id() {
            url.query_pairs_mut().append_pair("id", &id.join(","));
        }
        if let Some(include) = params.include() {
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|p| serde_json::to_string(p).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(order_by) = params.order_by() {
            url.query_pairs_mut().append_pair("order_by", &order_by);
        }
        if let Some(per_page) = params.per_page() {
            url.query_pairs_mut()
                .append_pair("per_page", &per_page.to_string());
        }
        if let Some(status) = params.status() {
            url.query_pairs_mut().append_pair(
                "status",
                &status
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(tax_category) = params.tax_category() {
            url.query_pairs_mut().append_pair(
                "tax_category",
                &tax_category
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(p_type) = params.p_type() {
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