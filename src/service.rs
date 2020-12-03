use log::info;

use crate::dao;
use crate::error::Result;

use crate::dao::{MongoObject, Dao};
use crate::model::Chain;

pub struct ChainService {
    op: dao::Dao,
}

impl ChainService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(Chain::TABLE_NAME),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<MongoObject<Chain>>> {
        self.op.find_by_id(id).await
    }

    pub async fn save(&self, chain: Chain) -> Result<MongoObject<Chain>> {
        self.op.save_data(chain).await
    }

    pub async fn get_list(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<(Vec<MongoObject<Chain>>, i64)> {
        let doc = doc! {};
        let list = self.op.list(doc.clone(), limit, page).await?;
        let count = self.op.count(doc).await?;
        Ok((list, count))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let res = self.op.delete(id).await?;
        info!("delete {:}", res);
        Ok(())
    }

    pub async fn update(&self, chain: &MongoObject<Chain>) -> Result<()> {
        self.op.update(chain).await
    }
}
