use std::collections::HashMap;
use std::error::Error;

use serde_derive::{Deserialize, Serialize};

use crate::Track;

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
        _config: Option<HashMap<String, ExtensionConfigValue>>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_add_to_queue(&mut self, tracks: Vec<Track>) -> Result<Vec<Track>, Box<dyn Error>> {
        Ok(tracks)
    }
}
