use crate::client::{Client, RusticClientHandle};
use crate::models::player::Player;
use std::ptr;
use tokio::runtime;

#[no_mangle]
pub unsafe extern "C" fn client_get_default_player_sync(
    client: *mut RusticClientHandle,
) -> *const Player {
    let client = Client::from_ptr(client);
    let mut rt = runtime::Runtime::new().unwrap();

    let response = match rt.block_on(client.get_player()) {
        Ok(r) => r,
        Err(_) => return ptr::null(),
    };

    Box::into_raw(Box::new(response))
}
