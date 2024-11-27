pub mod price;
pub mod product;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub trait EntityBaseGettersSetters {
    fn id(&self) -> Option<&str>;
    fn name(&self) -> Option<&str>;
    fn set_name<T: Into<String>>(self, name: T) -> Self;
    fn description(&self) -> Option<&str>;
    fn set_description<T: Into<String>>(self, description: T) -> Self;
    fn p_type(&self) -> Option<&EntityType>;
    fn set_p_type(self, p_type: EntityType) -> Self;
    fn status(&self) -> Option<&EntityStatus>;
    fn set_status(self, status: EntityStatus) -> Self;
    fn custom_data(&self) -> Option<&serde_json::Value>;
    fn set_custom_data(self, custom_data: serde_json::Value) -> Self;
    fn import_meta(&self) -> Option<&serde_json::Value>;
    fn created_at(&self) -> Option<&str>;
    fn updated_at(&self) -> Option<&str>;
}

pub trait BaseListParamsGettersSetters {
    fn after(&self) -> Option<&str>;
    fn set_after<T: Into<String>>(self, after: T) -> Self;
    fn id(&self) -> Option<&Vec<String>>;
    fn set_id<T: Into<Vec<String>>>(self, id: T) -> Self;
    fn order_by(&self) -> Option<&str>;
    fn set_order_by<T: Into<String>>(self, order_by: T) -> Self;
    fn per_page(&self) -> Option<i32>;
    fn set_per_page<T: Into<i32>>(self, per_page: T) -> Self;
    fn status(&self) -> Option<&Vec<EntityStatus>>;
    fn set_status(self, status: Vec<EntityStatus>) -> Self;
    fn p_type(&self) -> Option<&EntityType>;
    fn set_p_type(self, p_type: EntityType) -> Self;
}

/// The base entity object contains common attributes for all entities.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct EntityBase {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: Option<EntityType>,
    status: Option<EntityStatus>,
    custom_data: Option<serde_json::Value>,
    import_meta: Option<serde_json::Value>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct BaseListParams {
    after: Option<String>,
    id: Option<Vec<String>>,
    order_by: Option<String>,
    per_page: Option<i32>,
    status: Option<Vec<EntityStatus>>,
    #[serde(rename = "type")]
    p_type: Option<EntityType>,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum EntityStatus {
    #[default]
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "archived")]
    Archived,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum EntityType {
    #[serde(rename = "custom")]
    Custom,
    #[default]
    #[serde(rename = "standard")]
    Standard,
}

/// The meta object contains additional information about the request.
// https://developer.paddle.com/api-reference/products/list-products#response
#[derive(Deserialize, Debug)]
pub struct Meta {
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub per_page: u32,
    pub next: String,
    pub has_more: bool,
    pub estimated_total: u32,
}

impl Meta {
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Custom => write!(f, "custom"),
            Self::Standard => write!(f, "standard"),
        }
    }
}
