use std::convert::TryFrom;

use futures::StreamExt;

use rustic_core::provider::{
    Authentication, InternalUri, ProviderFolder, ProviderItem, ProviderItemType, ProviderState,
    Thumbnail,
};
use rustic_core::sync::{SyncEvent, SyncItem, SyncItemState};
use rustic_core::{Album, Artist, PlayerEvent, PlayerState, Playlist, ProviderType, QueuedTrack, RepeatMode, Track, Rating, TrackPosition};
use rustic_extension_api::ExtensionMetadata;

use crate::cursor::{from_cursor, to_cursor, Cursor};
use crate::models::*;
use rustic_core::library::MetaValue;

impl From<Album> for AlbumModel {
    fn from(album: Album) -> Self {
        let cursor = to_cursor(&album.uri);
        let mut tracks = album.tracks.into_iter().map(TrackModel::from).collect::<Vec<_>>();
        tracks.sort_by_key(|track| track.position);
        AlbumModel {
            cursor: cursor.clone(),
            title: album.title,
            artist: album.artist.map(ArtistModel::from),
            tracks,
            provider: album.provider.into(),
            in_library: album.id.is_some(),
            coverart: if album.thumbnail.has_thumbnail() {
                Some(format!("/api/albums/{}/coverart", &cursor))
            } else {
                None
            },
            meta: album.meta.into_iter().map(|(k, v)| (k, v.into())).collect(),
            explicit: album.explicit,
        }
    }
}

impl From<Artist> for ArtistModel {
    fn from(artist: Artist) -> Self {
        let cursor = to_cursor(&artist.uri);
        ArtistModel {
            cursor: cursor.clone(),
            name: artist.name,
            albums: Some(artist.albums.into_iter().map(AlbumModel::from).collect()),
            tracks: None,
            playlists: Some(
                artist
                    .playlists
                    .into_iter()
                    .map(PlaylistModel::from)
                    .collect(),
            ),
            image: artist
                .image_url
                .map(|_| format!("/api/artists/{}/coverart", &cursor)),
            provider: artist.provider.into(),
            meta: artist
                .meta
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<ProviderState> for ProviderStateModel {
    fn from(state: ProviderState) -> Self {
        match state {
            ProviderState::InvalidConfiguration(msg) => {
                ProviderStateModel::InvalidConfiguration(msg)
            }
            ProviderState::NoAuthentication => ProviderStateModel::NoAuthentication,
            ProviderState::RequiresOAuth(url) => ProviderStateModel::OAuthAuthentication { url },
            ProviderState::RequiresPassword => ProviderStateModel::PasswordAuthentication,
            ProviderState::Authenticated(_) => ProviderStateModel::Authenticated,
        }
    }
}

impl From<ExtensionMetadata> for ExtensionModel {
    fn from(metadata: ExtensionMetadata) -> Self {
        ExtensionModel {
            name: metadata.name,
            id: metadata.id,
            version: metadata.version,
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
            coverart: if track.thumbnail.has_thumbnail() {
                Some(format!("/api/tracks/{}/coverart", &cursor))
            } else {
                None
            },
            duration: track.duration,
            artist: track.artist.map(ArtistModel::from),
            album: track.album.map(AlbumModel::from),
            meta: track.meta.into_iter().map(|(k, v)| (k, v.into())).collect(),
            explicit: track.explicit,
            rating: track.rating.into(),
            position: track.position.map(TrackPositionModel::from),
        }
    }
}

impl From<TrackPosition> for TrackPositionModel {
    fn from(position: TrackPosition) -> Self {
        TrackPositionModel {
            track: position.track,
            disc: position.disc,
        }
    }
}

impl From<QueuedTrack> for QueuedTrackModel {
    fn from(track: QueuedTrack) -> Self {
        QueuedTrackModel {
            track: track.track.into(),
            playing: track.playing,
        }
    }
}

impl From<ProviderFolder> for ProviderFolderModel {
    fn from(folder: ProviderFolder) -> Self {
        ProviderFolderModel {
            folders: folder.folders,
            items: folder.items.iter().map(ProviderItemModel::from).collect(),
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

impl From<ProviderType> for ProviderTypeModel {
    fn from(provider: ProviderType) -> Self {
        match provider {
            ProviderType::Internal => ProviderTypeModel::Internal,
            ProviderType::Pocketcasts => ProviderTypeModel::Pocketcasts,
            ProviderType::GooglePlayMusic => ProviderTypeModel::GooglePlayMusic,
            ProviderType::LocalMedia => ProviderTypeModel::LocalMedia,
            ProviderType::Soundcloud => ProviderTypeModel::Soundcloud,
            ProviderType::Spotify => ProviderTypeModel::Spotify,
            ProviderType::Youtube => ProviderTypeModel::Youtube,
        }
    }
}

impl From<ProviderTypeModel> for ProviderType {
    fn from(provider: ProviderTypeModel) -> Self {
        match provider {
            ProviderTypeModel::Internal => ProviderType::Internal,
            ProviderTypeModel::Pocketcasts => ProviderType::Pocketcasts,
            ProviderTypeModel::GooglePlayMusic => ProviderType::GooglePlayMusic,
            ProviderTypeModel::LocalMedia => ProviderType::LocalMedia,
            ProviderTypeModel::Soundcloud => ProviderType::Soundcloud,
            ProviderTypeModel::Spotify => ProviderType::Spotify,
            ProviderTypeModel::Youtube => ProviderType::Youtube,
        }
    }
}

impl From<ProviderAuthModel> for Authentication {
    fn from(model: ProviderAuthModel) -> Self {
        match model {
            ProviderAuthModel::OAuthToken {
                state: Some(state),
                code,
                scope: _,
            } => Authentication::TokenWithState(code, state),
            ProviderAuthModel::OAuthToken {
                state: None,
                code,
                scope: _,
            } => Authentication::Token(code),
            ProviderAuthModel::UserPass { username, password } => {
                Authentication::Password(username, password)
            }
        }
    }
}

impl From<SyncEvent> for SyncStateModel {
    fn from(event: SyncEvent) -> Self {
        match event {
            SyncEvent::Synchronizing(items) => {
                SyncStateModel::Synchronizing(items.into_iter().map(SyncItemModel::from).collect())
            }
            SyncEvent::Idle => SyncStateModel::Idle,
        }
    }
}

impl From<SyncItem> for SyncItemModel {
    fn from(item: SyncItem) -> Self {
        SyncItemModel {
            provider: item.provider.into(),
            state: item.state.into(),
        }
    }
}

impl From<SyncItemState> for SyncItemStateModel {
    fn from(item: SyncItemState) -> Self {
        match item {
            SyncItemState::Idle => SyncItemStateModel::Idle,
            SyncItemState::Syncing => SyncItemStateModel::Syncing,
            SyncItemState::Done => SyncItemStateModel::Done,
            SyncItemState::Error => SyncItemStateModel::Error,
        }
    }
}

impl From<PlayerEvent> for PlayerEventModel {
    fn from(event: PlayerEvent) -> Self {
        match event {
            PlayerEvent::TrackChanged(track) => {
                PlayerEventModel::TrackChanged(TrackModel::from(track))
            }
            PlayerEvent::Buffering => PlayerEventModel::Buffering,
            PlayerEvent::Seek(seek) => PlayerEventModel::Seek(seek),
            PlayerEvent::StateChanged(state) => {
                PlayerEventModel::StateChanged(state == PlayerState::Play)
            }
            PlayerEvent::VolumeChanged(volume) => PlayerEventModel::VolumeChanged(volume),
            _ => unreachable!("this should be filtered before"),
        }
    }
}

impl From<PlayerEvent> for QueueEventModel {
    fn from(event: PlayerEvent) -> Self {
        match event {
            PlayerEvent::QueueUpdated(tracks) => QueueEventModel::QueueUpdated(
                tracks.into_iter().map(QueuedTrackModel::from).collect(),
            ),
            _ => unreachable!("this should be filtered before"),
        }
    }
}

impl From<Thumbnail> for CoverArtModel {
    fn from(cover_art: Thumbnail) -> Self {
        match cover_art {
            Thumbnail::Url(url) => CoverArtModel::Url(url),
            Thumbnail::Data { data, mime_type } => {
                let stream = futures::stream::once(async { data });

                CoverArtModel::Data {
                    data: stream.boxed(),
                    mime_type,
                }
            }
        }
    }
}

impl TryFrom<Cursor> for InternalUri {
    type Error = failure::Error;

    fn try_from(cursor: Cursor) -> Result<Self, Self::Error> {
        use Cursor::*;

        let cursor = match cursor {
            Track(cursor) => InternalUri::Track(from_cursor(&cursor)?),
            Album(cursor) => InternalUri::Album(from_cursor(&cursor)?),
            Artist(cursor) => InternalUri::Artist(from_cursor(&cursor)?),
            Playlist(cursor) => InternalUri::Playlist(from_cursor(&cursor)?),
        };
        Ok(cursor)
    }
}

impl From<InternalUri> for Cursor {
    fn from(uri: InternalUri) -> Self {
        use InternalUri::*;

        match uri {
            Track(uri) => Cursor::Track(to_cursor(&uri)),
            Album(uri) => Cursor::Album(to_cursor(&uri)),
            Artist(uri) => Cursor::Artist(to_cursor(&uri)),
            Playlist(uri) => Cursor::Playlist(to_cursor(&uri)),
        }
    }
}

impl From<RepeatMode> for RepeatModeModel {
    fn from(repeat: RepeatMode) -> Self {
        match repeat {
            RepeatMode::None => RepeatModeModel::None,
            RepeatMode::Single => RepeatModeModel::Single,
            RepeatMode::All => RepeatModeModel::All,
        }
    }
}

impl From<RepeatModeModel> for RepeatMode {
    fn from(repeat: RepeatModeModel) -> Self {
        match repeat {
            RepeatModeModel::None => RepeatMode::None,
            RepeatModeModel::Single => RepeatMode::Single,
            RepeatModeModel::All => RepeatMode::All,
        }
    }
}

impl From<MetaValue> for MetaValueModel {
    fn from(value: MetaValue) -> Self {
        match value {
            MetaValue::String(string) => MetaValueModel::String(string),
            MetaValue::Int(int) => MetaValueModel::Int(int),
            MetaValue::Float(float) => MetaValueModel::Float(float),
            MetaValue::Bool(bool) => MetaValueModel::Bool(bool),
        }
    }
}

impl From<Rating> for RatingModel {
    fn from(rating: Rating) -> Self {
        match rating {
            Rating::None => RatingModel::None,
            Rating::Like => RatingModel::Like,
            Rating::Dislike => RatingModel::Dislike,
            Rating::Stars(stars) => RatingModel::Stars(stars)
        }
    }
}
