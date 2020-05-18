use log::error;

use rustic_core::Provider;

use crate::config::Config;
use rustic_core::provider::ProviderInstance;

pub(crate) async fn setup_providers(config: &Config) -> Result<Vec<Provider>, failure::Error> {
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
    for provider in &mut providers {
        provider
            .setup()
            .await
            .unwrap_or_else(|err| error!("Can't setup {} provider: {:?}", provider.title(), err));
    }

    let providers = providers.into_iter()
        .map(Provider::from)
        .collect();

    Ok(providers)
}
