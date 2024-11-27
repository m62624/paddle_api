use std::str::FromStr;

use crate::entities::{
    BaseListParams, BaseListParamsGettersSetters, EntityStatus, EntityType, Meta,
};
use crate::error::PaddleError;
use crate::Client;

use super::{Product, ProductTaxCategory};

use serde::Deserialize;
use serde::Serialize;
use serde_with::formats::CommaSeparator;
use serde_with::{serde_as, StringWithSeparator};

// https://developer.paddle.com/api-reference/products/list-products#query-parameters
#[serde_as]
#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsParams {
    #[serde(flatten)]
    base: BaseListParams,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, ProductTaxCategory>>")]
    tax_category: Option<Vec<ProductTaxCategory>>,
}

#[derive(Deserialize)]
// https://developer.paddle.com/api-reference/products/list-products#response
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListProductsResponse {
    data: Vec<ProductResponseFromList>,
    meta: Meta,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ProductResponseFromList {
    #[serde(flatten)]
    product: Product,
}

impl BaseListParamsGettersSetters for ListProductsParams {
    fn after(&self) -> Option<&str> {
        self.base.after.as_deref()
    }

    fn set_after<T: Into<String>>(self, after: T) -> Self {
        Self {
            base: BaseListParams {
                after: Some(after.into()),
                ..self.base
            },
            ..self
        }
    }

    fn id(&self) -> Option<&Vec<String>> {
        self.base.id.as_ref()
    }

    fn set_id<T, I>(self, id: T) -> Self
    where
        T: IntoIterator<Item = I>,
        I: Into<String>,
    {
        Self {
            base: BaseListParams {
                id: Some(id.into_iter().map(Into::into).collect()),
                ..self.base
            },
            ..self
        }
    }

    fn order_by(&self) -> Option<&str> {
        self.base.order_by.as_deref()
    }

    fn set_order_by<T: Into<String>>(self, order_by: T) -> Self {
        Self {
            base: BaseListParams {
                order_by: Some(order_by.into()),
                ..self.base
            },
            ..self
        }
    }

    fn per_page(&self) -> Option<i32> {
        self.base.per_page
    }

    fn set_per_page<T: Into<i32>>(self, per_page: T) -> Self {
        Self {
            base: BaseListParams {
                per_page: Some(per_page.into()),
                ..self.base
            },
            ..self
        }
    }

    fn status(&self) -> Option<&[EntityStatus]> {
        self.base.status.as_deref()
    }

    fn set_status<T, I>(self, status: T) -> Self
    where
        T: IntoIterator<Item = I>,
        I: Into<EntityStatus>,
    {
        Self {
            base: BaseListParams {
                status: Some(status.into_iter().map(Into::into).collect()),
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
            base: BaseListParams {
                p_type: Some(p_type),
                ..self.base
            },
            ..self
        }
    }

    fn include(&self) -> Option<&[String]> {
        self.base.include.as_deref()
    }

    fn set_include<T, I>(self, include: T) -> Self
    where
        T: IntoIterator<Item = I>,
        I: Into<String>,
    {
        Self {
            base: BaseListParams {
                include: Some(include.into_iter().map(Into::into).collect()),
                ..self.base
            },
            ..self
        }
    }
}

impl ListProductsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tax_category(&self) -> Option<&[ProductTaxCategory]> {
        self.tax_category.as_deref()
    }

    pub fn set_tax_category<T, I>(self, tax_category: T) -> Self
    where
        T: IntoIterator<Item = I>,
        I: Into<ProductTaxCategory>,
    {
        Self {
            tax_category: Some(tax_category.into_iter().map(Into::into).collect()),
            ..self
        }
    }
}

impl ListProductsResponse {
    pub fn data(&self) -> &[ProductResponseFromList] {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl ProductResponseFromList {
    pub fn product(&self) -> &Product {
        &self.product
    }
}

impl Client {
    /// List all products (GET)
    ///
    /// ### Arguments
    /// `params` - list products parameters
    // https://developer.paddle.com/api-reference/products/list-products
    pub async fn get_list_products(
        &self,
        params: ListProductsParams,
    ) -> Result<ListProductsResponse, anyhow::Error> {
        let query = serde_qs::to_string(&params)?;
        let mut url = self.url.join("products")?;

        url.set_query(Some(&query));

        Ok(PaddleError::handle_response(
            self.client
                .get(url)
                .headers(self.default_headers()?)
                .send()
                .await?,
        )
        .await?
        .json()
        .await?)
    }
}

impl From<ListProductsResponse> for (Vec<ProductResponseFromList>, Meta) {
    fn from(r: ListProductsResponse) -> Self {
        (r.data, r.meta)
    }
}

impl From<ProductResponseFromList> for Product {
    fn from(p: ProductResponseFromList) -> Self {
        p.product
    }
}

impl FromStr for ProductTaxCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "digital-goods" => Ok(Self::DigitalGoods),
            "ebooks" => Ok(Self::EBooks),
            "implementation-services" => Ok(Self::ImplementationServices),
            "professional-services" => Ok(Self::ProfessionalServices),
            "saas" => Ok(Self::SaaS),
            "software-programming-services" => Ok(Self::SoftwareProgrammingServices),
            "standard" => Ok(Self::Standard),
            "training-services" => Ok(Self::TrainingServices),
            "website-hosting" => Ok(Self::WebsiteHosting),
            _ => Err("Unknown product tax category".to_string()),
        }
    }
}
