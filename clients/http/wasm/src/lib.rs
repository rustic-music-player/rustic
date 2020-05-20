use rustic_api::RusticApiClient;
use rustic_http_client::*;
use wasm_bindgen::prelude::*;

mod client;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CLIENT: RusticHttpClient<client::RusticWasmHttpClient> = client::RusticWasmHttpClient::new();

#[wasm_bindgen]
pub async fn get_players() -> Result<JsValue, JsValue> {
    let players = CLIENT.get_players().await?;

    Ok(JsValue::from_serde(&players).unwrap())
}

#[wasm_bindgen]
pub async fn search(query: JsValue) -> Result<JsValue, JsValue> {
    if let Some(query) = query.as_string() {
        let result = CLIENT.search(query).await?;

        Ok(JsValue::from_serde(&result).unwrap())
    } else {
        Err(JsValue::from_str(""))
    }
}
