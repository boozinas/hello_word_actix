<<<<<<< HEAD
extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db_utils;
mod models;
mod schema;
mod actors;

fn main() {
    println!("Hello, world!");
}
=======
use crate::repository::{MemoryRepository, RepositoryInjector};
use actix_web::body::Body::Message;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use uuid::Uuid;

mod manual_hello;
mod repository;
mod user;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("La req es {}", req_body);
    HttpResponse::Ok().body(req_body)
}
// index: web::Data<Arc<AtomicU16>>
// Movido al módulo manual_hello.rs
/*
async fn manual_hello(thread_index: web::Data<u16>) -> impl Responder {
    HttpResponse::Ok()
        .header("thread-id", thread_index.to_string())
        .body("Hey there!")
}
*/

async fn get_user(user_id: web::Path<Uuid>, repo: web::Data<RepositoryInjector>) -> HttpResponse {
    match repo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    let thread_counter = Arc::new(AtomicU16::new(1));
    // línea utilizada hasta el tutorial número 3
    // let repo = Arc::new(MemoryRepository::default());

    //Starting shared memory
    let repo = MemoryRepository::default();
    let repo = RepositoryInjector::new(repo);
    let repo = web::Data::new(repo);

    // Starting server
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        println!("starting thread {}", thread_index);
        // Starting services
        App::new()
            .data(thread_index)
            .app_data(repo.clone())
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .service(hello)
            .service(echo)
            // utilizado antes de la sección 6
            // .route("/hey", web::get().to(manual_hello))
            // Mandando funcionamiento a un módulo
            .configure(manual_hello::service)
        // .route("/")
    })
    .bind("127.0.0.1:8080")
    .unwrap_or_else(|err| panic!("Couldn't start server in port 8080: {:?}", err))
    .run()
    .await
}
/* async fn get_user(req: HttpRequest) -> HttpResponse {
    if let Some(user_id) = req.match_info().get("user_id") {
        match uuid::Uuid::parse_str(user_id) {
            Ok(parsed_user_id) => {
                let repo = MemoryRepository::default();
                match repo.get_user(parsed_user_id){
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::NotFound().body("Not Found")
                }
            }
            Err(_) => {
                HttpResponse::BadRequest().body("Invalid user_id")
            }
        }
    } else {
        HttpResponse::NotFound().body("Not found")
    }
} */
>>>>>>> 1d154780afa2e4a4e44e66b8da505d56fb22d65e
