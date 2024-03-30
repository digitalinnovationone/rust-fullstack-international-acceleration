use rocket::FromFormField;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Clone, Copy, FromFormField)]
#[ExistingTypePath = "crate::schema::sql_types::Slot"]
pub enum Slot {
    Cpu,
    Gpu,
    Memory,
    Storage,
}
