use crate::cursor::to_cursor;
use crate::models::*;
use rustic_core::extension::HostedExtension;
use rustic_core::provider::{
    AuthState, InternalUri, ProviderFolder, ProviderItem, ProviderItemType,
};
use rustic_core::{Album, Artist, Playlist, Provider, Track};

impl From<Album> for AlbumModel {
    fn from(album: Album) -> Self {
        AlbumModel {
            cursor: to_cursor(&album.uri),
            title: album.title,
            artist: album.artist.map(ArtistModel::from),
            tracks: album.tracks.into_iter().map(TrackModel::from).collect(),
            provider: album.provider.into(),
            coverart: album.image_url.clone(),
        }
    }
}

impl From<Artist> for ArtistModel {
    fn from(artist: Artist) -> Self {
        ArtistModel {
            cursor: to_cursor(&artist.uri),
            name: artist.name,
            albums: None,
            tracks: None,
            image: artist.image_url.clone(),
        }
    }
}

impl From<AuthState> for ProviderAuthenticationState {
    fn from(state: AuthState) -> Self {
        match state {
            AuthState::NoAuthentication => ProviderAuthenticationState::NoAuthentication,
            AuthState::RequiresOAuth(url) => {
                ProviderAuthenticationState::OAuthAuthentication { url }
            }
            AuthState::RequiresPassword => ProviderAuthenticationState::PasswordAuthentication,
            AuthState::Authenticated(_) => ProviderAuthenticationState::Authenticated,
        }
    }
}

impl From<&HostedExtension> for ExtensionModel {
    fn from(extension: &HostedExtension) -> Self {
        ExtensionModel {
            name: extension.name.clone(),
            id: extension.id.clone(),
            version: extension.version.clone(),
            enabled: true,
        }
    }
}

impl From<InternalUri> for OpenResultModel {
    fn from(uri: InternalUri) -> Self {
        match uri {
            InternalUri::Track(track_url) => OpenResultModel::Track(to_cursor(&track_url)),
            InternalUri::Album(track_url) => OpenResultModel::Album(to_cursor(&track_url)),
            InternalUri::Artist(track_url) => OpenResultModel::Artist(to_cursor(&track_url)),
            InternalUri::Playlist(track_url) => OpenResultModel::Playlist(to_cursor(&track_url)),
        }
    }
}

impl From<Playlist> for PlaylistModel {
    fn from(playlist: Playlist) -> Self {
        let tracks = playlist.tracks.into_iter().map(TrackModel::from).collect();

        PlaylistModel {
            cursor: to_cursor(&playlist.uri),
            title: playlist.title,
            tracks,
            provider: playlist.provider.into(),
        }
    }
}

impl From<Track> for TrackModel {
    fn from(track: Track) -> Self {
        let cursor = to_cursor(&track.uri);
        TrackModel {
            cursor: cursor.clone(),
            title: track.title,
            provider: track.provider.into(),
            coverart: if track.has_coverart {
                Some(format!("/api/tracks/{}/coverart", &cursor))
            } else {
                None
            },
            duration: track.duration,
            artist: track.artist.map(ArtistModel::from),
            album: track.album.map(AlbumModel::from),
        }
    }
}

impl From<ProviderFolder> for ProviderFolderModel {
    fn from(folder: ProviderFolder) -> Self {
        ProviderFolderModel {
            folders: folder.folders,
            items: folder
                .items
                .iter()
                .map(ProviderItemModel::from)
                .collect(),
        }
    }
}

impl From<&ProviderItem> for ProviderItemModel {
    fn from(item: &ProviderItem) -> Self {
        ProviderItemModel {
            label: item.label.clone(),
            data: ProviderItemTypeModel::from(&item.data),
        }
    }
}

impl From<&ProviderItemType> for ProviderItemTypeModel {
    fn from(item_type: &ProviderItemType) -> Self {
        match item_type {
            ProviderItemType::Track(track) => {
                ProviderItemTypeModel::Track(TrackModel::from(track.clone()))
            }
            ProviderItemType::Album(album) => {
                ProviderItemTypeModel::Album(AlbumModel::from(album.clone()))
            }
            ProviderItemType::Artist(artist) => {
                ProviderItemTypeModel::Artist(ArtistModel::from(artist.clone()))
            }
            ProviderItemType::Playlist(playlist) => {
                ProviderItemTypeModel::Playlist(PlaylistModel::from(playlist.clone()))
            }
        }
    }
}

impl From<Provider> for ProviderType {
    fn from(provider: Provider) -> Self {
        match provider {
            Provider::Pocketcasts => ProviderType::Pocketcasts,
            Provider::GooglePlayMusic => ProviderType::GooglePlayMusic,
            Provider::LocalMedia => ProviderType::LocalMedia,
            Provider::Soundcloud => ProviderType::Soundcloud,
            Provider::Spotify => ProviderType::Spotify,
        }
    }
}