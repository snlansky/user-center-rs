use diesel::prelude::*;
use crate::error::Result;
use crate::schema::user;

pub fn establish_connection(url: &str) -> MysqlConnection {
    MysqlConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

pub struct Dao {
    conn: MysqlConnection,
}

impl Dao {
    pub fn new(conn :MysqlConnection ) -> Self {
        Self{conn}
    }

    pub fn save<U, T>(&self, records: U) -> Result<()>
        where
            U: Insertable<T> + diesel::Insertable<table>
    {
        let v = diesel::insert_into(user::table).values(records).execute(&self.conn).unwrap();
        Ok(())
    }
}
