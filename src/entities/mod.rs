pub mod price;
pub mod product;

use serde::{Deserialize, Serialize};

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
// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize, Debug)]
pub struct Meta {
    request_id: String,
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
