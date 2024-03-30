use rocket_db_pools::{diesel::PgPool, Database};

#[derive(Database)]
#[database("api")]
pub struct Db(PgPool);
