use serde::{Deserialize, Serialize};

/// How authentication is defined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authentication {
    pub auth_header_name: Option<String>,
}

/// Description of how definition is defined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub location: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionData {
    pub not_versioned: bool,
    // versions: Option<HashMap<String, VersionDeclaration>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEndpoint {
    listen_path: String,
    target_url: String,
    strip_listen_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub slug: String,
    pub auth: Authentication,
    pub definition: Option<Definition>,
    pub version_data: Option<VersionData>,
    pub proxy: ProxyEndpoint,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcKey {
    pub key_id: String,
    pub key_hash: String,
    pub apps: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcKeyAddRequest {
    pub apps: Vec<String>,
    pub expires: u32,
    pub quota_second: Option<u64>,
    pub quota_minute: Option<u64>,
    pub quota_hour: Option<u64>,
    pub quota_day: Option<u64>,
    pub quota_week: Option<u64>,
    pub quota_month: Option<u64>,
    pub quota_year: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcResponseStatus {
    OK,
    Failure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcKeyAction {
    Add,
    Get,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcKeyAddResponse {
    pub action: RpcKeyAction,
    pub status: RpcResponseStatus,
    pub key: String,
    pub key_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcKeyListResponse {
    pub status: RpcResponseStatus,
    pub action: RpcKeyAction,
    pub keys: Vec<RpcKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcKeyResponse {
    pub status: RpcResponseStatus,
    pub action: RpcKeyAction,
    pub keys: Vec<RpcKey>,
}
