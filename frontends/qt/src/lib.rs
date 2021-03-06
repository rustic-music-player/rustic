use rustic_api::ApiClient;

use std::ffi::CStr;

use qmetaobject::*;

qrc!(entrypoint, "rustic/qml" {
    "qml/main.qml" as "main.qml",
});

#[derive(QObject, Default)]
pub struct Frontend {
    base: qt_base_class!(trait QObject),
    exit: qt_method!(fn(&self)),
}

impl Frontend {
    fn exit(&self) {
        unimplemented!()
    }
}

pub fn start(_client: ApiClient) {
    entrypoint();
    qml_register_type::<Frontend>(
        CStr::from_bytes_with_nul(b"Rustic\0").unwrap(),
        1,
        0,
        CStr::from_bytes_with_nul(b"Frontend\0").unwrap(),
    );
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/rustic/qml/main.qml".into());
    engine.exec();
    println!("exec done");
}
