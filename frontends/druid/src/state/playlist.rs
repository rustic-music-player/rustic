use crate::state::{AsyncData, Link, TrackList};
use druid::{Data, Lens};
use rustic_api::models::PlaylistModel;
use std::sync::Arc;

impl From<&Arc<PlaylistModel>> for Link {
    fn from(model: &Arc<PlaylistModel>) -> Self {
        Link {
            cursor: model.cursor.clone(),
            title: model.title.clone(),
        }
    }
}

#[derive(Clone, Debug, Data, Lens, Default)]
pub struct PlaylistState {
    pub playlist: AsyncData<Arc<PlaylistModel>, Link>,
    pub tracks: AsyncData<TrackList, Link>,
}

impl From<&Arc<PlaylistModel>> for TrackList {
    fn from(playlist: &Arc<PlaylistModel>) -> Self {
        let link = playlist.into();
        let tracks = playlist.tracks.iter().cloned().map(Arc::new).collect();

        TrackList { link, tracks }
    }
}
