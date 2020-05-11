use std::ptr;

use libc::*;

#[cfg(feature = "http")]
use rustic_native_http_client::RusticNativeHttpClient;

pub use rustic_api::models::*;

use crate::client::{to_str, RusticClientHandle, Client};
use rustic_api::RusticApiClient;

#[cfg(feature = "async_extern_executor")]
mod async_extern_executor;
#[cfg(feature = "sync")]
mod sync;

pub mod models;

pub(crate) mod client;
pub(crate) mod error;

#[no_mangle]
#[cfg(feature = "http")]
pub unsafe extern "C" fn connect_http_client(url: *const c_char) -> *mut RusticClientHandle {
    let url = to_str(url);

    let url = match url {
        Ok(s) => s,
        Err(_) => return ptr::null_mut()
    };

    if let Some(url) = url {
        let client = RusticNativeHttpClient::new(url);
        let client: Box<dyn RusticApiClient> = Box::new(client);
        let client = Client::new(client);

        client.to_ptr()
    }else {
        ptr::null_mut()
    }
}
