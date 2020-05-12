use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "type", content = "cursor")]
pub enum OpenResultModel {
    Track(String),
    Artist(String),
    Album(String),
    Playlist(String),
}
