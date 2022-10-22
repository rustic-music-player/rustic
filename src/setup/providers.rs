use log::{error, info};

use rustic_core::{CredentialStore, Provider};

use crate::config::Config;
use rustic_core::provider::ProviderInstance;

pub(crate) async fn setup_providers(
    config: &Config,
    cred_store: &dyn CredentialStore,
) -> Result<Vec<Provider>, failure::Error> {
    let mut providers: Vec<Box<dyn ProviderInstance + Send + Sync>> = vec![];

    #[cfg(feature = "pocketcasts-provider")]
    {
        if let Some(pocketcasts) = config.provider.pocketcasts.clone() {
            info!("Loading Pocketcasts Provider");
            providers.push(Box::new(pocketcasts));
        }
    }
    #[cfg(feature = "soundcloud-provider")]
    {
        if let Some(soundcloud) = config.provider.soundcloud.clone() {
            info!("Loading Soundcloud Provider");
            providers.push(Box::new(soundcloud));
        }
    }
    #[cfg(feature = "spotify-provider")]
    {
        if let Some(spotify) = config.provider.spotify.clone() {
            info!("Loading Spotify Provider");
            providers.push(Box::new(spotify));
        }
    }
    #[cfg(feature = "local-files-provider")]
    {
        if let Some(local) = config.provider.local.clone() {
            info!("Loading Local Provider");
            providers.push(Box::new(local));
        }
    }
    #[cfg(feature = "youtube-provider")]
    {
        if let Some(youtube) = config.provider.youtube.clone() {
            info!("Loading Youtube Provider");
            providers.push(Box::new(youtube));
        }
    }
    #[cfg(feature = "ytmusic-provider")]
    {
        if let Some(ytmusic) = config.provider.ytmusic.clone() {
            info!("Loading YouTube Music Provider");
            providers.push(Box::new(ytmusic));
        }
    }
    for provider in &mut providers {
        provider
            .setup(cred_store)
            .await
            .unwrap_or_else(|err| error!("Can't setup {} provider: {:?}", provider.title(), err));
    }

    let providers = providers.into_iter().map(Provider::from).collect();

    Ok(providers)
}
