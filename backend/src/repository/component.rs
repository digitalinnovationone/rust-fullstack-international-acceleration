use rocket_db_pools::{
    diesel::{prelude::RunQueryDsl, QueryResult, QueryDsl, ExpressionMethods},
    Connection,
};
use uuid::Uuid;

use crate::{database::Db, models::component::Component};
use crate::schema::components;

pub async fn get_component(id: Uuid, db: &mut Connection<Db>) -> QueryResult<Component> {
    Ok(
        components::table
            .find(id)
            .get_result::<Component>(db)
            .await?
    )
}

pub async fn list_components(db: &mut Connection<Db>) -> QueryResult<Vec<Component>> {
    Ok(
        components::table
            .load::<Component>(db)
            .await?
    )
}

pub async fn list_components_by_id(ids: &[Uuid], db: &mut Connection<Db>) -> QueryResult<Vec<Component>> {
    Ok(
        components::table
            .filter(components::id.eq_any(ids))
            .load::<Component>(db)
            .await?
    )
}

pub async fn create_component(component: Component, db: &mut Connection<Db>) -> QueryResult<Component> {
    Ok(
        diesel::insert_into(components::table)
            .values(component)
            .get_result::<Component>(db)
            .await?
    )
}
