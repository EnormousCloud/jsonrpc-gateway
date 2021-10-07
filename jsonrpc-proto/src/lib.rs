pub mod formatter;
pub mod redis;

use fasthash::murmur2;
use rand::Rng;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub expires: u64,
    pub active: bool,
    pub quota_second: Option<u64>,
    pub quota_minute: Option<u64>,
    pub quota_hour: Option<u64>,
    pub quota_day: Option<u64>,
    pub quota_week: Option<u64>,
    pub quota_month: Option<u64>,
    pub quota_year: Option<u64>,
}

impl RpcKey {
    pub fn generate(
        app: String,
        tag: Vec<String>,
        expires: Option<u64>,
        quota_second: Option<u64>,
        quota_minute: Option<u64>,
        quota_hour: Option<u64>,
        quota_day: Option<u64>,
        quota_week: Option<u64>,
        quota_month: Option<u64>,
        quota_year: Option<u64>,
    ) -> Self {
        const CHARSET: &[u8] = b"abcdefghijkmnpqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        let key_id: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let key_hash = format!("{:x}", murmur2::hash64(key_id.as_bytes()));
        Self {
            key_id,
            key_hash,
            app,
            tags: tag.clone(),
            expires: match expires {
                Some(x) => x,
                None => {
                    let dur = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    dur.as_secs() + 315360000 // expires 10 years from now
                }
            },
            quota_second,
            quota_minute,
            quota_hour,
            quota_day,
            quota_week,
            quota_month,
            quota_year,
            active: true,
        }
    }
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
pub enum RpcKeyResponse {
    Add {
        action: RpcKeyAction,
        status: RpcResponseStatus,
        key: String,
        key_hash: String,
    },
    List {
        status: RpcResponseStatus,
        action: RpcKeyAction,
        keys: Vec<String>,
    },
    Get {
        status: RpcResponseStatus,
        action: RpcKeyAction,
        key: RpcKey,
    },
}
