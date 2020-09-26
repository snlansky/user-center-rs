use crate::controller::index;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync;

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

    let user_service = service::UserService::new();
    let ctrl = controller::Controller::new(user_service);
    let ctrl = sync::Arc::new(ctrl);
    HttpServer::new(move ||
        App::new().data(ctrl.clone()).service(index)
    )
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
