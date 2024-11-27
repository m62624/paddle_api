pub mod create;
pub mod get;
pub mod list;
pub mod update;

use super::{
    product::Product, EntityBase, EntityBaseGettersSetters, EntityStatus, EntityType, Meta,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    #[serde(flatten)]
    base: EntityBase,
    product_id: Option<String>,
    unit_price: Option<UnitPrice>,
    billing_cycle: Option<BillingCycle>,
    trial_period: Option<TrialPeriod>,
    tax_mode: Option<TaxMode>,
    unit_price_overrides: Option<Vec<UnitPriceOverride>>,
    quantity: Option<Quantity>,
    product: Option<Product>,
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

#[derive(Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize, Default, PartialEq)]
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

impl PriceResponse {
    pub fn data(&self) -> &Price {
        &self.data
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl EntityBaseGettersSetters for Price {
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

impl Price {
    pub fn product_id(&self) -> Option<&str> {
        self.product_id.as_deref()
    }

    pub fn set_product_id<T: Into<String>>(mut self, product_id: T) -> Self {
        self.product_id = Some(product_id.into());
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

    pub fn product(&self) -> Option<&Product> {
        self.product.as_ref()
    }
}

impl BillingCycle {
    pub fn new<T: Into<String>>(frequency: i32, interval: T) -> Self {
        Self {
            frequency,
            interval: interval.into(),
        }
    }

    pub fn frequency(&self) -> i32 {
        self.frequency
    }

    pub fn interval(&self) -> &str {
        &self.interval
    }

    pub fn set_frequency<T: Into<i32>>(mut self, frequency: T) -> Self {
        self.frequency = frequency.into();
        self
    }

    pub fn set_interval<T: Into<String>>(mut self, interval: T) -> Self {
        self.interval = interval.into();
        self
    }
}

impl TrialPeriod {
    pub fn new(frequency: i32, interval: PriceInterval) -> Self {
        Self {
            frequency,
            interval,
        }
    }

    pub fn frequency(&self) -> i32 {
        self.frequency
    }

    pub fn interval(&self) -> &PriceInterval {
        &self.interval
    }

    pub fn set_frequency<T: Into<i32>>(mut self, frequency: T) -> Self {
        self.frequency = frequency.into();
        self
    }

    pub fn set_interval(mut self, interval: PriceInterval) -> Self {
        self.interval = interval;
        self
    }
}

impl UnitPrice {
    pub fn new<A: Into<String>, C: Into<String>>(amount: A, currency_code: C) -> Self {
        Self {
            amount: amount.into(),
            currency_code: currency_code.into(),
        }
    }

    pub fn amount(&self) -> &str {
        &self.amount
    }

    pub fn currency_code(&self) -> &str {
        &self.currency_code
    }

    pub fn set_amount<T: Into<String>>(mut self, amount: T) -> Self {
        self.amount = amount.into();
        self
    }

    pub fn set_currency_code<T: Into<String>>(mut self, currency_code: T) -> Self {
        self.currency_code = currency_code.into();
        self
    }
}

impl UnitPriceOverride {
    pub fn new<T, I>(country_codes: T, unit_price: UnitPrice) -> Self
    where
        T: IntoIterator<Item = I>,
        I: Into<String>,
    {
        Self {
            country_codes: country_codes.into_iter().map(Into::into).collect(),
            unit_price,
        }
    }

    pub fn country_codes(&self) -> &[String] {
        &self.country_codes
    }

    pub fn unit_price(&self) -> &UnitPrice {
        &self.unit_price
    }
}

impl Quantity {
    pub fn new(minimum: i32, maximum: i32) -> Self {
        Self { minimum, maximum }
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
