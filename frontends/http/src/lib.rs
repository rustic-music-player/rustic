extern crate rustic_core;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate mime;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate failure;
extern crate rayon;
extern crate uuid;

use rustic_core::Rustic;

use std::sync::Arc;

use std::thread;

mod app;
mod controller;
mod handler;
mod socket;
mod viewmodels;

#[derive(Deserialize, Clone)]
pub struct HttpConfig {
    pub ip: String,
    pub port: i32,
}

pub fn start(config: Option<HttpConfig>, app: Arc<Rustic>) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(HttpConfig {
        ip: "0.0.0.0".to_owned(),
        port: 8080,
    });
    thread::spawn(move || {
        app::start(&config, app).unwrap();
    })
}
