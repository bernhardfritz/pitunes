table! {
    albums (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    genres (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        duration -> Integer,
        album_id -> Nullable<Integer>,
        artist_id -> Nullable<Integer>,
        genre_id -> Nullable<Integer>,
        track_number -> Nullable<Integer>,
    }
}

joinable!(tracks -> albums (album_id));
joinable!(tracks -> artists (artist_id));
joinable!(tracks -> genres (genre_id));

allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    genres,
    tracks,
);
