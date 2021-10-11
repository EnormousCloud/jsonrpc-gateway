use crate::State;
use std::time::Duration;
use tide::{Error, Request, Response, Result};
use tracing::info;
use ureq::{Agent, AgentBuilder};

// "/rpc/{key}"
pub async fn proxy_rpc(mut req: Request<State>) -> Result {
    let path = req.url().path().chars().skip(1).collect::<String>();
    let used_key = match req.header("X-Key") {
        Some(x) => x.to_string(),
        None => path,
    };

    let body = req.body_string().await.expect("payload expected");
    let state = req.state();
    let mut guard = state.rpckeys.lock().expect("mutex lock error");
    let rpc_key = match guard.get(&state.default_app.name, &used_key) {
        Some(x) => x,
        None => return Err(Error::from_str(403, "access denied")),
    };
    info!(
        "used_key = {} details = {:?} proxy = {:?} payload = {}",
        used_key, rpc_key, state.default_app.proxy, body
    );

    let rpc_url = state.default_app.proxy.url.clone();
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(30))
        .timeout_write(Duration::from_secs(5))
        .build();
    let resp = agent
        .post(&rpc_url)
        .set("Content-Type", "application/json")
        .send_string(&body)?;

    let mut res = Response::new(resp.status());
    let body = resp.into_string().unwrap();
    res.set_body(body);
    Ok(res)
}
