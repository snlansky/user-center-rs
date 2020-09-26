use crate::model;

pub struct UserService {
    op: model::UserOp
}

impl UserService {
    pub fn new() -> Self {
        UserService{op:model::UserOp::new()}
    }
}
