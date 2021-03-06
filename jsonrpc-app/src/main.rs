pub mod args;
use jsonrpc_proto::formatter::Formatter;
use jsonrpc_proto::redis::{AppStorage, RedisConnection};
use jsonrpc_proto::Application;

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
    let mut storage = match AppStorage::from_redis(&conn) {
        Ok(x) => x,
        Err(e) => return fmt.wrap_error(e),
    };

    match args.cmd {
        args::Command::Add {
            name,
            slug,
            path,
            url,
            strip,
        } => {
            let doc = Application::new(&name, slug, path, url, strip);
            let key = doc.slug.to_owned();
            match storage.get(&key) {
                Some(_) => return fmt.fail("application already exists"),
                None => {
                    if let Err(e) = storage.set(&key, &doc) {
                        return fmt.wrap_error(e);
                    }
                    let updated = storage.get(&key).map(|x| x).unwrap();
                    fmt.out(&updated)
                }
            }
        }
        args::Command::Get { app } => {
            let key = app.clone();
            match storage.get(&key) {
                Some(doc) => fmt.out(&doc),
                None => return fmt.fail("application not found"),
            }
        }
        args::Command::Update {
            app,
            active,
            path,
            url,
            strip,
        } => {
            let key = app.clone();
            match storage.get(&key) {
                Some(orig) => {
                    let mut doc = orig.clone();
                    if let Some(active) = active {
                        doc.active = active;
                    }
                    if let Some(path) = path {
                        doc.proxy.path = path.to_owned();
                    }
                    if let Some(url) = url {
                        doc.proxy.path = url.to_owned();
                    }
                    if let Some(strip) = strip {
                        doc.proxy.strip = strip
                    }
                    if let Err(e) = storage.set(&key, &doc) {
                        return fmt.wrap_error(e);
                    }
                    let updated = storage.get(&key).map(|x| x).unwrap();
                    fmt.out(&updated)
                }
                None => return fmt.fail("application not found"),
            }
        }
        args::Command::List => fmt.out(&storage.scan()),
    }
}
