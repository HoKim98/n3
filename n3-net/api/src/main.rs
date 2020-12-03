#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde;

mod cors;
mod db;
mod error;
mod global;
mod model;
mod schema;
mod view;

fn main() {
    rocket::ignite()
        .attach(self::db::Database::fairing())
        .attach(self::cors::fairing())
        .register(self::view::catchers::all())
        .mount("/", self::view::routes::all())
        .launch();
}
