use std::collections::HashMap;

use pocketcasts::{Podcast, SearchPodcast};
use rustic::library::{Album, Artist};
use rustic::provider::{Provider, ProviderFolder};

use meta::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketcastAlbum(Podcast);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketcastSearchResult(SearchPodcast);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketcastAlbums(Vec<Podcast>);

impl From<PocketcastAlbum> for Album {
    fn from(podcast: PocketcastAlbum) -> Album {
        let podcast = podcast.0;
        let thumbnail_url = podcast.thumbnail_url();
        let id = podcast.uuid.clone();
        Album {
            id: None,
            title: podcast.title,
            artist_id: None,
            artist: Some(Artist {
                id: None,
                uri: format!("pocketcasts://interpret/{}", podcast.author),
                name: podcast.author,
                image_url: None,
                meta: hashmap!(
                    META_POCKETCASTS_PODCAST_UUID => id.clone().into()
                ),
            }),
            provider: Provider::Pocketcasts,
            image_url: Some(thumbnail_url),
            uri: format!("pocketcasts://podcast/{}", podcast.uuid),
            meta: hashmap!(
                META_POCKETCASTS_PODCAST_UUID => id.into()
            ),
        }
    }
}

impl From<PocketcastAlbum> for Artist {
    fn from(podcast: PocketcastAlbum) -> Artist {
        let podcast = podcast.0;
        Artist {
            id: None,
            uri: format!("pocketcasts://interpret/{}", podcast.author),
            name: podcast.author,
            image_url: None,
            meta: HashMap::new(),
        }
    }
}

impl From<PocketcastAlbums> for ProviderFolder {
    fn from(podcasts: PocketcastAlbums) -> ProviderFolder {
        ProviderFolder {
            folders: podcasts.0
                .iter()
                .cloned()
                .map(|podcast| podcast.title)
                .collect(),
            items: vec![],
        }
    }
}

impl From<PocketcastSearchResult> for Album {
    fn from(podcast: PocketcastSearchResult) -> Album {
        let podcast = podcast.0;
        let thumbnail_url = podcast.thumbnail_url();
        Album {
            id: None,
            title: podcast.title,
            artist_id: None,
            artist: Some(Artist {
                id: None,
                uri: format!("pocketcasts://interpret/{}", podcast.author),
                name: podcast.author,
                image_url: None,
                meta: HashMap::new(),
            }),
            provider: Provider::Pocketcasts,
            image_url: Some(thumbnail_url),
            uri: format!("pocketcasts://podcast/{}", podcast.uuid),
            meta: HashMap::new(),
        }
    }
}

impl From<Podcast> for PocketcastAlbum {
    fn from(podcast: Podcast) -> Self {
        PocketcastAlbum(podcast)
    }
}

impl From<Vec<Podcast>> for PocketcastAlbums {
    fn from(podcasts: Vec<Podcast>) -> Self {
        PocketcastAlbums(podcasts)
    }
}

impl From<SearchPodcast> for PocketcastSearchResult {
    fn from(podcast: SearchPodcast) -> Self {
        PocketcastSearchResult(podcast)
    }
}
