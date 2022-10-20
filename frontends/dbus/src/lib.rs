use std::collections::HashMap;

use failure::{Error};
use zbus::{ConnectionBuilder, dbus_interface};
use zbus::export::futures_util::StreamExt;
use zbus::zvariant::{Value};

use rustic_api::models::PlayerEventModel;
use rustic_api::ApiClient;

const MPRIS_PATH: &str = "/org/mpris/MediaPlayer2";

pub async fn start(client: ApiClient) -> Result<(), Error> {
    let player = MprisPlayer { client: client.clone() };

    let connection = ConnectionBuilder::session()?
        .name("org.mpris.MediaPlayer2.rustic")?
        .serve_at(MPRIS_PATH, MprisPlayerIdentity)?
        .serve_at(MPRIS_PATH, player)?
        .build()
        .await?;

    tokio::task::spawn(async move {
        let player_ref = connection
            .object_server()
            .interface::<_, MprisPlayer>(MPRIS_PATH)
            .await
            .unwrap();

        while let Some(event) = client.observe_player(None).next().await {
            let player = player_ref.get_mut().await;

            match event {
                PlayerEventModel::VolumeChanged(_) => player.volume_changed(player_ref.signal_context()).await.unwrap(),
                PlayerEventModel::StateChanged(_) => player.playback_status_changed(player_ref.signal_context()).await.unwrap(),
                PlayerEventModel::TrackChanged(_) => player.metadata_changed(player_ref.signal_context()).await.unwrap(),
                _ => {}
            }
        }
    });

    Ok(())
}

struct MprisPlayerIdentity;

#[dbus_interface(name = "org.mpris.MediaPlayer2")]
impl MprisPlayerIdentity {
    #[dbus_interface(property)]
    fn identity(&self) -> String {
        "Rustic Music Player".to_string()
    }

    #[dbus_interface(property)]
    fn can_quit(&self) -> bool {
        false
    }

    #[dbus_interface(property)]
    fn can_raise(&self) -> bool {
        false
    }

    #[dbus_interface(property)]
    fn has_track_list(&self) -> bool {
        false
    }

    #[dbus_interface(property)]
    fn supported_uri_schemes(&self) -> Vec<String> {
        vec![
            "http".to_string(),
            "file".to_string()
        ]
    }

    #[dbus_interface(property)]
    fn supported_mime_types(&self) -> Vec<String> {
        vec![
            "audio/mpeg".to_string(),
            "audio/wav".to_string()
        ]
    }
}

struct MprisPlayer {
    client: ApiClient
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl MprisPlayer {
    #[dbus_interface(property)]
    fn can_seek(&self) -> bool {
        false
    }

    #[dbus_interface(property)]
    fn can_go_next(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_go_previous(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_control(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_play(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_pause(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    async fn playback_status(&self) -> String {
        let player = self.client.get_player(None).await.unwrap().unwrap();

        if player.playing {
            "Playing"
        } else if player.current.is_some() {
            "Paused"
        } else {
            "Stopped"
        }.to_string()
    }

    #[dbus_interface(property)]
    async fn metadata(&self) -> HashMap<&str, Value> {
        let player = self.client.get_player(None).await.unwrap().unwrap();

        let mut metadata = HashMap::new();

        if let Some(track) = player.current {
            metadata.insert("mpris:trackid", track.cursor.into());
            metadata.insert("xesam:title", track.title.into());
            if let Some(duration) = track.duration {
                metadata.insert("mpris:length", duration.into());
            }
            if let Some(artist) = track.artist {
                metadata.insert("xesam:artist", artist.name.into());
            }
            if let Some(album) = track.album {
                metadata.insert("xesam:album", album.title.into());
            }
            if let Some(coverart) = track.coverart {
                metadata.insert("mpris:artUrl", format!("http://127.0.0.1:8080{coverart}").into()); // TODO: get http frontend base url
            }
        }

        metadata
    }

    #[dbus_interface(property)]
    async fn volume(&self) -> f32 {
        let player = self.client.get_player(None).await.unwrap().unwrap();

        player.volume
    }

    #[dbus_interface(property)]
    async fn set_volume(&self, volume: f64) -> zbus::Result<()> {
        self.client.player_set_volume(None, volume as f32).await.map_err(to_zbus_error)
    }

    async fn play(&self) -> zbus::fdo::Result<()> {
        self.client.player_control_play(None).await.map_err(to_zbus_fdo_error)
    }

    async fn pause(&self) -> zbus::fdo::Result<()> {
        self.client.player_control_pause(None).await.map_err(to_zbus_fdo_error)
    }

    async fn stop(&self) -> zbus::fdo::Result<()> {
        self.client.player_control_pause(None).await.map_err(to_zbus_fdo_error)?;
        self.client.clear_queue(None).await.map_err(to_zbus_fdo_error)?;

        Ok(())
    }

    async fn play_pause(&self) -> zbus::fdo::Result<()> {
        let player = self.client.get_player(None).await.map_err(to_zbus_fdo_error)?.unwrap();

        if player.playing {
            self.client.player_control_pause(None).await.map_err(to_zbus_fdo_error)?;
        }else {
            self.client.player_control_play(None).await.map_err(to_zbus_fdo_error)?;
        }

        Ok(())
    }

    async fn next(&self) -> zbus::fdo::Result<()> {
        self.client.player_control_next(None).await.map_err(to_zbus_fdo_error)?;

        Ok(())
    }

    async fn previous(&self) -> zbus::fdo::Result<()> {
        self.client.player_control_prev(None).await.map_err(to_zbus_fdo_error)?;

        Ok(())
    }
}

fn to_zbus_fdo_error(err: Error) -> zbus::fdo::Error {
    zbus::fdo::Error::Failed(err.to_string())
}

fn to_zbus_error(err: Error) -> zbus::Error {
    zbus::Error::FDO(Box::new(to_zbus_fdo_error(err)))
}
