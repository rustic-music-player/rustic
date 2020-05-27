use std::sync::Arc;

use rustic_api::{ApiClient, RusticApiClient};
use rustic_core::{Rustic, CredentialStore};
use rustic_extension_api::ExtensionManager;

use crate::config::ClientConfig;
use crate::options;

pub(crate) fn setup_client(app: &Arc<Rustic>, extensions: ExtensionManager, cred_store: Box<dyn CredentialStore>) -> ApiClient {
    let client: Box<dyn RusticApiClient> = {
        let client = rustic_native_client::RusticNativeClient::new(Arc::clone(app), extensions, cred_store);
        Box::new(client)
    };
    Arc::new(client)
}

pub(crate) fn setup_remote_client(options: &options::CliOptions, config: &ClientConfig) -> ApiClient {
    let client: Box<dyn RusticApiClient> = match config {
        #[cfg(feature = "http-client")]
        ClientConfig::Native if options.connect.is_some() => {
            let client = rustic_native_http_client::RusticNativeHttpClient::new(options.connect.as_ref().unwrap());
            Box::new(client)
        },
        #[cfg(feature = "http-client")]
        ClientConfig::Http { url } => {
            let client = rustic_native_http_client::RusticNativeHttpClient::new(url);
            Box::new(client)
        }
        _ => unreachable!()
    };
    Arc::new(client)
}
