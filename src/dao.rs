use crate::error::{BusinessError, Result};
use bson::{oid::ObjectId, Document};
use mongodb::{
    bson::doc,
    options::{ClientOptions, CountOptions, FindOneOptions},
    Client, Collection, Database,
};
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use std::sync::Mutex;
use std::time::Duration;

lazy_static! {
    static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

pub async fn init(uri: &str) {
    let mut options = ClientOptions::parse(uri).await.unwrap();
    options.connect_timeout = Some(Duration::from_secs(3));
    options.heartbeat_freq = Some(Duration::from_secs(3));
    options.server_selection_timeout = Some(Duration::from_secs(3));
    let client = Client::with_options(options).unwrap();
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
        let oid = Self::get_object_id(id)?;

        let filter = doc! { "_id":  oid};
        let mut opt = FindOneOptions::default();
        opt.max_time = Some(Duration::from_secs(3));
        let data = self.coll.find_one(filter, opt).await?;

        match data {
            Some(d) => {
                let data: T = bson::from_document(d)
                    .map_err(|e| BusinessError::InternalError { source: anyhow!(e) })?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
    pub async fn count(&self, filter: impl Into<Option<Document>>) -> Result<i64> {
        let opt = CountOptions::default();
        let count = self.coll.count_documents(filter, opt).await?;
        Ok(count)
    }

    fn get_object_id(id: &str) -> Result<ObjectId> {
        ObjectId::with_string(id).map_err(|_| BusinessError::ValidationError {
            field: id.to_owned(),
        })
    }
}
