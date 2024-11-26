pub mod check_auth;
pub mod create_product;
pub mod get_product;
pub mod list_products;
pub mod update_product;

use serde_with::skip_serializing_none;

#[cfg(test)]
mod unit_tests;

use serde::{Deserialize, Serialize};

use super::price::PriceData;
use super::{EntityStatus, EntityType, Meta};

/// The request to create a new product.
#[derive(Serialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct CreateProductRequest {
    #[serde(flatten)]
    product_data: Product,
}

/// The response to a successful product creation request.
// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponse {
    pub data: Product,
    pub meta: Meta,
}

/// Product entities describe the items that customers can purchase. They hold high-level product attributes.
// https://developer.paddle.com/api-reference/products/overview
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Product {
    id: Option<String>,
    name: String,
    description: Option<String>,
    #[serde(rename = "type")]
    p_type: Option<super::EntityType>,
    tax_category: Option<ProductTaxCategory>,
    image_url: Option<String>,
    status: Option<EntityStatus>,
    custom_data: Option<serde_json::Value>,
    import_meta: Option<serde_json::Value>,
    created_at: Option<String>,
    updated_at: Option<String>,
    prices: Option<PriceData>,
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

impl CreateProductRequest {
    pub fn new<T: Into<String>>(name: T, tax_category: ProductTaxCategory) -> Self {
        Self {
            product_data: Product {
                // required
                name: name.into(),
                // required
                tax_category: Some(tax_category),
                description: None,
                // default
                p_type: Some(EntityType::Standard),
                ..Default::default()
            },
        }
    }
}

impl Product {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn tax_category(&self) -> Option<&ProductTaxCategory> {
        self.tax_category.as_ref()
    }

    pub fn set_tax_category(mut self, tax_category: ProductTaxCategory) -> Self {
        self.tax_category = Some(tax_category);
        self
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn p_type(&self) -> Option<&EntityType> {
        self.p_type.as_ref()
    }

    pub fn set_type(mut self, p_type: EntityType) -> Self {
        self.p_type = Some(p_type);
        self
    }

    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn set_image_url<T: Into<String>>(mut self, image_url: T) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn custom_data(&self) -> Option<&serde_json::Value> {
        self.custom_data.as_ref()
    }

    pub fn set_custom_data(mut self, custom_data: serde_json::Value) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    pub fn status(&self) -> Option<&EntityStatus> {
        self.status.as_ref()
    }

    pub fn set_status(mut self, status: EntityStatus) -> Self {
        self.status = Some(status);
        self
    }
}

impl std::fmt::Display for EntityStatus {
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

impl From<CreateProductRequest> for Product {
    fn from(request: CreateProductRequest) -> Self {
        request.product_data
    }
}
