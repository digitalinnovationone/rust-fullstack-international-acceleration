use rocket_db_pools::diesel::{self, AsChangeset, Identifiable, Insertable, Queryable, Selectable, Associations};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::component::Component;

// use crate::schema::{rigs, rig_components};

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
#[diesel(table_name = crate::schema::rigs)]
pub struct Rig {
    pub id: Uuid,
    pub name: String,
}

impl Rig {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Queryable,
    Insertable,
    Associations,
    Identifiable,
    Selectable,
)]
#[diesel(belongs_to(Rig))]
#[diesel(belongs_to(Component))]
#[diesel(primary_key(rig_id, component_id))]
#[diesel(table_name = crate::schema::rig_components)]
pub struct RigComponent {
    pub rig_id: Uuid,
    pub component_id: Uuid,
}
