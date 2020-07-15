use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface QueuedTrackModel extends TrackModel {
    playing: boolean;
}

export interface RusticApiClient {
    search(query: string, providers: any): Promise<SearchResults>;
    aggregatedSearch(query: string, providers: any): Promise<AggregatedSearchResults>;
    searchLibrary(query: string): Promise<SearchResults>;
    getExtensions(): Promise<ExtensionModel[]>;
    enableExtension(id: string): Promise<void>;
    disableExtension(id: string): Promise<void>;
    openShareUrl(url: string): Promise<OpenResultModel>;
    getProviders(): Promise<ProviderModel[]>;
    getAvailableProviders(): Promise<AvailableProviderModel[]>;
    getAlbums(providers: any): Promise<AlbumModel[]>;
    getAlbum(cursor: string): Promise<AlbumModel>;
    getArtists(): Promise<ArtistModel[]>;
    getArtist(cursor: string): Promise<ArtistModel>;
    getPlaylists(providers: any): Promise<PlaylistModel[]>;
    getPlaylist(cursor: string): Promise<PlaylistModel>;
    getTracks(providers: any): Promise<TrackModel[]>;
    getTrack(cursor: string): Promise<TrackModel>;
    addTrackToLibrary(cursor: string): Promise<void>;
    addAlbumToLibrary(cursor: string): Promise<void>;
    addArtistToLibrary(cursor: string): Promise<void>;
    addPlaylistToLibrary(cursor: string): Promise<void>;
    removeTrackFromLibrary(cursor: string): Promise<void>;
    removeAlbumFromLibrary(cursor: string): Promise<void>;
    removeArtistFromLibrary(cursor: string): Promise<void>;
    removePlaylistFromLibrary(cursor: string): Promise<void>;
    addPlaylist(name: String): Promise<PlaylistModel>;
    removePlaylist(cursor: String): Promise<void>;
    addTrackToPlaylist(playlist: String, track: String): Promise<void>;
    removeTrackFromPlaylist(playlist: String, track: String): Promise<void>;
    getQueue(player_id?: string): Promise<QueuedTrackModel[]>;
    queueTrack(player_id: string | undefined, cursor: string): Promise<void>;
    queueAlbum(player_id: string | undefined, cursor: string): Promise<void>;
    queuePlaylist(player_id: string | undefined, cursor: string): Promise<void>;
    clearQueue(player_id?: string): Promise<void>;
    removeQueueItem(player_id: string | undefined, item: number): Promise<void>;
    selectQueueItem(player_id: string | undefined, item: number): Promise<void>;
    reorderQueueItem(player_id: string | undefined, before: number, after: number): Promise<void>;
    getPlayers(): Promise<PlayerModel[]>;
    getPlayer(player_id?: string): Promise<PlayerModel>;
    playerControlNext(player_id?: string): Promise<void>;
    playerControlPrev(player_id?: string): Promise<void>;
    playerControlPlay(player_id?: string): Promise<void>;
    playerControlPause(player_id?: string): Promise<void>;
    playerSetVolume(player_id: string | undefined, volume: number): Promise<void>;
}"#;
