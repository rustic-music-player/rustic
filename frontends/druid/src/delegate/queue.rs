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
