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
