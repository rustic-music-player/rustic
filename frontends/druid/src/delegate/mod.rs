use druid::{AppDelegate, Command, DelegateCtx, Env, ExtEventSink, Handled, Target};

use rustic_api::ApiClient;

use crate::commands;
use crate::state::State;

use self::album::AlbumDelegate;
use self::image::ImageDelegate;
use self::navigation::NavigationDelegate;
pub use self::partial::PartialDelegate;
use self::partial::*;
use self::playlist::PlaylistDelegate;
use crate::delegate::queue::QueueDelegate;

mod album;
mod image;
mod navigation;
mod partial;
mod playlist;

pub struct RusticDelegate {
    sink: ExtEventSink,
    delegates: Vec<Box<dyn PartialDelegate>>,
    runtime: tokio::runtime::Runtime,
}

impl RusticDelegate {
    pub fn new(sink: ExtEventSink, client: ApiClient) -> Result<Self, failure::Error> {
        sink.submit_command(commands::LOAD_PLAYLISTS, (), Target::Auto)?;
        sink.submit_command(commands::LOAD_ALBUMS, (), Target::Auto)?;
        Ok(RusticDelegate {
            sink,
            delegates: vec![
                PlaylistDelegate::new(client.clone()).boxed(),
                AlbumDelegate::new(client.clone()).boxed(),
                ImageDelegate::new().boxed(),
                NavigationDelegate::new().boxed(),
                QueueDelegate::new(client.clone()).boxed(),
            ],
            runtime: tokio::runtime::Runtime::new()?,
        })
    }
}

impl AppDelegate<State> for RusticDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        target: Target,
        cmd: &Command,
        data: &mut State,
        env: &Env,
    ) -> Handled {
        log::trace!("received command {:?}", &cmd);
        for delegate in &mut self.delegates {
            if let Some(future) = delegate.command(self.sink.clone(), target, cmd, data, env) {
                self.runtime.spawn(future);
                return Handled::Yes;
            }
        }
        Handled::No
    }
}

mod queue {
    use crate::commands;
    use crate::delegate::PartialDelegate;
    use crate::state::State;
    use druid::{Command, Env, ExtEventSink, Target};
    use futures::future::BoxFuture;
    use futures::FutureExt;
    use rustic_api::ApiClient;

    pub struct QueueDelegate {
        client: ApiClient,
    }

    impl QueueDelegate {
        pub fn new(client: ApiClient) -> Self {
            QueueDelegate { client }
        }
    }

    impl PartialDelegate for QueueDelegate {
        fn command(
            &mut self,
            _sink: ExtEventSink,
            _target: Target,
            cmd: &Command,
            _data: &mut State,
            _env: &Env,
        ) -> Option<BoxFuture<'static, ()>> {
            if let Some(track_link) = cmd.get(commands::QUEUE_TRACK) {
                let client = self.client.clone();
                let cursor = track_link.cursor.clone();

                Some(
                    async move {
                        client.queue_track(None, &cursor).await.unwrap();
                    }
                    .boxed(),
                )
            } else {
                None
            }
        }
    }
}
