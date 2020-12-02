use crate::dao;
use crate::error::{Result, BusinessError};
use crate::model;

pub struct ChainService {
    op: dao::Dao,
}

impl ChainService {
    pub fn new() -> Self {
        Self {
            op: dao::Dao::new(model::Chain::TABLE_NAME),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<dao::MongoObject<model::Chain>>> {
        self.op.find_by_id(id).await
    }

    pub async fn save(&self, chain: model::Chain) -> Result<dao::MongoObject<model::Chain>> {
        self.op.save_data(chain).await
    }

    pub async fn get_list(&self, page: i32, limit: i32) -> Result<(Vec<dao::MongoObject<model::Chain>>, i64)> {
        let doc = doc! {};
        Ok((self.op.list(doc).await?, 0))
    }
}
