use std::collections::HashMap;

use rspotify::model::artist::SimplifiedArtist;
use rspotify::model::image::Image;

use rustic_core::library::Artist;

pub fn convert_images(images: &[Image]) -> Option<String> {
    images.first().map(|image| image.url.clone())
}

pub fn artists_to_artist(artists: Vec<SimplifiedArtist>) -> Option<Artist> {
    if artists.is_empty() {
        return None;
    }
    let name = artists
        .into_iter()
        .map(|artist| artist.name)
        .collect::<Vec<String>>()
        .join(", ");
    Some(Artist {
        id: None,
        name,
        uri: String::new(),
        image_url: None,
        meta: HashMap::new(),
    })
}
