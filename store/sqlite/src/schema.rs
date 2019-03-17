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
    artists (id) {
        id -> Integer,
        name -> Text,
        image_url -> Nullable<Text>,
        uri -> Text,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        title -> Text,
        artist_id -> Nullable<Integer>,
        album_id -> Nullable<Integer>,
        provider -> Integer,
        uri -> Text,
        image_url -> Nullable<Text>,
        duration -> Nullable<Integer>,
    }
}

joinable!(albums -> artists (artist_id));
joinable!(tracks -> albums (album_id));
joinable!(tracks -> artists (artist_id));

allow_tables_to_appear_in_same_query!(albums, artists, tracks,);
