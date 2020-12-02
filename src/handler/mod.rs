use crate::model::{self, RequestCreateChain, Response};
use crate::service;
use actix_web::{get, post, web, Responder};
use std::sync;

pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(index)
        .service(web::scope("/api/v1").service(create_chain));
}

pub struct Controller {
    pub chain_service: service::ChainService,
}

#[get("/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let user = ctrl.chain_service.find_by_id(&id).await?;

    Response::ok(user).to_json_result()
}

#[post("/chains/create")]
async fn create_chain(
    req: web::Json<RequestCreateChain>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let req = req.into_inner();

    let mut chain = model::Chain {
        name: req.name,
        network_id: req.network_id,
        consensus: req.consensus,
        consortium_id: req.consortium_id,
        node_count: req.node_count,
        account: "admin".to_owned(),
        team: "admin".to_owned(),
        tls_enabled: req.tls_enabled,
        create_time: chrono::Utc::now().timestamp(),
        description: req.description,
    };

    let ret = ctrl.chain_service.save(chain).await?;
    Response::ok(ret).to_json_result()
}
