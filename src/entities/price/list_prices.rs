use serde::Deserialize;

use crate::{entities::{product::Product, EntityStatus, EntityType, Meta}, error::PaddleError, Client};

use super::Price;


#[derive(Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListPriceParams{
    after: Option<String>,
    id: Option<Vec<String>>,
    include: Option<Vec<Product>>,
    order_by: Option<String>,
    per_page: Option<i32>,
    status: Option<Vec<EntityStatus>>,
    recurring: Option<bool>,
    #[serde(rename = "type")]
    p_type: Option<EntityType>,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ListPricesResponse{
    data: Vec<PriceResponseFromList>,
    meta: Meta,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct PriceResponseFromList{
    #[serde(flatten)]
    price: Price,
}

impl ListPriceParams{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn after(&self) -> Option<&str>{
        self.after.as_deref()
    }

    pub fn set_after<T: Into<String>>(mut self, after: T) -> Self{
        self.after = Some(after.into());
        self
    }

    pub fn id(&self) -> Option<&Vec<String>>{
        self.id.as_ref()
    }

    pub fn set_id(mut self, id: Vec<String>) -> Self{
        self.id = Some(id);
        self
    }

    pub fn include(&self) -> Option<&Vec<Product>>{
        self.include.as_ref()
    }

    pub fn set_include(mut self, include: Vec<Product>) -> Self{
        self.include = Some(include);
        self
    }

    pub fn order_by(&self) -> Option<&str>{
        self.order_by.as_deref()
    }

    pub fn set_order_by<T: Into<String>>(mut self, order_by: T) -> Self{
        self.order_by = Some(order_by.into());
        self
    }

    pub fn per_page(&self) -> Option<i32>{
        self.per_page
    }

    pub fn set_per_page<T: Into<i32>>(mut self, per_page: T) -> Self{
        self.per_page = Some(per_page.into());
        self
    }

    pub fn status(&self) -> Option<&Vec<EntityStatus>>{
        self.status.as_ref()
    }

    pub fn set_status(mut self, status: Vec<EntityStatus>) -> Self{
        self.status = Some(status);
        self
    }

    pub fn recurring(&self) -> Option<bool>{
        self.recurring
    }

    pub fn set_recurring(mut self, recurring: bool) -> Self{
        self.recurring = Some(recurring);
        self
    }

    pub fn p_type(&self) -> Option<&EntityType>{
        self.p_type.as_ref()
    }

    pub fn set_p_type(mut self, p_type: EntityType) -> Self{
        self.p_type = Some(p_type);
        self
    }
}

impl ListPricesResponse{
    pub fn data(&self) -> &Vec<PriceResponseFromList>{
        &self.data
    }

    pub fn meta(&self) -> &Meta{
        &self.meta
    }
}

impl PriceResponseFromList{
    pub fn price(&self) -> &Price{
        &self.price
    }
}

impl Client{
    pub async fn get_list_prices(&self, params: ListPriceParams) -> Result<ListPricesResponse, anyhow::Error>{

        let mut url = self.url.join("prices")?;

        if let Some(after) = params.after{
            url.query_pairs_mut().append_pair("after", &after);
        }


        if let Some(id) = params.id{
            for i in id{
                url.query_pairs_mut().append_pair("id", &i);
            }
        }

        if let Some(include) = params.include{
            url.query_pairs_mut().append_pair(
                "include",
                &include
                    .iter()
                    .map(|p| serde_json::to_string(p).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }

        if let Some(order_by) = params.order_by{
            url.query_pairs_mut().append_pair("order_by", &order_by);
        }

        if let Some(per_page) = params.per_page{
            url.query_pairs_mut().append_pair("per_page", &per_page.to_string());
        }

        if let Some(status) = params.status{
            for s in status{
                url.query_pairs_mut().append_pair("status", &s.to_string());
            }
        }

        if let Some(recurring) = params.recurring{
            url.query_pairs_mut().append_pair("recurring", &recurring.to_string());
        }

        if let Some(p_type) = params.p_type{
            url.query_pairs_mut().append_pair("type", &p_type.to_string());
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