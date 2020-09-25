use mongodb::{Client, Database};
use std::sync::Mutex;
lazy_static! {
    pub static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

pub async fn init(uri: &str) {
    let client = Client::with_uri_str(uri).await.unwrap();
    let mut lock = DB.lock().unwrap();
    *lock = Some(client.database("blockchain_manager"));
}
