use druid::{AppDelegate, Command, DelegateCtx, Env, ExtEventSink, Handled, Target};

use rustic_api::ApiClient;

use crate::commands;
use crate::state::State;

use self::album::AlbumDelegate;
use self::image::ImageDelegate;
use self::library::LibraryDelegate;
use self::navigation::NavigationDelegate;
pub use self::partial::PartialDelegate;
use self::partial::*;
use self::playlist::PlaylistDelegate;
use self::queue::QueueDelegate;
use self::stream::*;

mod album;
mod image;
mod library;
mod navigation;
mod partial;
mod playlist;
mod queue;
mod stream;

pub struct RusticDelegate {
    sink: ExtEventSink,
    partial_delegates: Vec<Box<dyn PartialDelegate>>,
    stream_delegates: Vec<Box<dyn StreamDelegate>>,
    runtime: tokio::runtime::Runtime,
}

impl RusticDelegate {
    pub fn new(sink: ExtEventSink, client: ApiClient) -> Result<Self, failure::Error> {
        sink.submit_command(commands::LOAD_PLAYLISTS, (), Target::Auto)?;
        sink.submit_command(commands::LOAD_ALBUMS, (), Target::Auto)?;
        let delegate = RusticDelegate {
            sink,
            partial_delegates: vec![
                PlaylistDelegate::new(client.clone()).boxed(),
                AlbumDelegate::new(client.clone()).boxed(),
                ImageDelegate::new().boxed(),
                NavigationDelegate::new().boxed(),
                QueueDelegate::new(client.clone()).boxed(),
            ],
            stream_delegates: vec![LibraryDelegate::new(client.clone()).boxed()],
            runtime: tokio::runtime::Runtime::new()?,
        };
        delegate.start_streams();
        Ok(delegate)
    }

    fn start_streams(&self) {
        for stream_delegate in &self.stream_delegates {
            self.runtime
                .spawn(stream_delegate.observe(self.sink.clone()));
        }
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
        for delegate in &mut self.partial_delegates {
            if let Some(future) = delegate.command(self.sink.clone(), target, cmd, data, env) {
                self.runtime.spawn(future);
                return Handled::Yes;
            }
        }
        Handled::No
    }
}
