pub mod formatter;
pub mod redis;

use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEndpoint {
    pub path: String,
    pub url: String,
    pub strip: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub slug: String,
    pub proxy: ProxyEndpoint,
    pub active: bool,
}

impl Application {
    pub fn new(name: &str, slug: Option<String>, path: String, url: String, strip: bool) -> Self {
        Self {
            name: name.to_owned(),
            slug: match slug {
                Some(x) => x,
                None => slugify(name),
            },
            proxy: ProxyEndpoint { path, url, strip },
            active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcKey {
    pub key_id: String,
    pub key_hash: String,
    pub app: String,
    pub tags: Vec<String>,
    pub expires: u32,
    pub active: bool,
    pub quota_second: Option<u64>,
    pub quota_minute: Option<u64>,
    pub quota_hour: Option<u64>,
    pub quota_day: Option<u64>,
    pub quota_week: Option<u64>,
    pub quota_month: Option<u64>,
    pub quota_year: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RpcKeyAddRequest {
    pub app: String,
    pub tags: Vec<String>,
    pub expires: u32,
    pub quota_second: Option<u64>,
    pub quota_minute: Option<u64>,
    pub quota_hour: Option<u64>,
    pub quota_day: Option<u64>,
    pub quota_week: Option<u64>,
    pub quota_month: Option<u64>,
    pub quota_year: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcResponseStatus {
    OK,
    Failure,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcKeyAction {
    Add,
    Get,
    List,
    Update,
}

#[derive(Debug, Clone, Serialize)]
pub struct RpcKeyAddResponse {
    pub action: RpcKeyAction,
    pub status: RpcResponseStatus,
    pub key: String,
    pub key_hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RpcKeyListResponse {
    pub status: RpcResponseStatus,
    pub action: RpcKeyAction,
    pub keys: Vec<RpcKey>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RpcKeyResponse {
    pub status: RpcResponseStatus,
    pub action: RpcKeyAction,
    pub keys: Vec<RpcKey>,
}
