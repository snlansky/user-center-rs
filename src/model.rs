use crate::dao;
use crate::error::Result;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub network_id: String,
    pub consensus: String,
    pub node_count: i32,
    pub account: String,
    pub team: String,
    pub tls_enabled: String,
    pub create_time: i64,
    pub description: String,
}

pub struct UserOp {
    coll: mongodb::Collection,
}

impl UserOp {
    pub fn new() -> Self {
        let db = dao::DB.lock().unwrap();
        let coll = (*db).as_ref().unwrap().collection("user");
        UserOp { coll }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let filter = doc! { "_id":  id};
        let data = self.coll.find_one(filter, None).await?;
        match data {
            Some(d) => {
                let user: User = bson::from_document(d)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
