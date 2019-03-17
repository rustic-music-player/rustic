use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;

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
    fn handle(&self, _app: &Arc<Rustic>) -> Result<Vec<OutputEntry>, Error> {
        Ok(vec![OutputEntry {
            id: 0,
            name: String::from("Default"),
            enabled: true,
        }])
    }
}
