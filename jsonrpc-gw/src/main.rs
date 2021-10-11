pub mod api;
pub mod args;
pub mod telemetry;

use http_types::headers::HeaderValue;
use jsonrpc_proto::redis::{AppStorage, RpcKeyStorage};
use jsonrpc_proto::Application;
use std::sync::{Arc, Mutex};
use tide::security::{CorsMiddleware, Origin};
use tracing::info;

#[derive(Clone)]
pub struct State {
    default_app: Application,
    apps: Arc<Mutex<AppStorage>>,
    rpckeys: Arc<Mutex<RpcKeyStorage>>,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("args parsing error {}", e))),
    };
    let conn = args.get_redis_connection();
    let mut apps = AppStorage::from_redis(&conn).expect("apps storage init error");
    let rpckeys = RpcKeyStorage::from_redis(&conn).expect("key storage init error");
    let state = State {
        default_app: apps
            .get(&args.application)
            .expect("APPLICATION not configured"),
        apps: Arc::new(Mutex::new(apps)),
        rpckeys: Arc::new(Mutex::new(rpckeys)),
    };
    info!("Using default gateway for {:?}", state.default_app);
    if !state.default_app.active {
        panic!("Application is not active")
    }

    let mut app = tide::with_state(state);
    app.with(telemetry::TraceMiddleware::new());
    app.with(
        CorsMiddleware::new()
            .allow_methods("POST, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from("*"))
            .allow_credentials(false),
    );
    info!("Starting HTTP GW server {}", &args.addr);
    app.at("/*").post(api::proxy_rpc);
    app.at("/").post(api::proxy_rpc);
    app.listen(&args.addr).await?;
    Ok(())
}
