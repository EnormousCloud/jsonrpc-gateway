pub mod args;
use jsonrpc_proto::formatter::Formatter;
use jsonrpc_proto::redis::{AppStorage, RedisConnection, RpcKeyStorage};
use jsonrpc_proto::{RpcKey, RpcKeyAction, RpcResponseStatus};

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
    let mut apps = match AppStorage::from_redis(&conn) {
        Ok(x) => x,
        Err(e) => return fmt.wrap_error(e),
    };
    let mut keys = match RpcKeyStorage::from_redis(&conn) {
        Ok(x) => x,
        Err(e) => return fmt.wrap_error(e),
    };

    match args.cmd {
        args::Command::Gen {
            app,
            tag,
            expires,
            quota_second,
            quota_minute,
            quota_hour,
            quota_day,
            quota_week,
            quota_month,
            quota_year,
        } => {
            let a = match apps.get(&app) {
                Some(x) => x,
                None => return fmt.fail("application not found"),
            };
            if !a.active {
                return fmt.fail("application is not active");
            }
            let k = RpcKey::generate(
                app,
                tag,
                expires,
                quota_second,
                quota_minute,
                quota_hour,
                quota_day,
                quota_week,
                quota_month,
                quota_year,
            );
            return fmt.out(&jsonrpc_proto::RpcKeyAddResponse {
                action: RpcKeyAction::Add,
                status: RpcResponseStatus::OK,
                key: k.key_id,
                key_hash: k.key_hash,
            });
        }
        args::Command::Get { app, key } => {
            if let None = apps.get(&app) {
                return fmt.fail("application not found");
            };
            let k = match keys.get(&app, &key) {
                Some(x) => x,
                None => return fmt.fail("key not found"),
            };
            return fmt.out(&jsonrpc_proto::RpcKeyResponse {
                action: RpcKeyAction::Add,
                status: RpcResponseStatus::OK,
                key: k,
            });
        }
        args::Command::Update {
            app,
            key,
            expires,
            active,
            tag,
            quota_second,
            quota_minute,
            quota_hour,
            quota_day,
            quota_week,
            quota_month,
            quota_year,
        } => {
            if let None = apps.get(&app) {
                return fmt.fail("application not found");
            };
            let mut k = match keys.get(&app, &key) {
                Some(x) => x,
                None => return fmt.fail("key not found"),
            };
            // TODO:

            let updated = keys.get(&app, &key).map(|x| x).unwrap();
            return fmt.out(&jsonrpc_proto::RpcKeyResponse {
                action: RpcKeyAction::Add,
                status: RpcResponseStatus::OK,
                key: updated,
            });
        }
        args::Command::List { app, .. } => {
            if let None = apps.get(&app) {
                return fmt.fail("application not found");
            };
            return fmt.out(&jsonrpc_proto::RpcKeyListResponse {
                action: RpcKeyAction::Add,
                status: RpcResponseStatus::OK,
                keys: keys.scan(&app),
            });
        }
    }
    Ok(())
}
