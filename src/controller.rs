use crate::model::Response;
use crate::service;
use actix_web::{get, web, Responder};
use std::sync;

pub struct Controller {
    pub user_service: service::UserService,
}

#[get("/{id}")]
pub async fn index(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let user = ctrl.user_service.find_by_id(&id).await?;

    Response::ok(user).to_json_result()
}
