use serde::Deserialize;

use crate::{
    entities::{
        product::Product, BaseListParams, BaseListParamsGettersSetters, EntityStatus, EntityType,
        Meta,
    },
    error::PaddleError,
    Client,
};

use super::Price;

#[derive(Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListPricesParams {
    #[serde(flatten)]
    base: BaseListParams,
    include: Option<Vec<Product>>,
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

    fn set_id<T: Into<Vec<String>>>(self, id: T) -> Self {
        Self {
            base: BaseListParams {
                id: Some(id.into()),
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

    fn status(&self) -> Option<&Vec<EntityStatus>> {
        self.base.status.as_ref()
    }

    fn set_status(self, status: Vec<EntityStatus>) -> Self {
        Self {
            base: BaseListParams {
                status: Some(status),
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
}

impl ListPricesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include(&self) -> Option<&Vec<Product>> {
        self.include.as_ref()
    }

    pub fn set_include(mut self, include: Vec<Product>) -> Self {
        self.include = Some(include);
        self
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
        let mut url = self.url.join("prices")?;

        if let Some(after) = params.after() {
            url.query_pairs_mut().append_pair("after", &after);
        }

        if let Some(id) = params.id() {
            for i in id {
                url.query_pairs_mut().append_pair("id", &i);
            }
        }

        if let Some(include) = params.include() {
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|p| serde_json::to_string(p).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }

        if let Some(order_by) = params.order_by() {
            url.query_pairs_mut().append_pair("order_by", &order_by);
        }

        if let Some(per_page) = params.per_page() {
            url.query_pairs_mut()
                .append_pair("per_page", &per_page.to_string());
        }

        if let Some(status) = params.status() {
            for s in status {
                url.query_pairs_mut().append_pair("status", &s.to_string());
            }
        }

        if let Some(recurring) = params.recurring() {
            url.query_pairs_mut()
                .append_pair("recurring", &recurring.to_string());
        }

        if let Some(p_type) = params.p_type() {
            url.query_pairs_mut()
                .append_pair("type", &p_type.to_string());
        }

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
