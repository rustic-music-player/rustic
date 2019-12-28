use crate::Track;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExtensionConfigValue {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
}

pub trait Extension: std::fmt::Debug {
    fn setup(
        &mut self,
        config: Option<HashMap<String, ExtensionConfigValue>>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_add_to_queue(&mut self, tracks: Vec<Track>) -> Result<Vec<Track>, Box<dyn Error>> {
        Ok(tracks)
    }
}
