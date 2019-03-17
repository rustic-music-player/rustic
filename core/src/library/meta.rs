use serde_derive::{Serialize, Deserialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetaValue {
    Bool(bool),
    String(String),
    Float(f64),
    Int(u64),
}

impl From<bool> for MetaValue {
    fn from(value: bool) -> Self {
        MetaValue::Bool(value)
    }
}

impl From<String> for MetaValue {
    fn from(value: String) -> Self {
        MetaValue::String(value)
    }
}

impl From<f64> for MetaValue {
    fn from(value: f64) -> Self {
        MetaValue::Float(value)
    }
}

impl From<u64> for MetaValue {
    fn from(value: u64) -> Self {
        MetaValue::Int(value)
    }
}

impl From<usize> for MetaValue {
    fn from(value: usize) -> Self {
        MetaValue::Int(value as u64)
    }
}
