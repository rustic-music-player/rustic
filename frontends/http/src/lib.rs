#[macro_use]
extern crate actix;
extern crate actix_files;
extern crate actix_web;
extern crate actix_web_actors;
extern crate base64;
extern crate crossbeam_channel;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate mime;
extern crate rayon;
extern crate rustic_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate stopwatch;
extern crate uuid;

use std::sync::Arc;
use std::thread;

use rustic_core::Rustic;

mod app;
mod controller;
mod cursor;
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
