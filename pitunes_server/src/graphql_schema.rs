use crate::db::SqlitePool;
use crate::models::{
    Album, AlbumChangeset, Artist, ArtistChangeset, Genre, GenreChangeset, NewAlbum, NewArtist,
    NewGenre, Playlist, Track,
};
use crate::schema;
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
        let conn = context.pool.get().unwrap();
        Ok(schema::albums::table.find(id).get_result(&conn)?)
    }

    fn albums(context: &Context) -> Vec<Album> {
        let conn = context.pool.get().unwrap();
        schema::albums::table
            .load::<Album>(&conn)
            .expect("Error loading albums")
    }

    fn artist(context: &Context, id: i32) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get().unwrap();
        Ok(schema::artists::table.find(id).get_result(&conn)?)
    }

    fn artists(context: &Context) -> Vec<Artist> {
        let conn = context.pool.get().unwrap();
        schema::artists::table
            .load::<Artist>(&conn)
            .expect("Error loading artists")
    }

    fn genre(context: &Context, id: i32) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get().unwrap();
        Ok(schema::genres::table.find(id).get_result(&conn)?)
    }

    fn genres(context: &Context) -> Vec<Genre> {
        let conn = context.pool.get().unwrap();
        schema::genres::table
            .load::<Genre>(&conn)
            .expect("Error loading genres")
    }

    fn track(context: &Context, id: i32) -> juniper::FieldResult<Track> {
        let conn = context.pool.get().unwrap();
        Ok(schema::tracks::table.find(id).get_result(&conn)?)
    }

    fn tracks(context: &Context) -> Vec<Track> {
        let conn = context.pool.get().unwrap();
        schema::tracks::table
            .load::<Track>(&conn)
            .expect("Error loading tracks")
    }

    fn playlist(context: &Context, id: i32) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get().unwrap();
        Ok(schema::playlists::table.find(id).get_result(&conn)?)
    }

    fn playlists(context: &Context) -> Vec<Playlist> {
        let conn = context.pool.get().unwrap();
        schema::playlists::table
            .load::<Playlist>(&conn)
            .expect("Error loading playlists")
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_album(context: &Context, new_album: NewAlbum) -> juniper::FieldResult<Album> {
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(schema::albums::table)
                .values(&new_album)
                .execute(&conn)
                .expect("Error saving new album");
            schema::albums::table
                .order(schema::albums::id.desc())
                .first::<Album>(&conn)
        })?)
    }

    fn update_album(
        context: &Context,
        album_changeset: AlbumChangeset,
    ) -> juniper::FieldResult<Album> {
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(schema::albums::table)
                .set(&album_changeset)
                .execute(&conn)?;
            schema::albums::table
                .find(album_changeset.id)
                .get_result(&conn)
        })?)
    }

    fn create_artist(context: &Context, new_artist: NewArtist) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(schema::artists::table)
                .values(&new_artist)
                .execute(&conn)
                .expect("Error saving new artist");
            schema::artists::table
                .order(schema::artists::id.desc())
                .first::<Artist>(&conn)
        })?)
    }

    fn update_artist(
        context: &Context,
        artist_changeset: ArtistChangeset,
    ) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(schema::artists::table)
                .set(&artist_changeset)
                .execute(&conn)?;
            schema::artists::table
                .find(artist_changeset.id)
                .get_result(&conn)
        })?)
    }

    fn create_genre(context: &Context, new_genre: NewGenre) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(schema::genres::table)
                .values(&new_genre)
                .execute(&conn)
                .expect("Error saving new artist");
            schema::genres::table
                .order(schema::genres::id.desc())
                .first::<Genre>(&conn)
        })?)
    }

    fn update_genre(
        context: &Context,
        genre_changeset: GenreChangeset,
    ) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(schema::genres::table)
                .set(&genre_changeset)
                .execute(&conn)?;
            schema::genres::table
                .find(genre_changeset.id)
                .get_result(&conn)
        })?)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
