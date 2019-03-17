#[macro_use]
extern crate qmetaobject;
extern crate rustic_core as rustic;

use std::sync::Arc;
use qmetaobject::QmlEngine;

pub fn start(app: Arc<rustic::Rustic>) {
    let mut engine = QmlEngine::new();
    engine.load_data(include_str!("main.qml").into());
    engine.exec();
    println!("exec done");
}