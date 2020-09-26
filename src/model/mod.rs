mod user;
pub use user::*;
use serde::{Serialize, Deserialize};
use actix_web::HttpResponse;
use crate::error::Result;


const SUCCESS_CODE :i32 = 0;
const SUCCESS_MSG :&str = "ok";

#[derive(Deserialize, Serialize)]
pub struct Response<T> where T: Serialize{
    code:i32,
    message:String,
    data: Option<T>,
}

impl <T: Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_owned(),
            data: Some(data),
        }
    }

    pub fn new_internal_error(msg: &str) -> Self{
        Response {
            code: 500,
            message: msg.to_owned(),
            data: None
        }
    }

    pub fn new_from_errmsg(code: i32, msg: &str) -> Self {
        Response {
            code,
            message:msg.to_owned(),
            data:None,
        }
    }

    pub fn to_json_result(&self) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().json(self))
    }
}
