use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use crate::FutureExt;

#[derive(Debug, Serialize)]
pub struct OutputEntry {
    #[serde(rename = "outputid")]
    id: i64,
    #[serde(rename = "outputname")]
    name: String,
    #[serde(rename = "outputenabled")]
    enabled: bool,
}

pub struct OutputsCommand {}

impl OutputsCommand {
    pub fn new() -> OutputsCommand {
        OutputsCommand {}
    }
}

impl MpdCommand<Vec<OutputEntry>> for OutputsCommand {
    fn handle(&self, _: Arc<Rustic>, _client: ApiClient) -> BoxFuture<Result<Vec<OutputEntry>, Error>> {
        async move {
            Ok(vec![OutputEntry {
                id: 0,
                name: String::from("Default"),
                enabled: true,
            }])
        }.boxed()
    }
}
