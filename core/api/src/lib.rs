pub mod client;
#[cfg(feature = "conversion")]
mod conversion;
pub mod cursor;
pub mod models;
#[cfg(feature = "testing")]
pub mod test_client;

pub use self::client::RusticApiClient;

use rustic_reflect_macros::export_reflections;

#[cfg(feature = "testing")]
pub use self::test_client::TestApiClient;
use std::sync::Arc;

pub type ApiClient = Arc<Box<dyn RusticApiClient>>;

export_reflections!();
