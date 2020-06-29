use tokio::runtime::Runtime;

use lazy_static::lazy_static;

use crate::client::RusticClientHandle;
use crate::models::*;

#[cfg(feature = "http")]
pub use self::http::*;

#[cfg(feature = "http")]
mod http;

#[macro_use]
mod helpers;

pub mod models;

pub(crate) mod client;
pub(crate) mod error;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

rustic_ffi_client_macros::ffi_client!(RusticApiClient, crate::client::Client);
rustic_ffi_client_macros::ffi_client!(ProviderApiClient, crate::client::Client);
rustic_ffi_client_macros::ffi_client!(LibraryApiClient, crate::client::Client);
// rustic_ffi_client_macros::ffi_client!(QueueApiClient, crate::client::Client);
rustic_ffi_client_macros::ffi_client!(PlaylistApiClient, crate::client::Client);
// rustic_ffi_client_macros::ffi_client!(PlayerApiClient, crate::client::Client);
