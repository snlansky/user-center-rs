use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chain {
    pub name: String,
    pub network_id: String,
    pub consensus: String,
    pub consortium_id: Option<String>,
    pub node_count: i32,
    pub account: String,
    pub team: String,
    pub tls_enabled: String,
    pub create_time: i64,
    pub description: Option<String>,
}

impl Chain {
    pub const TABLE_NAME: &'static str = "chain";
}
