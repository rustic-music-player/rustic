use std::ptr;

use libc::*;
use tokio::runtime::Runtime;

use lazy_static::lazy_static;
use rustic_api::client::*;
#[cfg(feature = "http")]
use rustic_native_http_client::RusticNativeHttpClient;

use crate::client::{Client, RusticClientHandle, to_str};
use crate::models::*;

#[macro_use]
mod helpers;

pub mod models;

pub(crate) mod client;
pub(crate) mod error;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

rustic_ffi_client_macros::ffi_client!(RusticApiClient, crate::client::Client);
rustic_ffi_client_macros::ffi_client!(LibraryApiClient, crate::client::Client);

#[no_mangle]
#[cfg(feature = "http")]
pub unsafe extern "C" fn connect_http_client(url: *const c_char) -> *mut RusticClientHandle {
    let url = to_str(url);

    let url = match url {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    if let Some(url) = url {
        let client = RusticNativeHttpClient::new(url);
        let client: Box<dyn RusticApiClient> = Box::new(client);
        let client = Client::new(client);

        client.to_ptr()
    } else {
        ptr::null_mut()
    }
}
