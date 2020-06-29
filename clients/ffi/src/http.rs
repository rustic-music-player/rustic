use std::ptr;

use libc::*;

use rustic_native_http_client::RusticNativeHttpClient;

use crate::client::{Client, RusticClientHandle};
use crate::helpers::to_str;
use rustic_api::RusticApiClient;

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
