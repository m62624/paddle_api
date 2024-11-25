use serde::{Deserialize, Serialize};

// https://developer.paddle.com/api-reference/products/overview
#[derive(Serialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductData {
    name: String,
    description: Option<String>,
    tax_category: Option<String>,
    image_url: Option<String>,
    custom_data: Option<serde_json::Value>,
}

// https://developer.paddle.com/api-reference/products/overview
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponse {
    id: String,
    name: String,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: String,
    tax_category: Option<String>,
    image_url: Option<String>,
    custom_data: Option<serde_json::Value>,
    status: String,
    import_meta: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
}
