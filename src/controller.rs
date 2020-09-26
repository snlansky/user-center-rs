use crate::service;
use actix_web::{get, web, Responder};
use std::sync;

pub struct Controller {
    user_service: service::UserService,
}

impl Controller {
    pub fn new(user_service: service::UserService) -> Self {
        Controller { user_service }
    }
}

#[get("/{id}")]
pub async fn index(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let user = ctrl.user_service.find_by_id(&id).await.unwrap();

    format!("Hello {:?}!", user)
}
