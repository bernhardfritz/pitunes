use crate::db::SqlitePool;
use crate::models::{
    Album, AlbumChangeset, Artist, ArtistChangeset, Genre, GenreChangeset, NewAlbum, NewArtist,
    NewGenre, NewPlaylist, NewPlaylistTrack, Playlist, PlaylistChangeset, Track, TrackChangeset,
};
use crate::schema::{albums, artists, genres, playlists, playlists_tracks, tracks};
use diesel::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub pool: SqlitePool,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = Context,
)]
impl Query {
    fn album(context: &Context, id: i32) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        Ok(albums::table.find(id).get_result(&conn)?)
    }

    fn albums(context: &Context) -> juniper::FieldResult<Vec<Album>> {
        let conn = context.pool.get()?;
        Ok(albums::table.load::<Album>(&conn)?)
    }

    fn artist(context: &Context, id: i32) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        Ok(artists::table.find(id).get_result(&conn)?)
    }

    fn artists(context: &Context) -> juniper::FieldResult<Vec<Artist>> {
        let conn = context.pool.get()?;
        Ok(artists::table.load::<Artist>(&conn)?)
    }

    fn genre(context: &Context, id: i32) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        Ok(genres::table.find(id).get_result(&conn)?)
    }

    fn genres(context: &Context) -> juniper::FieldResult<Vec<Genre>> {
        let conn = context.pool.get()?;
        Ok(genres::table.load::<Genre>(&conn)?)
    }

    fn track(context: &Context, id: i32) -> juniper::FieldResult<Track> {
        let conn = context.pool.get()?;
        Ok(tracks::table.find(id).get_result(&conn)?)
    }

    fn tracks(context: &Context) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(tracks::table.load::<Track>(&conn)?)
    }

    fn playlist(context: &Context, id: i32) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(playlists::table.find(id).get_result(&conn)?)
    }

    fn playlists(context: &Context) -> juniper::FieldResult<Vec<Playlist>> {
        let conn = context.pool.get()?;
        Ok(playlists::table.load::<Playlist>(&conn)?)
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_album(context: &Context, new_album: NewAlbum) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(albums::table)
                .values(&new_album)
                .execute(&conn)?;
            albums::table.order(albums::id.desc()).first::<Album>(&conn)
        })?)
    }

    fn update_album(
        context: &Context,
        album_changeset: AlbumChangeset,
    ) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(albums::table.find(album_changeset.id))
                .set(&album_changeset)
                .execute(&conn)?;
            albums::table.find(album_changeset.id).get_result(&conn)
        })?)
    }

    fn delete_album(context: &Context, album_id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(albums::table.find(album_id)).execute(&conn)? == 1)
    }

    fn create_artist(context: &Context, new_artist: NewArtist) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(artists::table)
                .values(&new_artist)
                .execute(&conn)?;
            artists::table
                .order(artists::id.desc())
                .first::<Artist>(&conn)
        })?)
    }

    fn update_artist(
        context: &Context,
        artist_changeset: ArtistChangeset,
    ) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(artists::table.find(artist_changeset.id))
                .set(&artist_changeset)
                .execute(&conn)?;
            artists::table.find(artist_changeset.id).get_result(&conn)
        })?)
    }

    fn delete_artist(context: &Context, artist_id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(artists::table.find(artist_id)).execute(&conn)? == 1)
    }

    fn create_genre(context: &Context, new_genre: NewGenre) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(genres::table)
                .values(&new_genre)
                .execute(&conn)?;
            genres::table.order(genres::id.desc()).first::<Genre>(&conn)
        })?)
    }

    fn update_genre(
        context: &Context,
        genre_changeset: GenreChangeset,
    ) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(genres::table.find(genre_changeset.id))
                .set(&genre_changeset)
                .execute(&conn)?;
            genres::table.find(genre_changeset.id).get_result(&conn)
        })?)
    }

    fn delete_genre(context: &Context, genre_id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(genres::table.find(genre_id)).execute(&conn)? == 1)
    }

    fn update_track(
        context: &Context,
        track_changeset: TrackChangeset,
    ) -> juniper::FieldResult<Track> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(tracks::table.find(track_changeset.id))
                .set(&track_changeset)
                .execute(&conn)?;
            tracks::table.find(track_changeset.id).get_result(&conn)
        })?)
    }

    fn create_playlist(
        context: &Context,
        new_playlist: NewPlaylist,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(playlists::table)
                .values(&new_playlist)
                .execute(&conn)?;
            playlists::table
                .order(playlists::id.desc())
                .first::<Playlist>(&conn)
        })?)
    }

    fn update_playlist(
        context: &Context,
        playlist_changeset: PlaylistChangeset,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(playlists::table.find(playlist_changeset.id))
                .set(&playlist_changeset)
                .execute(&conn)?;
            playlists::table
                .find(playlist_changeset.id)
                .get_result(&conn)
        })?)
    }

    fn delete_playlist(context: &Context, playlist_id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(playlists::table.find(playlist_id)).execute(&conn)? == 1)
    }

    fn create_playlist_track(
        context: &Context,
        new_playlist_track: NewPlaylistTrack,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(playlists_tracks::table)
                .values(&new_playlist_track)
                .execute(&conn)?;
            playlists::table
                .find(new_playlist_track.playlist_id)
                .get_result(&conn)
        })?)
    }

    // TODO: update_playlist_track should allow to change playlist track order

    // TODO: delete_playlist_track
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
