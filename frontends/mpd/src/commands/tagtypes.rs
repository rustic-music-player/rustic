use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct TagTypesCommand {}

#[derive(Serialize, Debug)]
pub struct TagType {
    tagtype: String,
}

impl TagType {
    fn new(label: &'static str) -> TagType {
        TagType {
            tagtype: label.to_owned(),
        }
    }
}

impl TagTypesCommand {
    pub fn new() -> TagTypesCommand {
        TagTypesCommand {}
    }
}

impl MpdCommand<Vec<TagType>> for TagTypesCommand {
    fn handle(&self, _: Arc<Rustic>, _client: ApiClient) -> BoxFuture<Result<Vec<TagType>, Error>> {
        async move {
            Ok(vec![TagType::new("Track")])
        }.boxed()
    }
}
