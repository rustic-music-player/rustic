use crate::config::Config;
use std::sync::{RwLock, Arc};
use log::error;
use futures::executor::block_on;

pub(crate) fn setup_providers(config: &Config) -> rustic_core::provider::SharedProviders {
    let mut providers: rustic_core::provider::SharedProviders = vec![];

    #[cfg(feature = "pocketcasts-provider")]
        {
            if let Some(pocketcasts) = config.provider.pocketcasts.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(pocketcasts))));
            }
        }
    #[cfg(feature = "soundcloud-provider")]
        {
            if let Some(soundcloud) = config.provider.soundcloud.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(soundcloud))));
            }
        }
    #[cfg(feature = "spotify-provider")]
        {
            if let Some(spotify) = config.provider.spotify.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(spotify))));
            }
        }
    #[cfg(feature = "local-files-provider")]
        {
            if let Some(local) = config.provider.local.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(local))));
            }
        }
    #[cfg(feature = "gmusic-provider")]
        {
            if let Some(gmusic) = config.provider.gmusic.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(gmusic))));
            }
        }
    for provider in &providers {
        let mut provider = provider.write().unwrap();
        block_on(provider
            .setup())
            .unwrap_or_else(|err| error!("Can't setup {} provider: {:?}", provider.title(), err));
    }

    providers
}
