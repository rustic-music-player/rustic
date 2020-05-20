use extern_executor::spawn;

use libc::*;
use std::ffi::CString;
use std::ptr::{null, null_mut};

use crate::client::{Client, RusticClientHandle};
use crate::error::FFIError;
use crate::models::player::Player;

fn handle_result<T>(result: Result<T, FFIError>, callback: fn(*mut c_char, *const T)) {
    match result {
        Ok(r) => callback(null_mut(), Box::into_raw(Box::new(r))),
        Err(e) => callback(CString::new(e.to_string()).unwrap().into_raw(), null()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn client_get_default_player_async(
    client: *mut RusticClientHandle,
    callback: fn(*mut c_char, *const Player),
) {
    let client = Client::from_ptr(client);

    spawn(async move {
        let result = client.get_player();
        handle_result(result.await, callback)
    });
}
