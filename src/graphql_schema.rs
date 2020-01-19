use crate::db::SqlitePool;
use crate::models::{
    Album, AlbumChangeset, Artist, ArtistChangeset, Genre, GenreChangeset, NewAlbum, NewArtist,
    NewGenre, Track,
};
use crate::schema::{albums, artists, genres, tracks};
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
    fn albums(context: &Context) -> Vec<Album> {
        use self::albums::dsl::*;
        let conn = context.pool.get().unwrap();
        albums.load::<Album>(&conn).expect("Error loading albums")
    }

    fn artists(context: &Context) -> Vec<Artist> {
        use self::artists::dsl::*;
        let conn = context.pool.get().unwrap();
        artists
            .load::<Artist>(&conn)
            .expect("Error loading artists")
    }

    fn genres(context: &Context) -> Vec<Genre> {
        use self::genres::dsl::*;
        let conn = context.pool.get().unwrap();
        genres.load::<Genre>(&conn).expect("Error loading genres")
    }

    fn tracks(context: &Context) -> Vec<Track> {
        use self::tracks::dsl::*;
        let conn = context.pool.get().unwrap();
        tracks.load::<Track>(&conn).expect("Error loading tracks")
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_album(context: &Context, new_album: NewAlbum) -> Album {
        use self::albums::dsl::*;
        use diesel::result::Error;
        let conn = context.pool.get().unwrap();
        conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(albums)
                .values(&new_album)
                .execute(&conn)
                .expect("Error saving new album");
            albums.order(id.desc()).first::<Album>(&conn)
        })
        .unwrap()
    }

    fn update_album(
        context: &Context,
        album_changeset: AlbumChangeset,
    ) -> juniper::FieldResult<Album> {
        use self::albums::dsl::*;
        use diesel::result::Error;
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, Error, _>(|| {
            diesel::update(albums)
                .set(&album_changeset)
                .execute(&conn)?;
            albums.find(album_changeset.id).get_result(&conn)
        })?)
    }

    fn create_artist(context: &Context, new_artist: NewArtist) -> Artist {
        use self::artists::dsl::*;
        use diesel::result::Error;
        let conn = context.pool.get().unwrap();
        conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(artists)
                .values(&new_artist)
                .execute(&conn)
                .expect("Error saving new artist");
            artists.order(id.desc()).first::<Artist>(&conn)
        })
        .unwrap()
    }

    fn update_artist(
        context: &Context,
        artist_changeset: ArtistChangeset,
    ) -> juniper::FieldResult<Artist> {
        use self::artists::dsl::*;
        use diesel::result::Error;
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, Error, _>(|| {
            diesel::update(artists)
                .set(&artist_changeset)
                .execute(&conn)?;
            artists.find(artist_changeset.id).get_result(&conn)
        })?)
    }

    fn create_genre(context: &Context, new_genre: NewGenre) -> Genre {
        use self::genres::dsl::*;
        use diesel::result::Error;
        let conn = context.pool.get().unwrap();
        conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(genres)
                .values(&new_genre)
                .execute(&conn)
                .expect("Error saving new artist");
            genres.order(id.desc()).first::<Genre>(&conn)
        })
        .unwrap()
    }

    fn update_genre(
        context: &Context,
        genre_changeset: GenreChangeset,
    ) -> juniper::FieldResult<Genre> {
        use self::genres::dsl::*;
        use diesel::result::Error;
        let conn = context.pool.get().unwrap();
        Ok(conn.transaction::<_, Error, _>(|| {
            diesel::update(genres)
                .set(&genre_changeset)
                .execute(&conn)?;
            genres.find(genre_changeset.id).get_result(&conn)
        })?)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
