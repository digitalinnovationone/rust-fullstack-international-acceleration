use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{component::Component, rig::{Rig, RigComponent}};

#[derive(Serialize, Deserialize)]
pub struct CreateRigData {
    pub name: String,
    pub components: Vec<Uuid>,
}

impl From<CreateRigData> for (Rig, Vec<RigComponent>) {
    fn from(value: CreateRigData) -> Self {
        let CreateRigData { name, components } = value;
        let rig = Rig::new(name);
        let rig_components = components
            .into_iter()
            .map(|component_id| RigComponent {
                rig_id: rig.id, component_id
            })
            .collect();

        (rig, rig_components)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RigWithComponents {
    pub id: Uuid,
    pub name: String,
    pub components: Vec<Component>,
}

impl Rig {
    pub fn with_components(self, components: Vec<Component>) -> RigWithComponents {
        let Self { id, name } = self;
        RigWithComponents {
            id,
            name,
            components,
        }
    }
}

impl From<RigWithComponents> for (Rig, Vec<Component>) {
    fn from(value: RigWithComponents) -> Self {
        let RigWithComponents { id, name, components } = value;
        (
            Rig { id, name },
            components,
        )
    }
}
