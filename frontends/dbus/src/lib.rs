use std::collections::HashMap;
use std::sync::Arc;

use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus_crossroads::Crossroads;
use dbus_tokio::connection;
use failure::{format_err, Error};
use log::{debug, error};

use rustic_api::models::TrackModel;
use rustic_api::ApiClient;

struct PlayerState {
    playing: bool,
    track: Option<TrackModel>,
    volume: f32,
    client: ApiClient,
}

pub async fn start(client: ApiClient) -> Result<(), Error> {
    let (resource, conn) = connection::new_session_sync()?;

    tokio::spawn(async {
        let err = resource.await;
        error!("Lost connection to D-Bus: {}", err);
    });

    conn.request_name("org.mpris.MediaPlayer2.rustic", false, true, false)
        .await?;

    let mut cr = Crossroads::new();
    cr.set_async_support(Some((
        conn.clone(),
        Box::new(|x| {
            tokio::spawn(x);
        }),
    )));

    cr.insert("/", &[cr.introspectable(), cr.properties()], ());

    add_player(&mut cr, &client).await?;

    conn.start_receive(
        MatchRule::new_method_call(),
        Box::new(move |msg, conn| {
            debug!("Received msg {:?}", &msg);
            let msg_string = format!("{:?}", &msg);
            if cr.handle_message(msg, conn).is_err() {
                error!("Could not handle message {}", msg_string);
            }
            true
        }),
    );

    Ok(())
}

async fn add_player(cr: &mut Crossroads, client: &ApiClient) -> Result<(), Error> {
    let player = client
        .get_player(None)
        .await?
        .ok_or_else(|| format_err!("missing player"))?;
    let player_state = PlayerState {
        client: Arc::clone(client),
        playing: player.playing,
        track: player.current,
        volume: player.volume,
    };
    let identity_token = cr.register::<PlayerState, _, _>("org.mpris.MediaPlayer2", |b| {
        b.property("Identity")
            .get(|_, _| Ok(String::from("Rustic Music Player")));
        b.property("CanQuit").get(|_, _| Ok(false));
        b.property("CanRaise").get(|_, _| Ok(false));
        b.property("HasTrackList").get(|_, _| Ok(false));
        b.property("SupportedUriSchemes")
            .get(|_, _| Ok(vec!["http".to_string(), "file".to_string()]));
        b.property("SupportedMimeTypes")
            .get(|_, _| Ok(vec!["audio/mpeg".to_string(), "audio/wav".to_string()]));
    });

    let player_token = cr.register::<PlayerState, _, _>("org.mpris.MediaPlayer2.Player", |b| {
        b.property("PlaybackStatus").get(|_, state| {
            if state.playing {
                Ok(String::from("Playing"))
            } else if state.track.is_some() {
                Ok(String::from("Paused"))
            } else {
                Ok(String::from("Stopped"))
            }
        });
        b.property("Volume").get(|_, state| {
            let volume = state.volume as f64;
            Ok(volume)
        });
        b.property("CanSeek").get(|_, _| Ok(false));
        b.property("CanGoNext").get(|_, _| Ok(false));
        b.property("CanGoPrevious").get(|_, _| Ok(false));
        b.property("CanPlay").get(|_, _| Ok(true));
        b.property("CanPause").get(|_, _| Ok(false));
        b.property("Metadata").get(|_, state| {
            if let Some(track) = state.track.as_ref() {
                let mut metadata = HashMap::new();
                if let Some(ref album) = track.album {
                    metadata.insert("xesam:album".to_string(), album.title.clone());
                }
                metadata.insert("xesam:title".to_string(), track.title.clone());
                Ok(metadata)
            } else {
                Ok(HashMap::new())
            }
        });
        b.method_with_cr_async("Play", (), (), |mut ctx, cr, _: ()| {
            let client: ApiClient =
                Arc::clone(&cr.data_mut::<PlayerState>(ctx.path()).unwrap().client);
            async move {
                client.player_control_play(None).await.unwrap();
                ctx.reply_ok(())
            }
        });
        b.method_with_cr_async("Pause", (), (), |mut ctx, cr, _: ()| {
            let client: ApiClient =
                Arc::clone(&cr.data_mut::<PlayerState>(ctx.path()).unwrap().client);
            async move {
                client.player_control_pause(None).await.unwrap();
                ctx.reply_ok(())
            }
        });
        b.method_with_cr_async("Next", (), (), |mut ctx, cr, _: ()| {
            let client: ApiClient =
                Arc::clone(&cr.data_mut::<PlayerState>(ctx.path()).unwrap().client);
            async move {
                client.player_control_next(None).await.unwrap();
                ctx.reply_ok(())
            }
        });
        b.method_with_cr_async("Previous", (), (), |mut ctx, cr, _: ()| {
            let client: ApiClient =
                Arc::clone(&cr.data_mut::<PlayerState>(ctx.path()).unwrap().client);
            async move {
                client.player_control_prev(None).await.unwrap();
                ctx.reply_ok(())
            }
        });
    });

    cr.insert(
        "/org/mpris/MediaPlayer2",
        &[identity_token, player_token],
        player_state,
    );

    Ok(())
}
