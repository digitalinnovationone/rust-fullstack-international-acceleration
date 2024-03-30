use std::time::Duration;

use leptos::*;

use crate::types::{component::{ComponentData, CreateComponentData}, rig::{CreateRigData, Rig, RigData}};

#[server]
pub async fn fetch_rigs() -> Result<Vec<Rig>, ServerFnError> {
    let rigs_data = reqwest::get("http://localhost:8000/rigs")
        .await?
        .json::<Vec<RigData>>()
        .await?;

    let rigs: Vec<Rig> = rigs_data
        .into_iter()
        .map(Rig::from)
        .collect();

    Ok(rigs)
}

#[server]
pub async fn create_rig(create_rig_data: CreateRigData) -> Result<Rig, ServerFnError> {
    let client = reqwest::Client::new();

    Ok(client.post("http://localhost:8000/rigs")
        .json(&create_rig_data)
        .send()
        .await?
        .json::<RigData>()
        .await?
        .into())
}
