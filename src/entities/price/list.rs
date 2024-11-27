use crate::entities::price::Price;
use crate::Client;
use crate::{
    entities::{BaseListParams, BaseListParamsGettersSetters, EntityStatus, EntityType, Meta},
    error::PaddleError,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListPricesParams {
    #[serde(flatten)]
    base: BaseListParams,
    recurring: Option<bool>,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListPricesResponse {
    data: Vec<PriceResponseFromList>,
    meta: Meta,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct PriceResponseFromList {
    #[serde(flatten)]
    price: Price,
}

impl BaseListParamsGettersSetters for ListPricesParams {
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

impl ListPricesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recurring(&self) -> Option<bool> {
        self.recurring
    }

    pub fn set_recurring(mut self, recurring: bool) -> Self {
        self.recurring = Some(recurring);
        self
    }
}

impl ListPricesResponse {
    pub fn data(&self) -> &Vec<PriceResponseFromList> {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl PriceResponseFromList {
    pub fn price(&self) -> &Price {
        &self.price
    }
}

impl Client {
    /// Get a list of prices (GET).
    pub async fn get_list_prices(
        &self,
        params: ListPricesParams,
    ) -> Result<ListPricesResponse, anyhow::Error> {
        let query = serde_qs::to_string(&params)?;
        let mut url = self.url.join("prices")?;

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
