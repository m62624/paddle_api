pub mod create_price;
pub mod get_price;
pub mod list_prices;
pub mod update_price;

use super::{EntityStatus, EntityType, Meta};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct CreatePriceRequest {
    #[serde(flatten)]
    product_data: Price,
}

#[derive(Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct PriceResponse {
    pub data: Price,
    pub meta: Meta,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Price {
    id: Option<String>,
    product_id: Option<String>,
    description: Option<String>,
    unit_price: Option<UnitPrice>,
    #[serde(rename = "type")]
    p_type: Option<EntityType>,
    name: Option<String>,
    billing_cycle: Option<BillingCycle>,
    trial_period: Option<TrialPeriod>,
    tax_mode: Option<TaxMode>,
    unit_price_overrides: Option<Vec<UnitPriceOverride>>,
    quantity: Option<Quantity>,
    status: Option<EntityStatus>,
    custom_data: Option<serde_json::Value>,
    #[serde(rename = "import_meta")]
    import_meta: Option<ImportMeta>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct BillingCycle {

    frequency: i32,
    interval: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct TrialPeriod {
    frequency: i32,
    interval: PriceInterval,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum PriceInterval {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub enum TaxMode {
    #[default]
    #[serde(rename = "account_setting")]
    AccountSetting,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "internal")]
    Internal,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct UnitPrice {
    amount: String,
    currency_code: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct UnitPriceOverride {
    country_codes: Vec<String>,
    unit_price: UnitPrice,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct Quantity {
    minimum: i32,
    maximum: i32,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(any(feature = "debug", feature = "logs", test), derive(Debug))]
pub struct ImportMeta {
    imported_from: String,
    external_id: Option<String>,
}

impl CreatePriceRequest {
    pub fn new<T: Into<String>>(description: T, product_id: String, unit_price: UnitPrice) -> Self {
        Self {
            product_data: Price {
                description: Some(description.into()),
                product_id: Some(product_id),
                unit_price: Some(unit_price),
                p_type: Some(EntityType::Standard),
                status: Some(EntityStatus::Active),
                tax_mode: Some(TaxMode::AccountSetting),
                ..Default::default()
            },
        }
    }
}

impl PriceResponse {
    pub fn data(&self) -> &Price {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl Price {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn product_id(&self) -> Option<&str> {
        self.product_id.as_deref()
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

    pub fn set_p_type(mut self, p_type: EntityType) -> Self {
        self.p_type = Some(p_type);
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn billing_cycle(&self) -> Option<&BillingCycle> {
        self.billing_cycle.as_ref()
    }

    pub fn set_billing_cycle(mut self, billing_cycle: BillingCycle) -> Self {
        self.billing_cycle = Some(billing_cycle);
        self
    }

    pub fn trial_period(&self) -> Option<&TrialPeriod> {
        self.trial_period.as_ref()
    }

    pub fn set_trial_period(mut self, trial_period: TrialPeriod) -> Self {
        self.trial_period = Some(trial_period);
        self
    }

    pub fn tax_mode(&self) -> Option<&TaxMode> {
        self.tax_mode.as_ref()
    }

    pub fn set_tax_mode(mut self, tax_mode: TaxMode) -> Self {
        self.tax_mode = Some(tax_mode);
        self
    }

    pub fn unit_price(&self) -> Option<&UnitPrice> {
        self.unit_price.as_ref()
    }

    pub fn set_unit_price(mut self, unit_price: UnitPrice) -> Self {
        self.unit_price = Some(unit_price);
        self
    }

    pub fn unit_price_overrides(&self) -> Option<&Vec<UnitPriceOverride>> {
        self.unit_price_overrides.as_ref()
    }

    pub fn set_unit_price_overrides(
        mut self,
        unit_price_overrides: Vec<UnitPriceOverride>,
    ) -> Self {
        self.unit_price_overrides = Some(unit_price_overrides);
        self
    }

    pub fn quantity(&self) -> Option<&Quantity> {
        self.quantity.as_ref()
    }

    pub fn set_quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn status(&self) -> Option<&EntityStatus> {
        self.status.as_ref()
    }

    pub fn set_status(mut self, status: EntityStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn custom_data(&self) -> Option<&serde_json::Value> {
        self.custom_data.as_ref()
    }

    pub fn set_custom_data(mut self, custom_data: serde_json::Value) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    pub fn import_meta(&self) -> Option<&ImportMeta> {
        self.import_meta.as_ref()
    }

    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }

    pub fn updated_at(&self) -> Option<&str> {
        self.updated_at.as_deref()
    }
}

impl Default for Quantity {
    fn default() -> Self {
        Quantity {
            minimum: 1,
            maximum: 100,
        }
    }
}
