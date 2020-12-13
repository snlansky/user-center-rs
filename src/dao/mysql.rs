use diesel::prelude::*;
use crate::error::Result;

pub fn establish_connection(url: &str) -> MysqlConnection {
    MysqlConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

pub struct Dao {
    conn: MysqlConnection,
}

impl Dao {
    pub fn new(conn :MysqlConnection) -> Self {
        Self{conn}
    }

    pub fn save() -> Result<()> {
        Ok(())
    }
}