use crate::service;
use actix_web::{delete, get, post, web, Responder};

mod chain;
use chain::*;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(index).service(
        web::scope("/api/v1")
            .route("/chains", web::get().to(list_chain))
            .route("/chains/create", web::post().to(create_chain))
            .route("/chains/update", web::post().to(update_chain))
            .route("/chains/{id}", web::delete().to(delete_chain)),
    );
}

pub struct Controller {
    pub chain_service: service::ChainService,
}
