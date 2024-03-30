use rocket_db_pools::{
    diesel::{prelude::RunQueryDsl, QueryResult, QueryDsl, BelongingToDsl, ExpressionMethods, GroupedBy, Selectable, SelectableHelper},
    Connection,
};
use uuid::Uuid;

use crate::{database::Db, dto::rig::RigWithComponents, models::{component::Component, rig::{Rig, RigComponent}}};
use crate::schema::{components, rigs, rig_components};

pub async fn list_rigs(db: &mut Connection<Db>) -> QueryResult<Vec<RigWithComponents>> {
    let all_rigs = rigs::table
        .load::<Rig>(db)
        .await?;

    let rig_components = RigComponent::belonging_to(&all_rigs)
        .inner_join(components::table)
        .select((RigComponent::as_select(), Component::as_select()))
        .load(db)
        .await?;

    let rig_with_components: Vec<RigWithComponents> = rig_components
        .grouped_by(&all_rigs)
        .into_iter()
        .zip(all_rigs)
        .map(|(rc, rig)| rig.with_components(
            rc.into_iter().map(|(_, component)| component).collect()
        ))
        .collect();
    
    Ok(rig_with_components)
}

pub async fn create_rig(rig: &Rig, db: &mut Connection<Db>) -> QueryResult<usize> {
    Ok(diesel::insert_into(rigs::table)
        .values(rig)
        .execute(db)
        .await?)
}

pub async fn create_rig_components(rig_components: &[RigComponent], db: &mut Connection<Db>) -> QueryResult<usize> {
    Ok(diesel::insert_into(rig_components::table)
        .values(rig_components)
        .execute(db)
        .await?)
}