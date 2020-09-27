use crate::model::Response;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BusinessError>;

#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("Validation error on field: {field}")]
    ValidationError { field: String },
    #[error("argument error")]
    ArgumentError,
    #[error("An internal error occurred. Please try again later.")]
    InternalError {
        #[source]
        source: anyhow::Error,
    },
}

impl ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        use log::error;
        let mut code;
        match self {
            BusinessError::ValidationError { field } => {
                code = 1001;
            }
            BusinessError::ArgumentError => {
                code = 1002;
            }
            BusinessError::InternalError { source } => {
                error!("internal error: {:?}", source);
                code = 1003;
            }
        };
        let resp = Response::err(code, &self.to_string());
        HttpResponse::BadRequest().json(resp)
    }
}

impl From<mongodb::error::Error> for BusinessError {
    fn from(e: mongodb::error::Error) -> Self {
        BusinessError::InternalError {source:anyhow!(e)}
    }
}

impl From<bson::ser::Error> for BusinessError {
    fn from(e: bson::ser::Error) -> Self {
        BusinessError::InternalError {source:anyhow!(e)}
    }
}

