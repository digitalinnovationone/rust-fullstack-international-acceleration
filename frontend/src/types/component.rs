use leptos::*;
use strum::{VariantArray, Display};
use thaw::Icon;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentData {
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateComponentData {
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    VariantArray,
    Display,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Debug,
)]
pub enum Slot {
    Cpu,
    Gpu,
    Memory,
    Storage,
}

impl IntoView for Slot {
    fn into_view(self) -> leptos::View {
        let icon = match self {
            Slot::Cpu => icondata::BsCpu,
            Slot::Gpu => icondata::BsGpuCard,
            Slot::Memory => icondata::BsMemory,
            Slot::Storage => icondata::BsDeviceHdd,
        };

        view! {
            <Icon icon />
        }.into_view()
    }
}
