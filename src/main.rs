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
