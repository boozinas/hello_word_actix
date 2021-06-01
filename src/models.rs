use diesel::sql_types::Integer;
use diesel::types::Int4;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use crate::schema::users;
use crate::actors::db::DbActor;
use crate::actix::Addr;

pub struct AppState {}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub  struct Users {
    pub name: Option<String>,
    pub geopoints: Option<String>,
    pub id: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: Option<String>,
    pub geopoints: Option<String>,
    // name: String,
    // geopoints: String,
}