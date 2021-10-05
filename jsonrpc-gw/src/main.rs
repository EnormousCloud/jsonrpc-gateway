pub mod args;
pub mod telemetry;

#[derive(Clone)]
pub struct State {}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("args parsing error {}", e))),
    };

    println!("JSONRPC-GW");
    let state = State {};
    let mut app = tide::with_state(state);
    app.with(telemetry::TraceMiddleware::new());
    app.listen(args.addr.as_str()).await?;
    Ok(())
}
