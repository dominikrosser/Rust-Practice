#![feature(plugin, custom_derive, const_fn, decl_macro, attr_literals, custom_attribute, extern_prelude)]
#![plugin(rocket_codegen)]

#![feature(proc_macro_non_items)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

// Remove unnecessary imports in the end
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::catch;
use rocket::catchers;
use rocket::Request;

mod schema;
mod models;
mod db;
mod static_files;
mod routes;

use routes::*;

fn rocket() -> rocket::Rocket {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

        let pool = db::init_pool(database_url);
        rocket::ignite()
            .manage(pool)
            .mount(
                "/api/v1",
                routes![index, new, show, delete, author, update]
            )
            .mount(
                "/",
                routes![static_files::index, static_files::all]
            )
            // FIXME: method catch not found for rocket::Rocket:
            // .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
