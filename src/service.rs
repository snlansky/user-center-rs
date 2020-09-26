use crate::dao;
use crate::error::Result;
use crate::model;

pub struct UserService {
    op: dao::Dao,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            op: dao::Dao::new(model::User::TABLE_NAME),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<model::User>> {
        self.op.find_by_id(id).await
    }
}
