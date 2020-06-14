use crate::library::{Album, Artist, Playlist, Track};
use serde_derive::{Deserialize, Serialize};
use crate::provider::ThumbnailState;

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct ProviderItem {
    pub label: String,
    pub data: ProviderItemType,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum ProviderItemType {
    Track(Track),
    Album(Album),
    Artist(Artist),
    Playlist(Playlist),
}

impl ProviderItem {
    pub fn is_track(&self) -> bool {
        if let ProviderItemType::Track(_) = self.data {
            true
        } else {
            false
        }
    }

    pub fn is_album(&self) -> bool {
        if let ProviderItemType::Album(_) = self.data {
            true
        } else {
            false
        }
    }

    pub fn is_artist(&self) -> bool {
        if let ProviderItemType::Artist(_) = self.data {
            true
        } else {
            false
        }
    }

    pub fn is_playlist(&self) -> bool {
        if let ProviderItemType::Playlist(_) = self.data {
            true
        } else {
            false
        }
    }
}

impl ProviderItemType {
    pub fn thumbnail(&self) -> ThumbnailState {
        use ProviderItemType::*;
        match self {
            Track(track) => track.thumbnail.clone(),
            Album(album) => album.thumbnail.clone(),
            Artist(artist) => artist.image_url.clone().map(ThumbnailState::Url).unwrap_or(ThumbnailState::None),
            Playlist(_) => ThumbnailState::None,
        }
    }
}

impl From<ProviderItem> for Track {
    fn from(item: ProviderItem) -> Track {
        match item.data {
            ProviderItemType::Track(track) => track,
            _ => panic!("ProviderItem is not of type Track"),
        }
    }
}

impl From<ProviderItem> for Artist {
    fn from(item: ProviderItem) -> Artist {
        match item.data {
            ProviderItemType::Artist(artist) => artist,
            _ => panic!("ProviderItem is not of type Artist"),
        }
    }
}

impl From<ProviderItem> for Album {
    fn from(item: ProviderItem) -> Album {
        match item.data {
            ProviderItemType::Album(album) => album,
            _ => panic!("ProviderItem is not of type Album"),
        }
    }
}

impl From<ProviderItem> for Playlist {
    fn from(item: ProviderItem) -> Playlist {
        match item.data {
            ProviderItemType::Playlist(playlist) => playlist,
            _ => panic!("ProviderItem is not of type Playlist"),
        }
    }
}

impl From<Track> for ProviderItem {
    fn from(track: Track) -> ProviderItem {
        ProviderItem {
            label: track.title.clone(),
            data: ProviderItemType::Track(track),
        }
    }
}

impl From<Album> for ProviderItem {
    fn from(album: Album) -> ProviderItem {
        ProviderItem {
            label: album.title.clone(),
            data: ProviderItemType::Album(album),
        }
    }
}

impl From<Artist> for ProviderItem {
    fn from(artist: Artist) -> ProviderItem {
        ProviderItem {
            label: artist.name.clone(),
            data: ProviderItemType::Artist(artist),
        }
    }
}

impl From<Playlist> for ProviderItem {
    fn from(playlist: Playlist) -> ProviderItem {
        ProviderItem {
            label: playlist.title.clone(),
            data: ProviderItemType::Playlist(playlist),
        }
    }
}
