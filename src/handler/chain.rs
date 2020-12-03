use crate::model::{self, Response, ResponseList, SUCCESS_RESPONSE};
use crate::service;
use actix_web::{get, post,delete, web, Responder};
use std::sync;
use crate::handler::Controller;
use crate::error::BusinessError;

#[get("/{id}")]
pub async fn index(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let user = ctrl.chain_service.find_by_id(&id).await?;

    Response::ok(user).to_json_result()
}

#[get("/chains")]
pub async fn list_chain(
    req: web::Query<model::PageQuery>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let req = req.into_inner();
    let (list, total) = ctrl.chain_service.get_list(req.page, req.limit).await?;
    let req = ResponseList { total, list };
    Response::ok(req).to_json_result()
}

#[post("/chains/create")]
pub async fn create_chain(
    req: web::Json<model::RequestCreateChain>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let req = req.into_inner();

    let chain = model::Chain {
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

#[delete("/chains/{id}")]
pub async fn delete_chain(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let chain = ctrl.chain_service.find_by_id(&id).await?;
    let chain = chain.ok_or_else(||BusinessError::ArgumentError)?;
    ctrl.chain_service.delete(&chain.id.unwrap().to_hex()).await?;
    Response::ok(SUCCESS_RESPONSE).to_json_result()
}
