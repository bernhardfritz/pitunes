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
    prngs (id) {
        id -> Integer,
        state -> BigInt,
        inc -> BigInt,
    }
}

table! {
    tracks (id) {
        id -> Integer,
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
    users (username) {
        username -> Text,
        password -> Binary,
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
    prngs,
    tracks,
    users,
);
