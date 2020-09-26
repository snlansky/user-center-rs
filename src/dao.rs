use mongodb::{Client, Database};
use std::sync::Mutex;
lazy_static! {
    static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

pub async fn init(uri: &str) {
    let client = Client::with_uri_str(uri).await.unwrap();
    let mut lock = DB.lock().unwrap();
    *lock = Some(client.database("blockchain_manager"));
}

pub fn collection(name: &str) -> mongodb::Collection {
    let db = DB.lock().unwrap();
    (*db).as_ref().unwrap().collection(name)
}
