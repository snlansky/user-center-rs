use crate::model::{self, Response, ResponseList, SUCCESS_RESPONSE};

use crate::error::BusinessError;
use crate::handler::Controller;
use actix_web::{web, Responder};
use std::sync;

pub async fn query_chain(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let user = ctrl.chain_service.find_by_id(&id).await?;

    Response::ok(user).to_json_result()
}

pub async fn list_chain(
    req: web::Query<model::PageQuery>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let req = req.into_inner();
    let (list, total) = ctrl.chain_service.get_list(req.page, req.limit).await?;
    let req = ResponseList { total, list };
    Response::ok(req).to_json_result()
}

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

pub async fn delete_chain(
    web::Path(id): web::Path<String>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let chain = ctrl.chain_service.find_by_id(&id).await?;
    let chain = chain.ok_or_else(|| BusinessError::ArgumentError)?;
    ctrl.chain_service
        .delete(&chain.id.unwrap().to_hex())
        .await?;
    Response::ok(SUCCESS_RESPONSE).to_json_result()
}

pub async fn update_chain(
    req: web::Json<model::RequestUpdateChain>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let chain = ctrl.chain_service.find_by_id(&req.chain_id).await?;
    let mut chain = chain.ok_or_else(|| BusinessError::ArgumentError)?;

    chain.data.name = req.name.clone();
    chain.data.node_count = req.node_count;
    chain.data.tls_enabled = req.tls_enabled.clone();
    chain.data.description = req.description.clone();

    ctrl.chain_service.update(&chain).await?;
    Response::ok(chain).to_json_result()
}
