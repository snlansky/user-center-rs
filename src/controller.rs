use crate::{model, service};
use actix_web::{get, web, App, HttpServer, Responder};

pub struct Controller {
    user_service: service::UserService,
}

#[get("/{id}")]
pub async fn index(web::Path((id)): web::Path<(String)>) -> impl Responder {
    let op = model::UserOp::new();
    let user = op.find_by_id(&id).await.unwrap();
    format!("Hello {:?}!", user)
}
