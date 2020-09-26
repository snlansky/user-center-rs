use crate::error::Result;
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

pub async fn init(uri: &str) {
    let client = Client::with_uri_str(uri).await.unwrap();
    let mut lock = DB.lock().unwrap();
    *lock = Some(client.database("blockchain_manager"));
}

pub fn collection(name: &str) -> Collection {
    let db = DB.lock().unwrap();
    (*db).as_ref().unwrap().collection(name)
}

pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none(),
    }
}

pub struct Dao {
    coll: Collection,
}

impl Dao {
    pub fn new(name: &str) -> Self {
        let coll = collection(name);
        Dao { coll }
    }

    pub async fn save<T>(&self, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let data = bson::to_bson(data)?.as_document().unwrap().to_owned();
        self.coll.insert_one(data, None).await?;
        Ok(())
    }

    pub async fn find_by_id<T>(&self, id: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let oid = ObjectId::with_string(id)?;

        let filter = doc! { "_id":  oid};
        let data = self.coll.find_one(filter, None).await?;
        match data {
            Some(d) => {
                let data: T = bson::from_document(d).unwrap();
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}
