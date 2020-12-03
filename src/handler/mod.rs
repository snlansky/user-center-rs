use crate::{model::{self, Response}, service};
use actix_web::{web, Responder};

mod chain;
mod guard;
use actix_session::Session;
use chain::*;
use model::SUCCESS_RESPONSE;

pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/public").route("/login", web::post().to(login)))
        .service(
            web::scope("/api/v1")
                .guard(guard::SessionGuard {})
                .route("/chains", web::get().to(list_chain))
                .route("/chains/query", web::get().to(query_chain))
                .route("/chains/create", web::post().to(create_chain))
                .route("/chains/update", web::post().to(update_chain))
                .route("/chains/{id}", web::delete().to(delete_chain)),
        );
}

pub struct Controller {
    pub chain_service: service::ChainService,
}

pub async fn login(req: web::Json<model::Login>, session: Session) -> impl Responder {
    guard::set_uid(session, req.into_inner().username);
    Response::ok(SUCCESS_RESPONSE).to_json_result() 
}
