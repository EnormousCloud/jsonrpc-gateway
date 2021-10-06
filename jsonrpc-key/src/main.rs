pub mod args;
use jsonrpc_proto::formatter::Formatter;
use jsonrpc_proto::redis::{AppStorage, RedisConnection};

fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("args parsing error {}", e))),
    };
    let fmt = Formatter::new(args.format);
    let conn = RedisConnection {
        host: args.redis_host,
        port: args.redis_port,
        username: args.redis_username,
        password: args.redis_password,
        db: args.redis_db,
        use_tls: args.redis_tls,
    };
    let mut _apps = match AppStorage::from_redis(&conn) {
        Ok(x) => x,
        Err(e) => return fmt.wrap_error(e),
    };
    let mut _keys = match AppStorage::from_redis(&conn) {
        Ok(x) => x,
        Err(e) => return fmt.wrap_error(e),
    };

    fmt.out(&"JSONRPC-KEY".to_owned())
}
