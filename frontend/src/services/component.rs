use std::time::Duration;

use leptos::*;

use crate::types::component::{ComponentData, CreateComponentData};

#[server]
pub async fn fetch_components() -> Result<Vec<ComponentData>, ServerFnError> {
    actix_web::rt::time::sleep(Duration::from_secs(3)).await;

    Ok(
        reqwest::get("http://localhost:8000/components")
        .await?
        .json::<Vec<ComponentData>>()
        .await?
    )
}

#[server]
pub async fn create_component(create_component_data: CreateComponentData) -> Result<ComponentData, ServerFnError> {
    actix_web::rt::time::sleep(Duration::from_secs(3)).await;

    let client = reqwest::Client::new();
    Ok(
        client.post("http://localhost:8000/components")
        .json(&create_component_data)
        .send()
        .await?
        .json::<ComponentData>()
        .await?
    )
}