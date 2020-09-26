use crate::controller::index;
use actix_web::{get, web, App, HttpServer, Responder};

#[macro_use]
extern crate lazy_static;

mod controller;
mod dao;
mod error;
mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = "mongodb://root:ul1zXBBdfF@mongo-mongodb:27017/admin?authSource=admin";
    dao::init(uri).await;

    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
