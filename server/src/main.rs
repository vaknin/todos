#![allow(dead_code, unused_variables, unused_imports)]
use std::ffi::c_long;
use crate::{models::{NewTodo, Todo}, routes::delete_todo};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use helpers::initiate_server;
use routes::get_todos;
use tracing::info;

mod types;
mod models;
mod schema;
mod routes;
mod helpers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(initiate_server().unwrap());
    let address = format!("{}:{}", state.server_address.ip(), state.server_address.port());
    info!("running on {address}!");

    HttpServer::new(move || {
        App::new()
        .wrap(Cors::permissive())
        .app_data(state.clone())
            // .service(create_todo)
            .service(get_todos)

            // .service(get_todo)
            // .service(update_todo)
            .service(delete_todo)
    })
    .bind(address)?
    .run()
    .await
}