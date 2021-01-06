use crate::state::AsyncData;
use druid::im::Vector;
use druid::{Data, Lens};
use rustic_api::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};
use std::sync::Arc;

#[derive(Clone, Data, Default, Lens)]
pub struct SearchState {
    pub query: String,
    pub results: AsyncData<SearchResults, String>,
}

#[derive(Clone, Data, Default, Lens)]
pub struct SearchResults {
    pub tracks: Vector<Arc<TrackModel>>,
    pub albums: Vector<Arc<AlbumModel>>,
    pub artists: Vector<Arc<ArtistModel>>,
    pub playlists: Vector<Arc<PlaylistModel>>,
}
