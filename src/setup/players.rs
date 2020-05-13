use rustic_core::player::{queue::MemoryQueueBuilder, PlayerBuilder};
use crate::config::{PlayerBackend, PlayerBackendConfig};
use rustic_core::Rustic;
use std::sync::Arc;
#[cfg(feature = "google-cast-backend")]
use rustic_google_cast_backend::GoogleCastBuilder;
#[cfg(feature = "gstreamer-backend")]
use rustic_gstreamer_backend::GstreamerPlayerBuilder;
#[cfg(feature = "rodio-backend")]
use rustic_rodio_backend::RodioPlayerBuilder;

pub(crate) fn setup_player(
    app: &Arc<Rustic>,
    player_config: &PlayerBackendConfig,
) -> Result<(), failure::Error> {
    let name = player_config.name.clone();
    let player = match player_config.backend_type {
        #[cfg(feature = "gstreamer-backend")]
        PlayerBackend::GStreamer => PlayerBuilder::new(Arc::clone(&app))
            .with_name(&name)
            .with_memory_queue()
            .with_gstreamer()?
            .build(),
        #[cfg(feature = "google-cast-backend")]
        PlayerBackend::GoogleCast { ip } => PlayerBuilder::new(Arc::clone(&app))
            .with_name(&name)
            .with_memory_queue()
            .with_google_cast(ip)?
            .build(),
        #[cfg(feature = "rodio-backend")]
        PlayerBackend::Rodio => PlayerBuilder::new(Arc::clone(&app))
            .with_name(&name)
            .with_memory_queue()
            .with_rodio()?
            .build(),
    };
    app.add_player(name.clone(), player);
    if player_config.default {
        app.set_default_player(name);
    }
    Ok(())
}
