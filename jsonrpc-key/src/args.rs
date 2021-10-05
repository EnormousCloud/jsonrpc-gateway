use jsonrpc_proto::OutputFormat;
use structopt::StructOpt;
use tracing_subscriber::prelude::*;

#[derive(StructOpt, Debug, Clone)]
enum Command {
    Gen {
        #[structopt(short, long)]
        app: String,
        #[structopt(short, long)]
        tag: Vec<String>,
        #[structopt(long)]
        expires: Option<u32>,
        #[structopt(long)]
        active: Option<bool>,
        #[structopt(rename = "per-second", long)]
        quota_second: Option<u64>,
        #[structopt(rename = "per-minute", long)]
        quota_minute: Option<u64>,
        #[structopt(rename = "per-hour", long)]
        quota_hour: Option<u64>,
        #[structopt(rename = "per-day", long)]
        quota_day: Option<u64>,
        #[structopt(rename = "per-week", long)]
        quota_week: Option<u64>,
        #[structopt(rename = "per-month", long)]
        quota_month: Option<u64>,
        #[structopt(rename = "per-year", long)]
        quota_year: Option<u64>,
    },
    Get {
        #[structopt(short, long)]
        app: String,
        #[structopt(short, long)]
        key: String,
    },
    Update {
        #[structopt(short, long)]
        app: String,
        #[structopt(short, long)]
        key: String,
        #[structopt(long)]
        expires: Option<u32>,
        #[structopt(long)]
        active: Option<bool>,
        #[structopt(short, long)]
        tag: Vec<String>,
        #[structopt(rename = "per-second", long)]
        quota_second: Option<u64>,
        #[structopt(rename = "per-minute", long)]
        quota_minute: Option<u64>,
        #[structopt(rename = "per-hour", long)]
        quota_hour: Option<u64>,
        #[structopt(rename = "per-day", long)]
        quota_day: Option<u64>,
        #[structopt(rename = "per-week", long)]
        quota_week: Option<u64>,
        #[structopt(rename = "per-month", long)]
        quota_month: Option<u64>,
        #[structopt(rename = "per-year", long)]
        quota_year: Option<u64>,
    },
    List {
        #[structopt(short, long)]
        app: Option<String>,
        #[structopt(short, long)]
        tag: Vec<String>,
    },
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "jsonrpc-key", about = "RPC Key management CLI utility")]
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
    #[structopt(short, long)]
    format: Option<OutputFormat>,
    #[structopt(subcommand)]
    operation: Command,
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
