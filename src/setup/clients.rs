use std::sync::Arc;
use rustic_api::{RusticApiClient, ApiClient};
use crate::config::ClientConfig;
use rustic_core::Rustic;

pub(crate) fn setup_client(app: &Arc<Rustic>, config: &ClientConfig) -> ApiClient {
    let client: Box<dyn RusticApiClient> = match config {
        ClientConfig::Native => {
            let client = rustic_native_client::RusticNativeClient::new(Arc::clone(app));
            Box::new(client)
        },
        #[cfg(feature = "http-client")]
        ClientConfig::Http { url } => {
            let client = rustic_native_http_client::RusticNativeHttpClient::new(url);
            Box::new(client)
        }
    };
    Arc::new(client)
}
