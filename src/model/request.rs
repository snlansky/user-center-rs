use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestCreateChain {
    pub name: String,
    pub network_id: String,
    pub consensus: String,
    pub consortium_id: Option<String>,
    pub node_count: i32,
    pub tls_enabled: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PageQuery {
    pub page: i32,
    pub limit: i32,
}
