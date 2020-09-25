use actix_web::{get, web, App, HttpServer, Responder};

#[macro_use]
extern crate lazy_static;



mod controller;
mod service;
mod model;
pub mod dao;
pub mod error;


#[get("/{id}")]
async fn index(web::Path((id)): web::Path<(String)>) -> impl Responder {
    let op = model::UserOp::new();
    let user = op.find_by_id(&id).await.unwrap();
    format!("Hello {:?}!", user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = "mongodb://root:ul1zXBBdfF@mongo-mongodb:27017/admin?authSource=admin";
    dao::init(uri).await;


    HttpServer::new(||
        App::new().service(index)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
