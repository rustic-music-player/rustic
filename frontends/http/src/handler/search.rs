use failure::Error;
use rayon::prelude::*;
use rustic_core::{Album, Artist, Rustic, Track};
use std::sync::Arc;
use viewmodels::*;

pub fn search(query: &str, rustic: &Arc<Rustic>) -> Result<SearchResults, Error> {
    let providers = &rustic.providers;
    trace!("search {}", query);

    let sw = stopwatch::Stopwatch::start_new();
    let results = providers
        .iter()
        .map(|provider| provider.read().unwrap().search(query.to_string()))
        .collect::<Result<Vec<_>, Error>>()?;
    debug!("Searching took {}ms", sw.elapsed_ms());

    let tracks: Vec<TrackModel> = results
        .par_iter()
        .cloned()
        .flat_map(|items| items)
        .filter(|result| result.is_track())
        .map(Track::from)
        .map(|track| TrackModel::new(track, rustic))
        .collect();

    let albums: Vec<AlbumModel> = results
        .par_iter()
        .cloned()
        .flat_map(|items| items)
        .filter(|result| result.is_album())
        .map(Album::from)
        .map(|album| AlbumModel::new(album, rustic))
        .collect();

    let artists: Vec<ArtistModel> = results
        .par_iter()
        .cloned()
        .flat_map(|items| items)
        .filter(|result| result.is_artist())
        .map(Artist::from)
        .map(|artist| ArtistModel::new(artist, rustic))
        .collect();

    Ok(SearchResults {
        tracks,
        albums,
        artists,
        playlists: vec![],
    })
}
