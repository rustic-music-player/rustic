use cursor::to_cursor;
use rustic_core::provider::InternalUri;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "cursor")]
pub enum OpenResultModel {
    Track(String),
    Artist(String),
    Album(String),
    Playlist(String),
}

impl From<InternalUri> for OpenResultModel {
    fn from(uri: InternalUri) -> Self {
        match uri {
            InternalUri::Track(track_url) => OpenResultModel::Track(to_cursor(&track_url)),
            InternalUri::Album(track_url) => OpenResultModel::Album(to_cursor(&track_url)),
            InternalUri::Artist(track_url) => OpenResultModel::Artist(to_cursor(&track_url)),
            InternalUri::Playlist(track_url) => OpenResultModel::Playlist(to_cursor(&track_url)),
        }
    }
}
