use rocket::{fairing::AdHoc, get, post, routes, serde::json::Json, Responder};
use rocket_db_pools::{Connection, diesel::QueryResult};
use uuid::Uuid;

use crate::{database::Db, dto::component::CreateComponentData, models::component::Component, repository::component};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Component Routes", |rocket| async {
        rocket.mount("/components", routes![detail, list, create])
    })
}


#[derive(Responder)]
pub enum GetError {
    #[response(status = 400)]
    InvalidId(String),
    #[response(status = 404)]
    NotFound(()),
}

#[get("/<id>")]
pub async fn detail(id: &str, mut db: Connection<Db>) -> Result<Json<Component>, GetError> {
    let id = Uuid::parse_str(id)
        .map_err(|e| GetError::InvalidId(format!("{e}")))?;

    Ok(Json(
        component::get_component(id, &mut db)
        .await
        .map_err(|_| GetError::NotFound(()))?
    ))
}

#[get("/")]
pub async fn list(mut db: Connection<Db>) -> QueryResult<Json<Vec<Component>>> {
    Ok(Json(component::list_components(&mut db).await?))
}


#[post("/", data = "<component>")]
pub async fn create(component: Json<CreateComponentData>, mut db: Connection<Db>) -> QueryResult<Json<Component>> {
    Ok(Json(component::create_component(component.0.into(), &mut db).await?))
}
