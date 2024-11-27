pub mod price;
pub mod product;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The base entity object contains common attributes for all entities.
pub trait EntityBaseGettersSetters {
    /// Unique Paddle ID
    fn id(&self) -> Option<&str>;
    /// Name of the entity
    fn name(&self) -> Option<&str>;
    /// Set the name of the entity
    fn set_name<T: Into<String>>(self, name: T) -> Self;
    /// Description of the entity
    fn description(&self) -> Option<&str>;
    /// Set the description of the entity
    fn set_description<T: Into<String>>(self, description: T) -> Self;
    /// Type of the entity
    fn p_type(&self) -> Option<&EntityType>;
    /// Set the type of the entity
    fn set_p_type(self, p_type: EntityType) -> Self;
    /// Status of the entity
    fn status(&self) -> Option<&EntityStatus>;
    /// Set the status of the entity
    fn set_status(self, status: EntityStatus) -> Self;
    /// Custom data of the entity. Your own structured key-value data.
    fn custom_data(&self) -> Option<&serde_json::Value>;
    /// Set the custom data of the entity. Your own structured key-value data.
    fn set_custom_data(self, custom_data: serde_json::Value) -> Self;
    /// Import meta of the entity
    fn import_meta(&self) -> Option<&serde_json::Value>;
    /// Set the import meta of the entity
    fn created_at(&self) -> Option<&str>;
    /// Set the created at of the entity
    fn updated_at(&self) -> Option<&str>;
}

/// The base list params object contains common attributes for all list params.
pub trait BaseListParamsGettersSetters {
    /// Return entities after the specified Paddle ID when working with paginated endpoints.
    fn after(&self) -> Option<&str>;
    /// Set the after of the list params
    /// Return entities after the specified Paddle ID when working with paginated endpoints.
    /// Used in the `meta.pagination.next` URL in responses for list operations.
    fn set_after<T: Into<String>>(self, after: T) -> Self;
    /// Return entities with the specified Paddle IDs.
    fn id(&self) -> Option<&Vec<String>>;
    /// Set the id of the list params
    fn set_id<T: Into<Vec<String>>>(self, id: T) -> Self;
    /// Order returned entities by the specified field and direction ([ASC] or [DESC]).
    fn order_by(&self) -> Option<&str>;
    /// Order returned entities by the specified field and direction ([ASC] or [DESC]).
    /// For example, ?order_by=id[ASC].
    /// ### Product
    /// Valid fields for ordering:
    ///
    /// `created_at, custom_data, description, id, image_url, name, status, tax_category, and updated_at`.
    ///
    /// ### Price
    /// Valid fields for ordering:
    /// `billing_cycle.frequency, billing_cycle.interval, id, product_id,
    ///  quantity.maximum, quantity.minimum, status, tax_mode, unit_price.amount, and unit_price.currency_code.`
    fn set_order_by<T: Into<String>>(self, order_by: T) -> Self;
    /// Return how many entities are returned per page.
    fn per_page(&self) -> Option<i32>;
    /// Set how many entities are returned per page.\
    /// Paddle returns the maximum number of results if
    /// a number greater than the maximum is requested.\
    /// Check meta.pagination.per_page in the response to see how many were returned.\
    /// `Default: 50; Maximum: 200`.
    fn set_per_page<T: Into<i32>>(self, per_page: T) -> Self;
    /// Return entities with the specified status.
    fn status(&self) -> Option<&Vec<EntityStatus>>;
    /// Set the status of the list params
    fn set_status(self, status: Vec<EntityStatus>) -> Self;
    /// Return entities with the specified type.
    fn p_type(&self) -> Option<&EntityType>;
    /// Set the type of the list params
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

/// The base list params object contains common attributes for all list params.
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
    /// Entity is active and can be used.
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Entity is archived, so can't be used.
    #[serde(rename = "archived")]
    Archived,
}

/// Type of item. Standard items are considered part of your catalog
/// and are shown on the Paddle web app.
#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum EntityType {
    /// Non-catalog item. Typically created for a specific transaction or\
    /// subscription. Not returned when listing or shown in the Paddle web app.
    #[serde(rename = "custom")]
    Custom,
    /// Standard item. Can be considered part of your catalog
    /// and reused across transactions and subscriptions easily.
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
