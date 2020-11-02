use std::{fs, sync::RwLock};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub addr: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: RwLock<Config> = RwLock::new(Config {
        server: Server {
            addr: "0.0.0.0:80".to_owned()
        }
    });
}

pub fn init(path: &str) {
    let content = fs::read(path).unwrap();
    let conf = serde_yaml::from_slice(&content).unwrap();
    let mut w = GLOBAL_CONFIG.write().unwrap();
    println!("{:#?}", conf);
    *w = conf;
}
