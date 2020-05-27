use log::error;

use rustic_core::{Provider, CredentialStore};

use crate::config::Config;
use rustic_core::provider::ProviderInstance;

pub(crate) async fn setup_providers(config: &Config, cred_store: &dyn CredentialStore) -> Result<Vec<Provider>, failure::Error> {
    let mut providers: Vec<Box<dyn ProviderInstance + Send + Sync>> = vec![];

    #[cfg(feature = "pocketcasts-provider")]
        {
            if let Some(pocketcasts) = config.provider.pocketcasts.clone() {
                providers.push(Box::new(pocketcasts));
            }
        }
    #[cfg(feature = "soundcloud-provider")]
        {
            if let Some(soundcloud) = config.provider.soundcloud.clone() {
                providers.push(Box::new(soundcloud));
            }
        }
    #[cfg(feature = "spotify-provider")]
        {
            if let Some(spotify) = config.provider.spotify.clone() {
                providers.push(Box::new(spotify));
            }
        }
    #[cfg(feature = "local-files-provider")]
        {
            if let Some(local) = config.provider.local.clone() {
                providers.push(Box::new(local));
            }
        }
    #[cfg(feature = "gmusic-provider")]
        {
            if let Some(gmusic) = config.provider.gmusic.clone() {
                providers.push(Box::new(gmusic));
            }
        }
    #[cfg(feature = "youtube-provider")]
        {
            if let Some(youtube) = config.provider.youtube.clone() {
                providers.push(Box::new(youtube));
            }
        }
    for provider in &mut providers {
        provider
            .setup(cred_store)
            .await
            .unwrap_or_else(|err| error!("Can't setup {} provider: {:?}", provider.title(), err));
    }

    let providers = providers.into_iter()
        .map(Provider::from)
        .collect();

    Ok(providers)
}
