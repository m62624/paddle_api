pub mod check_auth;
pub mod create_product;
pub mod get_product;
pub mod list_products;
pub mod update_product;

#[cfg(test)]
mod unit_tests;

use serde::{Deserialize, Serialize};

// https://developer.paddle.com/api-reference/products/overview
#[derive(Serialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductData {
    name: String,
    tax_category: Option<ProductTaxCategory>,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: ProductType,
    image_url: Option<String>,
    custom_data: Option<serde_json::Value>,
    status: Option<ProductStatus>,
}

// https://developer.paddle.com/api-reference/products/overview
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponse {
    id: String,
    name: String,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: ProductType,
    tax_category: Option<ProductTaxCategory>,
    image_url: Option<String>,
    custom_data: Option<serde_json::Value>,
    status: ProductStatus,
    import_meta: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
}

// https://developer.paddle.com/api-reference/products/list-products
#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum ProductTaxCategory {
    #[serde(rename = "digital-goods")]
    DigitalGoods,
    #[serde(rename = "ebooks")]
    EBooks,
    #[serde(rename = "implementation-services")]
    ImplementationServices,
    #[serde(rename = "professional-services")]
    ProfessionalServices,
    #[serde(rename = "saas")]
    SaaS,
    #[serde(rename = "software-programming-services")]
    SoftwareProgrammingServices,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "training-services")]
    TrainingServices,
    #[serde(rename = "website-hosting")]
    WebsiteHosting,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum ProductType {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "standard")]
    Standard,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum ProductStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "archived")]
    Archived,
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
    pub fn new(name: String, tax_category: ProductTaxCategory) -> Self {
        Self {
            name,
            tax_category: Some(tax_category),
            description: None,
            p_type: ProductType::Standard,
            image_url: None,
            custom_data: None,
            status: Some(ProductStatus::Active),
        }
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn set_image_url(mut self, image_url: String) -> Self {
        self.image_url = Some(image_url);
        self
    }

    pub fn set_custom_data(mut self, custom_data: serde_json::Value) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    pub fn set_status(mut self, status: ProductStatus) -> Self {
        self.status = Some(status);
        self
    }
}

impl ProductResponse {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_p_type(&self) -> &ProductType {
        &self.p_type
    }

    pub fn get_tax_category(&self) -> Option<&ProductTaxCategory> {
        self.tax_category.as_ref()
    }

    pub fn get_image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn get_custom_data(&self) -> Option<&serde_json::Value> {
        self.custom_data.as_ref()
    }

    pub fn get_status(&self) -> &ProductStatus {
        &self.status
    }

    pub fn get_import_meta(&self) -> Option<&serde_json::Value> {
        self.import_meta.as_ref()
    }

    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }

    pub fn get_updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl GetProductResponse {
    pub fn get_data(&self) -> &ProductResponse {
        &self.data
    }

    pub fn get_meta(&self) -> &Meta {
        &self.meta
    }
}

impl Meta {
    pub fn get_request_id(&self) -> &str {
        &self.request_id
    }
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Custom => write!(f, "custom"),
            Self::Standard => write!(f, "standard"),
        }
    }
}

impl std::fmt::Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl std::fmt::Display for ProductTaxCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DigitalGoods => write!(f, "digital-goods"),
            Self::EBooks => write!(f, "ebooks"),
            Self::ImplementationServices => write!(f, "implementation-services"),
            Self::ProfessionalServices => write!(f, "professional-services"),
            Self::SaaS => write!(f, "saas"),
            Self::SoftwareProgrammingServices => write!(f, "software-programming-services"),
            Self::Standard => write!(f, "standard"),
            Self::TrainingServices => write!(f, "training-services"),
            Self::WebsiteHosting => write!(f, "website-hosting"),
        }
    }
}
