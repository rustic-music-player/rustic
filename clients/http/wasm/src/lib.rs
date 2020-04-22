use wasm_bindgen::prelude::*;
use rustic_http_client::HttpClient;

mod utils;
mod client;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CLIENT: client::RusticHttpClient = client::RusticHttpClient::new();

#[wasm_bindgen]
pub async fn get_players() -> Result<JsValue, JsValue> {
    let players = CLIENT.get_players().await?;

    Ok(JsValue::from_serde(&players).unwrap())
}