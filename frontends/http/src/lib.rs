#[macro_use]
extern crate actix;

use std::sync::Arc;
use std::thread;

use serde::Deserialize;

use rustic_api::ApiClient;
use rustic_core::Rustic;

mod app;
mod controller;
mod socket;
#[cfg(test)]
pub(crate) mod test;

#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct HttpConfig {
    pub ip: String,
    pub port: i32,
    pub static_files: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            ip: "0.0.0.0".into(),
            port: 8080,
            static_files: "static".into(),
        }
    }
}

pub fn start(
    handle: tokio::runtime::Handle,
    config: Option<HttpConfig>,
    app: Arc<Rustic>,
    client: ApiClient,
) -> thread::JoinHandle<()> {
    let config = config.unwrap_or_default();
    thread::spawn(move || {
        handle.block_on(async {
            let local = tokio::task::LocalSet::new();
            local.run_until(async {
                local.spawn_local(async move {
                    app::start(&config, app, client).await.unwrap();
                }).await
            }).await
        }).unwrap();
    })
}
