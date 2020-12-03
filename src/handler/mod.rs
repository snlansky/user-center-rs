use crate::model::{self, Response, ResponseList, SUCCESS_RESPONSE};
use crate::service;
use actix_web::{get, post,delete, web, Responder};
use std::sync;
mod chain;
use chain::*;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(index).service(
        web::scope("/api/v1")
            .service(create_chain)
            .service(list_chain)
            .service(delete_chain),
    );
}

pub struct Controller {
    pub chain_service: service::ChainService,
}
