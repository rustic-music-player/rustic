use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub enum MetaValueModel {
    Bool(bool),
    String(String),
    Float(f64),
    Int(u64),
}

impl Eq for MetaValueModel {}
