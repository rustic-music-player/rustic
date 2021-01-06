use crate::commands;
use crate::delegate::PartialDelegate;
use crate::state::{AsyncData, Route, State};
use druid::{Command, Env, ExtEventSink, Target};
use futures::{future, future::BoxFuture, FutureExt};

#[derive(Default)]
pub struct NavigationDelegate;

impl NavigationDelegate {
    pub fn new() -> Self {
        NavigationDelegate::default()
    }
}

impl PartialDelegate for NavigationDelegate {
    fn command(
        &mut self,
        sink: ExtEventSink,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> Option<BoxFuture<'static, ()>> {
        if let Some(route) = cmd.get(commands::NAVIGATE) {
            data.route = route.clone();
            match route {
                Route::PlaylistDetails(link) => {
                    data.playlist.playlist = AsyncData::Pending(link.clone());
                    data.playlist.tracks = AsyncData::Pending(link.clone());
                    sink.submit_command(commands::LOAD_PLAYLIST, link.clone(), Target::Auto)
                        .unwrap();
                }
                _ => {}
            }
            Some(future::ready(()).boxed())
        } else {
            None
        }
    }
}
