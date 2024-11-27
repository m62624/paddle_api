pub mod create;
pub mod get;
pub mod list;
pub mod update;

use super::{EntityBase, EntityBaseGettersSetters, EntityStatus, EntityType, Meta};
use crate::entities::price::Price;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response to a successful product creation request.
// https://developer.paddle.com/api-reference/products/get-product#response
#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponse {
    data: Product,
    meta: Meta,
}

/// Product entities describe the items that customers can purchase. They hold high-level product attributes.
// https://developer.paddle.com/api-reference/products/overview
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Product {
    #[serde(flatten)]
    base: EntityBase,
    tax_category: Option<ProductTaxCategory>,
    image_url: Option<String>,
    prices: Option<Vec<Price>>,
}

/// Tax category for this product. Used for charging the correct rate of tax.
/// Selected tax category must be enabled on your Paddle account.
/// [Official document](https://developer.paddle.com/api-reference/products/list-products)
#[derive(Serialize, Deserialize, PartialEq)]
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

impl ProductResponse {
    pub fn data(&self) -> &Product {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl EntityBaseGettersSetters for Product {
    fn id(&self) -> Option<&str> {
        self.base.id.as_deref()
    }

    fn name(&self) -> Option<&str> {
        self.base.name.as_deref()
    }

    fn set_name<T: Into<String>>(self, name: T) -> Self {
        Self {
            base: EntityBase {
                name: Some(name.into()),
                ..self.base
            },
            ..self
        }
    }

    fn description(&self) -> Option<&str> {
        self.base.description.as_deref()
    }

    fn set_description<T: Into<String>>(self, description: T) -> Self {
        Self {
            base: EntityBase {
                description: Some(description.into()),
                ..self.base
            },
            ..self
        }
    }

    fn p_type(&self) -> Option<&EntityType> {
        self.base.p_type.as_ref()
    }

    fn set_p_type(self, p_type: EntityType) -> Self {
        Self {
            base: EntityBase {
                p_type: Some(p_type),
                ..self.base
            },
            ..self
        }
    }

    fn status(&self) -> Option<&EntityStatus> {
        self.base.status.as_ref()
    }

    fn set_status(self, status: EntityStatus) -> Self {
        Self {
            base: EntityBase {
                status: Some(status),
                ..self.base
            },
            ..self
        }
    }

    fn custom_data(&self) -> Option<&serde_json::Value> {
        self.base.custom_data.as_ref()
    }

    fn set_custom_data(self, custom_data: serde_json::Value) -> Self {
        Self {
            base: EntityBase {
                custom_data: Some(custom_data),
                ..self.base
            },
            ..self
        }
    }

    fn import_meta(&self) -> Option<&serde_json::Value> {
        self.base.import_meta.as_ref()
    }

    fn created_at(&self) -> Option<&str> {
        self.base.created_at.as_deref()
    }

    fn updated_at(&self) -> Option<&str> {
        self.base.updated_at.as_deref()
    }
}

impl Product {
    pub fn tax_category(&self) -> Option<&ProductTaxCategory> {
        self.tax_category.as_ref()
    }

    pub fn set_tax_category(mut self, tax_category: ProductTaxCategory) -> Self {
        self.tax_category = Some(tax_category);
        self
    }

    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn set_image_url<T: Into<String>>(mut self, image_url: T) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn prices(&self) -> Option<&Vec<Price>> {
        self.prices.as_ref()
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

impl From<ProductResponse> for (Product, Meta) {
    fn from(response: ProductResponse) -> (Product, Meta) {
        (response.data, response.meta)
    }
}
