table! {
    albums (id) {
        id -> Integer,
        title -> Text,
        artist_id -> Nullable<Integer>,
        image_url -> Nullable<Text>,
        uri -> Text,
        provider -> Integer,
    }
}

table! {
    albums_meta (album_id, key) {
        album_id -> Integer,
        key -> Text,
        bool_variant -> Nullable<Integer>,
        float_variant -> Nullable<Float>,
        string_variant -> Nullable<Text>,
        int_variant -> Nullable<Integer>,
    }
}

table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        image_url -> Nullable<Text>,
        uri -> Text,
        provider -> Integer,
    }
}

table! {
    artists_meta (artist_id, key) {
        artist_id -> Integer,
        key -> Text,
        bool_variant -> Nullable<Integer>,
        float_variant -> Nullable<Float>,
        string_variant -> Nullable<Text>,
        int_variant -> Nullable<Integer>,
    }
}

table! {
    playlist_tracks (playlist_id, track_id) {
        playlist_id -> Integer,
        track_id -> Integer,
    }
}

table! {
    playlists (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        title -> Text,
        artist_id -> Nullable<Integer>,
        album_id -> Nullable<Integer>,
        uri -> Text,
        image_url -> Nullable<Text>,
        duration -> Nullable<Integer>,
        provider -> Integer,
    }
}

table! {
    tracks_meta (track_id, key) {
        track_id -> Integer,
        key -> Text,
        bool_variant -> Nullable<Integer>,
        float_variant -> Nullable<Float>,
        string_variant -> Nullable<Text>,
        int_variant -> Nullable<Integer>,
    }
}

joinable!(albums -> artists (artist_id));
joinable!(albums_meta -> albums (album_id));
joinable!(artists_meta -> artists (artist_id));
joinable!(playlist_tracks -> playlists (playlist_id));
joinable!(playlist_tracks -> tracks (track_id));
joinable!(tracks -> albums (album_id));
joinable!(tracks -> artists (artist_id));
joinable!(tracks_meta -> tracks (track_id));

allow_tables_to_appear_in_same_query!(
    albums,
    albums_meta,
    artists,
    artists_meta,
    playlist_tracks,
    playlists,
    tracks,
    tracks_meta,
);
