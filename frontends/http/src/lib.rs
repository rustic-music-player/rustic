#[macro_use]
extern crate actix;

use std::sync::Arc;
use std::thread;

use serde::Deserialize;

use rustic_api::RusticApiClient;
use rustic_core::Rustic;

mod app;
mod controller;
mod cursor;
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
    config: Option<HttpConfig>,
    app: Arc<Rustic>,
    client: Arc<Box<dyn RusticApiClient>>,
) -> thread::JoinHandle<()> {
    let config = config.unwrap_or_default();
    thread::spawn(move || {
        app::start(&config, app, client).unwrap();
    })
}
