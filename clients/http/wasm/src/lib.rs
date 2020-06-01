#[cfg(target_arch = "wasm32")]
mod client;
#[cfg(target_arch = "wasm32")]
mod client_interface;
#[cfg(target_arch = "wasm32")]
mod utils;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
