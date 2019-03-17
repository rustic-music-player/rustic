use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;

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
    fn handle(&self, _app: &Arc<Rustic>) -> Result<Vec<TagType>, Error> {
        Ok(vec![TagType::new("Track")])
    }
}
