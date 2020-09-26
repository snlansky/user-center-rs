mod user;
pub use user::*;


const SUCCESS_CODE :i32 = 0;
const SUCCESS_MSG :&str = "ok";

pub struct Response<T> {
    errorcode :i32,
    description:String,
    data: Option<T>,
}

impl <T> Response<T> {
    pub fn new(data: T) -> Self {
        Response {
            errorcode: SUCCESS_CODE,
            description: SUCCESS_MSG.to_string(),
            data: Some(data),
        }
    }

    pub fn new_internal_error(msg: &str) -> Self{
        Response {
            errorcode: 500,
            description: msg.to_string(),
            data: None
        }
    }

    pub fn new_from_errmsg(code: i32, msg: &str) -> Self {
        Response {
            errorcode:code,
            description:msg.to_string(),
            data:None,
        }
    }
}
