#![feature(plugin, proc_macro_hygiene, decl_macro, never_type)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;
extern crate serde_json;

extern crate test_shared;
extern crate r2d2;
extern crate nats;
extern crate r2d2_nats;

mod nats_connection;
mod controllers;

use nats_connection::pooled_connection::init_pool;
use controllers::api::*;

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .register(catchers![service_not_available])
        .mount("/api", routes![deposit, withdraw, transfer])
        .launch();
}
