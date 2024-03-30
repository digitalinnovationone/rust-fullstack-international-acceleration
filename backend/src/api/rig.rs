use rocket::{fairing::AdHoc, get, post, routes, serde::json::Json, Responder};
use rocket_db_pools::{Connection, diesel::QueryResult};
use uuid::Uuid;

use crate::{database::Db, dto::rig::{CreateRigData, RigWithComponents}, repository::{component, rig}};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Rig Routes", |rocket| async {
        rocket.mount("/rigs", routes![list, create])
    })
}

#[get("/")]
pub async fn list(mut db: Connection<Db>) -> QueryResult<Json<Vec<RigWithComponents>>> {
    Ok(Json(rig::list_rigs(&mut db).await?))
}

#[post("/", data = "<rig>")]
pub async fn create(rig: Json<CreateRigData>, mut db: Connection<Db>) -> QueryResult<Json<RigWithComponents>> {
    let (rig, rig_components) = rig.0.into();
    rig::create_rig(&rig, &mut db).await?;
    rig::create_rig_components(&rig_components, &mut db).await?;

    let component_ids: Vec<Uuid> = rig_components
        .into_iter()
        .map(|rc| rc.component_id)
        .collect();

    let components = component::list_components_by_id(&component_ids, &mut db)
        .await?;

    Ok(Json(rig.with_components(components)))
}
