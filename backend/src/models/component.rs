use rocket_db_pools::diesel::{self, AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::Slot;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::components)]
pub struct Component {
    pub id: Uuid,
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32,
}

impl Component {
    pub fn new(manufacturer: String, model: String, slot: Slot, price: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            manufacturer,
            model,
            slot,
            price,
        }
    }
}
