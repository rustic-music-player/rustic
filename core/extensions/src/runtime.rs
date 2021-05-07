use std::fmt::Formatter;
use std::sync::Arc;

use failure::{Error, format_err};
use futures::stream::BoxStream;

use rustic_core::{
    Album, Artist, InternalUri, Library, LibraryEvent, MultiQuery, Player, Playlist,
    provider::{ProviderItemType, Thumbnail}, Rustic, SearchResults, SharedLibrary, SharedStorageBackend, SingleQuery, Track,
};
use rustic_core::library::MetaValue;

use crate::ExtensionMetadata;

#[derive(Clone)]
pub struct ExtensionRuntime {
    app: Arc<Rustic>,
    pub(crate) storage: SharedStorageBackend,
    extension: Option<ExtensionMetadata>,
}

impl std::fmt::Debug for ExtensionRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExtensionRuntime")
            .field("storage", &self.storage)
            .field("extension", &self.extension)
            .finish()
    }
}

impl ExtensionRuntime {
    pub fn new(app: Arc<Rustic>, storage: SharedStorageBackend) -> Self {
        ExtensionRuntime {
            app,
            storage,
            extension: None,
        }
    }

    pub fn for_extension(&self, metadata: ExtensionMetadata) -> Self {
        ExtensionRuntime {
            app: self.app.clone(),
            storage: self.storage.clone(),
            extension: Some(metadata),
        }
    }

    pub async fn read_metadata(&self, key: &str) -> Result<Option<MetaValue>, Error> {
        if let Some(ref extension) = self.extension {
            let collection = self.storage.open_collection(&extension.id).await?;
            let value = collection.read(key).await?;

            Ok(value)
        } else {
            Err(format_err!("ExtensionRuntime is not setup properly"))
        }
    }

    pub async fn write_metadata(&self, key: &str, value: MetaValue) -> Result<(), Error> {
        if let Some(ref extension) = self.extension {
            let collection = self.storage.open_collection(&extension.id).await?;
            let value = collection.write(key, value).await?;

            Ok(value)
        } else {
            Err(format_err!("ExtensionRuntime is not setup properly"))
        }
    }

    pub async fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, failure::Error> {
        self.app.query_track(query).await
    }

    pub async fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, failure::Error> {
        self.app.query_album(query).await
    }

    pub async fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, failure::Error> {
        self.app.query_artist(query).await
    }

    pub async fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, failure::Error> {
        self.app.query_playlist(query).await
    }

    pub async fn resolve_share_url(&self, url: String) -> Result<Option<InternalUri>, failure::Error> {
        self.app.resolve_share_url(url).await
    }

    pub fn get_player_or_default(&self, player_id: Option<&str>) -> Result<Arc<Player>, failure::Error> {
        let player = match player_id {
            Some(id) => self.app.get_player(id.into()),
            None => self.app.get_default_player(),
        };
        player.ok_or_else(|| format_err!("Missing default player"))
    }

    pub async fn get_thumbnail(&self, provider_item: &ProviderItemType) -> Result<Option<Thumbnail>, failure::Error> {
        self.app.thumbnail(provider_item).await
    }
}

impl Library for ExtensionRuntime {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        self.app.library.query_track(query)
    }

    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        self.app.library.query_tracks(query)
    }

    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        self.app.library.query_album(query)
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        self.app.library.query_albums(query)
    }

    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        self.app.library.query_artist(query)
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        self.app.library.query_artists(query)
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        self.app.library.query_playlist(query)
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        self.app.library.query_playlists(query)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        self.app.library.add_track(track)
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        self.app.library.add_album(album)
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        self.app.library.add_artist(artist)
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        self.app.library.add_playlist(playlist)
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        self.app.library.add_tracks(tracks)
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        self.app.library.add_albums(albums)
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        self.app.library.add_artists(artists)
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        self.app.library.add_playlists(playlists)
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        self.app.library.sync_track(track)
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        self.app.library.sync_album(album)
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        self.app.library.sync_artist(artist)
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        self.app.library.sync_playlist(playlist)
    }

    fn remove_track(&self, track: &Track) -> Result<(), Error> {
        self.app.library.remove_track(track)
    }

    fn remove_album(&self, album: &Album) -> Result<(), Error> {
        self.app.library.remove_album(album)
    }

    fn remove_artist(&self, artist: &Artist) -> Result<(), Error> {
        self.app.library.remove_artist(artist)
    }

    fn remove_playlist(&self, playlist: &Playlist) -> Result<(), Error> {
        self.app.library.remove_playlist(playlist)
    }

    fn search(&self, query: String) -> Result<SearchResults, Error> {
        self.app.library.search(query)
    }

    fn flush(&self) -> Result<(), Error> {
        self.app.library.flush()
    }

    fn observe(&self) -> BoxStream<'static, LibraryEvent> {
        self.app.library.observe()
    }
}
