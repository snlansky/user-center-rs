use crate::error::Result;
use crate::model;

pub struct UserService {
    op: model::UserOp,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            op: model::UserOp::new(),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<model::User>> {
        self.op.find_by_id(id).await
    }
}
