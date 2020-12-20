table! {
    albums (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
    }
}

table! {
    artists (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
    }
}

table! {
    genres (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
    }
}

table! {
    playlists (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
    }
}

table! {
    playlists_tracks (id) {
        id -> Integer,
        created_at -> Timestamp,
        playlist_id -> Integer,
        track_id -> Integer,
        position -> Integer,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        uuid -> Text,
        created_at -> Timestamp,
        name -> Text,
        duration -> Integer,
        album_id -> Nullable<Integer>,
        artist_id -> Nullable<Integer>,
        genre_id -> Nullable<Integer>,
        track_number -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        created_at -> Timestamp,
        username -> Text,
        password -> Nullable<Text>,
    }
}

joinable!(playlists_tracks -> playlists (playlist_id));
joinable!(playlists_tracks -> tracks (track_id));
joinable!(tracks -> albums (album_id));
joinable!(tracks -> artists (artist_id));
joinable!(tracks -> genres (genre_id));

allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    genres,
    playlists,
    playlists_tracks,
    tracks,
    users,
);
