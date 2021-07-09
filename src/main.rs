extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db_utils;
mod models;
mod schema;
mod actors;

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, patch, post, put, web::{self, Data, Json, Path}};

use actix::SyncArbiter;
use actors::db::{Create, DbActor, Delete, GetUsers, Update};
use db_utils::{get_pool, run_migrations};
use models::{AppState, UserData};
use std::env;
use uuid::Uuid;


#[post("/new")]
async fn create_user(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    match db.send(Create{ name: user.name, geopoints: user.geopoints}).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    // let db_url = "postgres://booz:".expect("Error retrieving the database url");
    let db_url = "postgres://booz:R4ascuacho1@localhost/geoanp";
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr =  SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(create_user)
            .data(AppState {
                db: db_addr.clone()
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}