use std::sync::Arc;

use failure::Error;
use rayon::prelude::*;

use rustic_core::{Album, Artist, Rustic, Track, Provider};
use viewmodels::*;

pub fn search(query: &str, provider_filter: Option<&Vec<Provider>>, rustic: &Arc<Rustic>) -> Result<SearchResults, Error> {
    let providers = &rustic.providers;
    trace!("search {}", query);

    let sw = stopwatch::Stopwatch::start_new();
    let results = providers
        .iter()
        .filter(|provider| {
            if let Some(provider_filter) = provider_filter {
                let p = provider.read().unwrap().provider();
                provider_filter.contains(&p)
            }else {
                true
            }
        })
        .map(|provider| provider.read().unwrap().search(query.to_string()))
        .collect::<Result<Vec<_>, Error>>()?;
    debug!("Searching took {}ms", sw.elapsed_ms());

    let tracks: Vec<TrackModel> = results
        .par_iter()
        .cloned()
        .flat_map(|items| items)
        .filter(|result| result.is_track())
        .map(Track::from)
        .map(|track| TrackModel::new(track))
        .collect();

    let albums: Vec<AlbumModel> = results
        .par_iter()
        .cloned()
        .flat_map(|items| items)
        .filter(|result| result.is_album())
        .map(Album::from)
        .map(|album| AlbumModel::new(album))
        .collect();

    let artists: Vec<ArtistModel> = results
        .par_iter()
        .cloned()
        .flat_map(|items| items)
        .filter(|result| result.is_artist())
        .map(Artist::from)
        .map(|artist| ArtistModel::new(artist))
        .collect();

    Ok(SearchResults {
        tracks,
        albums,
        artists,
        playlists: vec![],
    })
}

pub fn open(url: String, rustic: &Arc<Rustic>) -> Result<Option<OpenResultModel>, Error> {
    let internal_url = rustic.resolve_share_url(url)?;
    let internal_url = internal_url.map(OpenResultModel::from);

    Ok(internal_url)
}
