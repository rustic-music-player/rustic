use crate::state::{AsyncData, Link, TrackList};
use druid::{Data, Lens};
use rustic_api::models::AlbumModel;
use std::sync::Arc;

impl From<&Arc<AlbumModel>> for Link {
    fn from(model: &Arc<AlbumModel>) -> Self {
        Link {
            cursor: model.cursor.clone(),
            title: model.title.clone(),
        }
    }
}

#[derive(Clone, Debug, Data, Lens, Default)]
pub struct AlbumState {
    pub album: AsyncData<Arc<AlbumModel>, Link>,
    pub tracks: AsyncData<TrackList, Link>,
}

impl From<&Arc<AlbumModel>> for TrackList {
    fn from(album: &Arc<AlbumModel>) -> Self {
        let link = album.into();
        let tracks = album.tracks.iter().cloned().map(Arc::new).collect();

        TrackList { link, tracks }
    }
}
