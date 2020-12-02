use std::sync::RwLock;

use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Server {
    pub addr: String,
}
#[derive(Debug, Deserialize, Default)]
pub struct Redis {
    pub uri: String,
    pub password: Option<String>,
    pub db: i32,
}

#[derive(Debug, Deserialize, Default)]
pub struct Mongo {
    pub uri: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Dao {
    pub redis: Redis,
    pub mongo: Mongo,
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    pub server: Server,
    pub dao: Dao,
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: RwLock<Settings> = RwLock::new(Settings::default());
}

pub fn init(path: &str) {
    let mut config = Config::default();
    config.merge(File::with_name(path)).unwrap();

    let s: Settings = config.try_into().unwrap();
    println!("{:#?}", s);
    let mut w = GLOBAL_CONFIG.write().unwrap();
    *w = s;
}
