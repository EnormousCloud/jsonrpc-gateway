use structopt::StructOpt;
use tracing_subscriber::prelude::*;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "jsonrpc-gw", about = "RPC gateway server")]
pub struct Args {
    #[structopt(long, default_value = "localhost", env = "REDIS_HOST")]
    redis_host: String,
    #[structopt(long, default_value = "6379", env = "REDIS_PORT")]
    redis_port: u32,
    #[structopt(long, default_value = "", env = "REDIS_USERNAME")]
    redis_username: String,
    #[structopt(long, default_value = "", env = "REDIS_PASSWORD")]
    redis_password: String,
    #[structopt(long, default_value = "0", env = "REDIS_DB")]
    redis_db: u32,
}

pub fn parse() -> anyhow::Result<Args> {
    dotenv::dotenv().ok();
    let log_level: String = std::env::var("LOG_LEVEL").unwrap_or("info".to_owned());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_ansi(false)
        .with_level(false)
        .with_target(false);
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new(&log_level))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let res = Args::from_args();
    tracing::debug!("{:?}", res);
    Ok(res)
}
