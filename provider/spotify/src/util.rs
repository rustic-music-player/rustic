use std::collections::HashMap;

use rspotify::model::artist::SimplifiedArtist;
use rspotify::model::image::Image;

use rustic_core::library::Artist;
use rustic_core::provider::ThumbnailState;
use rustic_core::ProviderType;

pub fn convert_images(images: &[Image]) -> ThumbnailState {
    images
        .first()
        .map(|image| image.url.clone())
        .map(ThumbnailState::Url)
        .unwrap_or_default()
}

pub fn artists_to_artist(artists: Vec<SimplifiedArtist>) -> Option<Artist> {
    if artists.is_empty() {
        return None;
    }
    let id: Vec<_> = artists
        .iter()
        .filter_map(|artist| artist.id.clone())
        .collect();
    let id = id
        .first()
        .cloned()
        .unwrap_or_else(|| String::from("unknown"));
    let uri = format!("spotify://artist/{}", id);
    let name = artists
        .into_iter()
        .map(|artist| artist.name)
        .collect::<Vec<String>>()
        .join(", ");
    Some(Artist {
        id: None,
        name,
        uri,
        image_url: None,
        meta: HashMap::new(),
        provider: ProviderType::Spotify,
        albums: Vec::new(),
        playlists: Vec::new(),
    })
}
