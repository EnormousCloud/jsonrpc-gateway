pub mod args;
use jsonrpc_proto::formatter::Formatter;
use jsonrpc_proto::redis::{AppStorage, RedisConnection, RpcKeyStorage};
use jsonrpc_proto::{RpcKey, RpcKeyAction, RpcKeyResponse, RpcResponseStatus};

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
            return fmt.out(&RpcKeyResponse::Add {
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
            return fmt.out(&RpcKeyResponse::Get {
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
            let mut doc = match keys.get(&app, &key) {
                Some(x) => x,
                None => return fmt.fail("key not found"),
            };
            if let Some(expires) = expires {
                doc.expires = expires
            }
            if let Some(active) = active {
                doc.active = active
            }
            if let Some(quota_second) = quota_second {
                doc.quota_second = Some(quota_second)
            }
            if let Some(quota_minute) = quota_minute {
                doc.quota_minute = Some(quota_minute)
            }
            if let Some(quota_hour) = quota_hour {
                doc.quota_hour = Some(quota_hour)
            }
            if let Some(quota_day) = quota_day {
                doc.quota_day = Some(quota_day)
            }
            if let Some(quota_week) = quota_week {
                doc.quota_week = Some(quota_week)
            }
            if let Some(quota_month) = quota_month {
                doc.quota_month = Some(quota_month)
            }
            if let Some(quota_year) = quota_year {
                doc.quota_year = Some(quota_year)
            }
            for t in tag {
                match t.get(..1) {
                    Some("-") => {
                        let excluded: String = t.chars().skip(1).collect();
                        doc.tags = doc
                            .tags
                            .iter()
                            .filter(|x| **x != excluded)
                            .map(|x| x.to_string())
                            .collect();
                    }
                    _ => doc.tags.push(t),
                };
            }
            if let Err(e) = keys.set(&app, &key, &doc) {
                return fmt.wrap_error(e);
            }
            let updated = keys.get(&app, &key).map(|x| x).unwrap();
            return fmt.out(&RpcKeyResponse::Get {
                action: RpcKeyAction::Add,
                status: RpcResponseStatus::OK,
                key: updated,
            });
        }
        args::Command::List { app, .. } => {
            if let None = apps.get(&app) {
                return fmt.fail("application not found");
            };
            return fmt.out(&RpcKeyResponse::List {
                action: RpcKeyAction::List,
                status: RpcResponseStatus::OK,
                keys: keys.scan(&app),
            });
        }
    }
}
