
use serde::{Deserialize, Serialize};
use crate::schema::user;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub age: i32,
    pub skey: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="user"]
pub struct NewUser {
    pub name: String,
    pub password: String,
    pub age: u32,
    pub skey: String
}