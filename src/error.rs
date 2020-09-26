use actix_web::{HttpResponse, error};
use thiserror::Error;
use crate::model::Response;

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, BusinessError>;

#[derive(Debug, Error)]
pub enum BusinessError {

}

impl BusinessError {
    fn to_code(&self) -> i32 {
        0
    }

    fn to_message(&self) -> String {
        "msg".to_owned()
    }
}

impl error::ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        let resp = Response::err(self.to_code(), &self.to_message());
        HttpResponse::BadRequest().json(resp)
    }
}

impl From<mongodb::error::Error> for BusinessError {
    fn from(e: mongodb::error::Error) -> Self {
        unimplemented!()
    }
}

impl From<bson::ser::Error> for BusinessError {
    fn from(e: bson::ser::Error) -> Self {
        unimplemented!()
    }
}

impl From<bson::oid::Error> for BusinessError {
    fn from(e: bson::oid::Error) -> Self {
        unimplemented!()
    }
}
