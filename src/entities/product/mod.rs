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
    product_data: ProductData,
}

/// The request to update a product.
pub struct UpdateProductRequest {
    product_data: ProductData,
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
#[derive(Deserialize, Debug)]
pub struct Meta {
    request_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ProductStatus>,
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
    pub fn new<T: Into<String>>(name: T, tax_category: ProductTaxCategory) -> Self {
        Self {
            product_data: ProductData {
                // required
                name: name.into(),
                // required
                tax_category: Some(tax_category),
                description: None,
                // default
                p_type: Some(ProductType::Standard),
                image_url: None,
                custom_data: None,
                status: None,
            },
        }
    }
}

impl UpdateProductRequest {
    pub fn new(data: ProductData) -> Self {
        Self { product_data: data }
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

    pub fn p_type(&self) -> Option<&ProductType> {
        self.p_type.as_ref()
    }

    pub fn set_type(mut self, p_type: ProductType) -> Self {
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

    pub fn status(&self) -> Option<&ProductStatus> {
        self.status.as_ref()
    }

    pub fn set_status(mut self, status: ProductStatus) -> Self {
        self.status = Some(status);
        self
    }
}

impl ProductResponse {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &ProductData {
        &self.data
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
    pub fn data(&self) -> &ProductData {
        &self.data.data
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

        map.serialize_entry("name", &self.product_data.name)?;

        if let Some(ref tax_category) = self.product_data.tax_category {
            map.serialize_entry("tax_category", tax_category)?;
        }

        if let Some(ref description) = self.product_data.description {
            map.serialize_entry("description", description)?;
        }

        if let Some(ref p_type) = self.product_data.p_type {
            map.serialize_entry("type", p_type)?;
        }

        if let Some(ref image_url) = self.product_data.image_url {
            map.serialize_entry("image_url", image_url)?;
        }

        if let Some(ref custom_data) = self.product_data.custom_data {
            map.serialize_entry("custom_data", custom_data)?;
        }

        if let Some(ref status) = self.product_data.status {
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

impl From<CreateProductRequest> for ProductData {
    fn from(request: CreateProductRequest) -> Self {
        request.product_data
    }
}

impl From<UpdateProductRequest> for ProductData {
    fn from(request: UpdateProductRequest) -> Self {
        request.product_data
    }
}
