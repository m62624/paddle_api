use serde::{Deserialize, Serialize};

// https://developer.paddle.com/api-reference/products/overview
#[derive(Serialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductData {
    name: String,
    tax_category: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: String,
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

impl ProductData {
    pub fn new(name: String, tax_category: String, p_type: String) -> Self {
        Self {
            name,
            tax_category: Some(tax_category),
            description: None,
            p_type,
            image_url: None,
            custom_data: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn image_url(mut self, image_url: String) -> Self {
        self.image_url = Some(image_url);
        self
    }

    pub fn custom_data(mut self, custom_data: serde_json::Value) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

impl ProductResponse {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn p_type(&self) -> &str {
        &self.p_type
    }

    pub fn tax_category(&self) -> Option<&str> {
        self.tax_category.as_deref()
    }

    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn custom_data(&self) -> Option<&serde_json::Value> {
        self.custom_data.as_ref()
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn import_meta(&self) -> Option<&serde_json::Value> {
        self.import_meta.as_ref()
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
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
