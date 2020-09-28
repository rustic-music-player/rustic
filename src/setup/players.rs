use std::sync::Arc;

use rustic_core::player::{PlayerBuilder, queue::MemoryQueueBuilder};
use rustic_core::Rustic;
#[cfg(feature = "google-cast-backend")]
use rustic_google_cast_backend::GoogleCastBuilder;
#[cfg(feature = "gstreamer-backend")]
use rustic_gstreamer_backend::GstreamerPlayerBuilder;
#[cfg(feature = "rodio-backend")]
use rustic_rodio_backend::RodioPlayerBuilder;
#[cfg(feature = "snapcast-backend")]
use rustic_snapcast_backend::SnapcastPlayerBuilder;

use crate::config::{PlayerBackend, PlayerBackendConfig};

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
        #[cfg(feature = "snapcast-backend")]
        PlayerBackend::Snapcast { ref api_url, ref host, ref port, ref pipe } => {
            let transport = if let Some(pipe) = pipe {
                rustic_snapcast_backend::SnapcastAudioTransport::Pipe(pipe.clone())
            }else {
                let host = host.clone().unwrap_or_else(|| "127.0.0.1".to_string()); // TODO: get default from api url
                let port = port.unwrap_or(4953);
                rustic_snapcast_backend::SnapcastAudioTransport::Tcp {
                    host,
                    port
                }
            };
            let api_url = api_url.clone().unwrap_or_else(|| "http://localhost:1780".to_string());
            PlayerBuilder::new(Arc::clone(&app))
                .with_name(&name)
                .with_memory_queue()
                .with_snapcast(api_url, transport)?
                .build()
        },
    };
    app.add_player(name.clone(), player);
    if player_config.default {
        app.set_default_player(name);
    }
    Ok(())
}
