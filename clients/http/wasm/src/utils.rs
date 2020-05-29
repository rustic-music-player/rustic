use std::future::Future;

use failure::format_err;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use rustic_api::models::ProviderTypeModel;

pub type ApiResult = Result<JsValue, String>;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn serialize<T: Serialize>(target: T) -> Result<JsValue, String> {
    JsValue::from_serde(&target).map_err(|e| format!("{:?}", e))
}

pub fn map_failure(error: failure::Error) -> String {
    format!("{:?}", error)
}

pub fn map_value(value: JsValue) -> failure::Error {
    format_err!("{:?}", value)
}

pub fn map_providers(value: JsValue) -> Option<Vec<ProviderTypeModel>> {
    if js_sys::Array::is_array(&value) {
        let providers: js_sys::Array = value.into();
        let providers = providers
            .iter()
            .filter_map(|value| value.into_serde().ok())
            .collect();

        Some(providers)
    } else {
        None
    }
}

pub async fn execute<T, I>(future: T) -> ApiResult
where
    T: Future<Output = Result<I, failure::Error>>,
    I: Serialize,
{
    future.await.map_err(map_failure).and_then(serialize)
}
