use crate::Application;
use serde::{de, Serialize};

pub struct RedisConnection {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db: u32,
    pub use_tls: bool,
}

struct RedisStorage {
    con: redis::Connection,
}

impl<'de> RedisStorage {
    pub fn from_redis(info: &RedisConnection) -> anyhow::Result<Self> {
        let uri_scheme = if info.use_tls { "rediss" } else { "redis" };
        let client = redis::Client::open(format!(
            "{}://{}:{}@{}:{}/{}",
            uri_scheme, info.username, info.password, info.host, info.port, info.db
        ))
        .unwrap();
        let mut con = client.get_connection()?;
        let _: () = redis::cmd("PING").query(&mut con).unwrap(); // ping to check we are connected
        Ok(Self { con })
    }

    pub fn set<T>(&mut self, key: &str, v: &T) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let val = serde_json::to_string(v)?;
        redis::cmd("SET").arg(key).arg(val).query(&mut self.con)?;
        Ok(())
    }

    pub fn get<T>(&mut self, key: &str) -> Option<T>
    where
        T: de::DeserializeOwned,
    {
        match redis::cmd("GET").arg(key).query::<String>(&mut self.con) {
            Ok(rval) => match serde_json::from_str::<T>(&rval) {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn scan<T>(&mut self, prefix: &str) -> Vec<T>
    where
        T: de::DeserializeOwned,
    {
        match redis::cmd("SCAN")
            .arg(0)
            .arg("MATCH")
            .arg(format!("{}*", prefix))
            .query::<String>(&mut self.con)
        {
            Ok(rval) => match serde_json::from_str::<T>(&rval) {
                Ok(v) => vec![v],
                Err(_) => vec![],
            },
            Err(_) => vec![],
        }
    }
}

pub struct AppStorage {
    prefix: String,
    kv: RedisStorage,
}

impl AppStorage {
    pub fn from_redis(info: &RedisConnection) -> anyhow::Result<Self> {
        Ok(Self {
            prefix: "app_".to_owned(),
            kv: RedisStorage::from_redis(info).unwrap(),
        })
    }
    fn realkey(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }
    pub fn set(&mut self, key: &str, v: &Application) -> anyhow::Result<()> {
        self.kv.set(&self.realkey(key), v)
    }
    pub fn get(&mut self, key: &str) -> Option<Application> {
        self.kv.get(&self.realkey(key))
    }
    pub fn scan(&mut self) -> Vec<Application> {
        self.kv.scan(&self.prefix)
    }
}

pub struct RpcKeyStorage {
    prefix: String,
    kv: RedisStorage,
}

impl RpcKeyStorage {
    pub fn from_redis(info: &RedisConnection) -> anyhow::Result<Self> {
        Ok(Self {
            prefix: "rk_".to_owned(),
            kv: RedisStorage::from_redis(info).unwrap(),
        })
    }
    fn realkey(&self, app: &str, key: &str) -> String {
        format!("{}a{}_{}", self.prefix, app, key)
    }
    pub fn set(&mut self, app: &str, key: &str, v: &Application) -> anyhow::Result<()> {
        self.kv.set(&self.realkey(app, key), v)
    }
    pub fn get(&mut self, app: &str, key: &str) -> Option<Application> {
        self.kv.get(&self.realkey(app, key))
    }
    pub fn scan(&mut self, app: &str) -> Vec<Application> {
        let p = format!("{}a{}_", self.prefix, app);
        self.kv.scan(&p)
    }
}
