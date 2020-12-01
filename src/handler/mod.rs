use crate::{ model::Response};
use actix_web::{get, web, Responder};
use std::sync;
use crate::service;


pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(index);
}

pub struct Controller {
    pub user_service: service::UserService,
}



#[get("/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let user = ctrl.user_service.find_by_id(&id).await?;

    Response::ok(user).to_json_result()
}
