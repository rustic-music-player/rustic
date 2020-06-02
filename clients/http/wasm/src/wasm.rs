use wasm_bindgen::prelude::*;

use rustic_api::client::*;
use rustic_http_client::*;

use crate::client::RusticWasmHttpClient;
use crate::utils::{execute, map_providers, ApiResult};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CLIENT: RusticHttpClient<RusticWasmHttpClient> = RusticWasmHttpClient::new();

// RusticApiClient

#[wasm_bindgen]
pub async fn search(query: String, providers: JsValue) -> ApiResult {
    execute(CLIENT.search(&query, map_providers(providers))).await
}

#[wasm_bindgen(js_name = "getExtensions")]
pub async fn get_extensions() -> ApiResult {
    execute(CLIENT.get_extensions()).await
}

#[wasm_bindgen(js_name = "openShareUrl")]
pub async fn open_share_url(url: String) -> ApiResult {
    execute(CLIENT.open_share_url(&url)).await
}

// ProviderApiClient

#[wasm_bindgen(js_name = "getProviders")]
pub async fn get_providers() -> ApiResult {
    execute(CLIENT.get_providers()).await
}

#[wasm_bindgen(js_name = "getAvailableProviders")]
pub async fn get_available_providers() -> ApiResult {
    execute(CLIENT.get_available_providers()).await
}

// LibraryApiClient

#[wasm_bindgen(js_name = "getAlbums")]
pub async fn get_albums(providers: JsValue) -> ApiResult {
    execute(CLIENT.get_albums(map_providers(providers))).await
}

#[wasm_bindgen(js_name = "getAlbum")]
pub async fn get_album(cursor: String) -> ApiResult {
    execute(CLIENT.get_album(&cursor)).await
}

#[wasm_bindgen(js_name = "getArtists")]
pub async fn get_artists() -> ApiResult {
    execute(CLIENT.get_artists()).await
}

#[wasm_bindgen(js_name = "getArtist")]
pub async fn get_artist(cursor: String) -> ApiResult {
    execute(CLIENT.get_artist(&cursor)).await
}

#[wasm_bindgen(js_name = "getPlaylists")]
pub async fn get_playlists(providers: JsValue) -> ApiResult {
    execute(CLIENT.get_playlists(map_providers(providers))).await
}

#[wasm_bindgen(js_name = "getPlaylist")]
pub async fn get_playlist(cursor: String) -> ApiResult {
    execute(CLIENT.get_playlist(&cursor)).await
}

#[wasm_bindgen(js_name = "getTracks")]
pub async fn get_tracks(providers: JsValue) -> ApiResult {
    execute(CLIENT.get_tracks(map_providers(providers))).await
}

#[wasm_bindgen(js_name = "getTrack")]
pub async fn get_track(cursor: String) -> ApiResult {
    execute(CLIENT.get_track(&cursor)).await
}

// QueueApiClient

#[wasm_bindgen(js_name = "getQueue")]
pub async fn get_queue(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.get_queue(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "queueTrack")]
pub async fn queue_track(player_id: Option<String>, cursor: String) -> ApiResult {
    execute(CLIENT.queue_track(player_id.as_deref(), &cursor)).await
}

#[wasm_bindgen(js_name = "queueAlbum")]
pub async fn queue_album(player_id: Option<String>, cursor: String) -> ApiResult {
    execute(CLIENT.queue_album(player_id.as_deref(), &cursor)).await
}

#[wasm_bindgen(js_name = "queuePlaylist")]
pub async fn queue_playlist(player_id: Option<String>, cursor: String) -> ApiResult {
    execute(CLIENT.queue_playlist(player_id.as_deref(), &cursor)).await
}

#[wasm_bindgen(js_name = "clearQueue")]
pub async fn clear_queue(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.clear_queue(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "removeQueueItem")]
pub async fn remove_queue_item(player_id: Option<String>, item: usize) -> ApiResult {
    execute(CLIENT.remove_queue_item(player_id.as_deref(), item)).await
}

#[wasm_bindgen(js_name = "reorderQueueItem")]
pub async fn reorder_queue_item(
    player_id: Option<String>,
    before: usize,
    after: usize,
) -> ApiResult {
    execute(CLIENT.reorder_queue_item(player_id.as_deref(), before, after)).await
}

// PlayerApiClient

#[wasm_bindgen(js_name = "getPlayers")]
pub async fn get_players() -> ApiResult {
    execute(CLIENT.get_players()).await
}

#[wasm_bindgen(js_name = "getPlayer")]
pub async fn get_player(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.get_player(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "playerControlNext")]
pub async fn player_control_next(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.player_control_next(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "playerControlPrev")]
pub async fn player_control_prev(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.player_control_prev(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "playerControlPlay")]
pub async fn player_control_play(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.player_control_play(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "playerControlPause")]
pub async fn player_control_pause(player_id: Option<String>) -> ApiResult {
    execute(CLIENT.player_control_pause(player_id.as_deref())).await
}

#[wasm_bindgen(js_name = "playerSetVolume")]
pub async fn player_set_volume(player_id: Option<String>, volume: f32) -> ApiResult {
    execute(CLIENT.player_set_volume(player_id.as_deref(), volume)).await
}
