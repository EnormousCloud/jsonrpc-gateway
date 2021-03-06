use jsonrpc_proto::formatter::OutputFormat;
use structopt::StructOpt;
use tracing_subscriber::prelude::*;

#[derive(StructOpt, Debug, Clone)]
pub enum Command {
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(long)]
        slug: Option<String>,
        #[structopt(long, default_value = "/")]
        path: String,
        #[structopt(short, long)]
        url: String,
        #[structopt(long)]
        strip: bool,
    },
    Get {
        #[structopt(short, long)]
        app: String,
    },
    Update {
        #[structopt(short, long)]
        app: String,
        #[structopt(long)]
        active: Option<bool>,
        #[structopt(long)]
        path: Option<String>,
        #[structopt(short, long)]
        url: Option<String>,
        #[structopt(long)]
        strip: Option<bool>,
    },
    List,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "jsonrpc-app", about = "RPC Apps management CLI utility")]
pub struct Args {
    #[structopt(long, default_value = "localhost", env = "REDIS_HOST")]
    pub redis_host: String,
    #[structopt(long, default_value = "6379", env = "REDIS_PORT")]
    pub redis_port: u32,
    #[structopt(long, default_value = "", env = "REDIS_USERNAME")]
    pub redis_username: String,
    #[structopt(long, default_value = "", env = "REDIS_PASSWORD")]
    pub redis_password: String,
    #[structopt(long, default_value = "0", env = "REDIS_DB")]
    pub redis_db: u32,
    #[structopt(long, env = "REDIS_TLS")]
    pub redis_tls: bool,
    #[structopt(
        short,
        long,
        possible_values = &OutputFormat::variants(), 
        case_insensitive = true,
    )]
    pub format: Option<OutputFormat>,
    #[structopt(subcommand)]
    pub cmd: Command,
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
