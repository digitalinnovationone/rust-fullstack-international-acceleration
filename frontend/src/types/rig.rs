use serde::{Deserialize, Serialize};

use super::component::ComponentData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RigData {
    pub id: String,
    pub name: String,
    pub components: Vec<ComponentData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rig {
    pub id: String,
    pub name: String,
    pub components: Vec<ComponentData>,
    pub total_price: i32,
}

impl From<RigData> for Rig {
    fn from(value: RigData) -> Self {
        let RigData { id, name, components } = value;
        let total_price = components
            .iter()
            .map(|c| c.price)
            .sum();

        Self {
            id,
            name,
            components,
            total_price,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateRigData {
    pub name: String,
    pub components: Vec<String>,
}
