use viewmodels::AlbumModel;
use viewmodels::ArtistModel;
use viewmodels::PlaylistModel;
use viewmodels::TrackModel;

#[derive(Serialize)]
pub struct SearchResults {
    pub tracks: Vec<TrackModel>,
    pub albums: Vec<AlbumModel>,
    pub artists: Vec<ArtistModel>,
    pub playlists: Vec<PlaylistModel>,
}
