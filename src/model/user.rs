use crate::dao;
use crate::error::Result;
use serde::{Deserialize, Serialize, Serializer};
use bson::oid::ObjectId;
use dao::serialize_object_id;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(serialize_with = "serialize_object_id")]
    pub _id: Option<ObjectId>,
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


impl User {
    pub const TABLE_NAME: &'static str = "user";
}

