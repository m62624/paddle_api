pub mod check_auth;
pub mod create_product;
pub mod get_product;
pub mod list_products;
pub mod update_product;

#[cfg(test)]
mod unit_tests;

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

/// The request to create a new product.
#[derive(Serialize)]
pub struct CreateProductRequest {
    #[serde(flatten)]
    pub data: ProductData,
}

/// The request to update a product.
pub struct UpdateProductRequest {
    pub data: ProductData,
}

/// Product entities describe the items that customers can purchase. They hold high-level product attributes.
// https://developer.paddle.com/api-reference/products/overview
#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductData {
    /// Name of this product.
    name: String,
    tax_category: Option<ProductTaxCategory>,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: Option<ProductType>,
    image_url: Option<String>,
    custom_data: Option<serde_json::Value>,
    status: Option<ProductStatus>,
}

/// The response to a successful product creation request.
// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct GetProductResponse {
    data: ProductResponse,
    meta: Meta,
}
/// The response to a successful list products request.
// https://developer.paddle.com/api-reference/products/overview
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponse {
    id: String,
    #[serde(flatten)]
    data: ProductData,
    import_meta: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
}

/// The meta object contains additional information about the request.
// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Meta {
    request_id: String,
}

/// Tax category for this product. Used for charging the correct rate of tax.
/// Selected tax category must be enabled on your Paddle account.
/// [Official document](https://developer.paddle.com/api-reference/products/list-products)
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

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum ProductType {
    #[serde(rename = "custom")]
    Custom,
    #[default]
    #[serde(rename = "standard")]
    Standard,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum ProductStatus {
    #[default]
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "archived")]
    Archived,
}

impl CreateProductRequest {
    pub fn new(data: ProductData) -> Self {
        Self { data }
    }
}

impl UpdateProductRequest {
    pub fn new(data: ProductData) -> Self {
        Self { data }
    }
}

impl ProductData {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            tax_category: None,
            description: None,
            p_type: None,
            image_url: None,
            custom_data: None,
            status: None,
        }
    }

    pub fn set_description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn set_image_url<T: Into<String>>(mut self, image_url: T) -> Self {
        self.image_url = Some(image_url.into());
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

    pub fn get_data(&self) -> &ProductData {
        &self.data
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

impl Serialize for UpdateProductRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        /*
        name: String,
        tax_category: Option<ProductTaxCategory>,
        description: Option<String>,
        #[serde(rename = "type")]
        p_type: Option<ProductType>,
        image_url: Option<String>,
        custom_data: Option<serde_json::Value>,
        status: Option<ProductStatus>,
             */
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("name", &self.data.name)?;

        if let Some(ref tax_category) = self.data.tax_category {
            map.serialize_entry("tax_category", tax_category)?;
        }

        if let Some(ref description) = self.data.description {
            map.serialize_entry("description", description)?;
        }

        if let Some(ref p_type) = self.data.p_type {
            map.serialize_entry("type", p_type)?;
        }

        if let Some(ref image_url) = self.data.image_url {
            map.serialize_entry("image_url", image_url)?;
        }

        if let Some(ref custom_data) = self.data.custom_data {
            map.serialize_entry("custom_data", custom_data)?;
        }

        if let Some(ref status) = self.data.status {
            map.serialize_entry("status", status)?;
        }

        map.end()
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
