#[macro_use]
extern crate actix;

use std::sync::Arc;
use std::thread;

use serde::Deserialize;

use rustic_core::Rustic;

mod app;
mod controller;
mod cursor;
mod handler;
mod socket;

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

pub fn start(config: Option<HttpConfig>, app: Arc<Rustic>) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(HttpConfig::default());
    thread::spawn(move || {
        app::start(&config, app).unwrap();
    })
}
