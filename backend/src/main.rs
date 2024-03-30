use database::Db;
use rocket::launch;
use rocket_db_pools::Database;

pub mod dto;
pub mod repository;
pub mod api;
pub mod schema;
pub mod types;
pub mod models;
pub mod database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(api::component::stage())
        .attach(api::rig::stage())
}
