use actix_web::{Responder, web};

use crate::model::{self, Response};

use super::Controller;
use std::sync;
use crate::schema::user;

pub async fn create_user(
    req: web::Json<model::NewUser>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let req = req.into_inner();

    diesel::insert_into(user::table).values(&req);
    Response::ok(0).to_json_result()
}