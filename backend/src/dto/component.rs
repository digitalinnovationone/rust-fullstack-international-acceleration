use serde::{Deserialize, Serialize};

use crate::{models::component::Component, types::Slot};

#[derive(Serialize, Deserialize)]
pub struct CreateComponentData {
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32,
}

impl From<CreateComponentData> for Component {
    fn from(value: CreateComponentData) -> Self {
        let CreateComponentData { manufacturer, model, slot, price } = value;
        Self::new(manufacturer, model, slot, price)
    }
}
