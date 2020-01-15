use diesel::prelude::*;
use crate::db::SqlitePool;
use crate::schema::{albums, artists, genres, tracks};
use crate::models::{Album, Artist, Genre, NewArtist, Track};

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
        albums
            .limit(100)
            .load::<Album>(&conn)
            .expect("Error loading albums")
    }

    fn artists(context: &Context) -> Vec<Artist> {
        use self::artists::dsl::*;
        let conn = context.pool.get().unwrap();
        artists
            .limit(100)
            .load::<Artist>(&conn)
            .expect("Error loading artists")
    }

    fn genres(context: &Context) -> Vec<Genre> {
        use self::genres::dsl::*;
        let conn = context.pool.get().unwrap();
        genres
            .limit(100)
            .load::<Genre>(&conn)
            .expect("Error loading genres")
    }

    fn tracks(context: &Context) -> Vec<Track> {
        use self::tracks::dsl::*;
        let conn = context.pool.get().unwrap();
        tracks
            .limit(100)
            .load::<Track>(&conn)
            .expect("Error loading tracks")
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_artist(context: &Context, new_artist: NewArtist) -> Artist {
        use diesel::result::Error;
        use self::artists::dsl::*;
        let conn = context.pool.get().unwrap();
        conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(artists)
                .values(&new_artist)
                .execute(&conn)
                .expect("Error saving new artist");
    
            artists
                .order(id.desc())
                .first::<Artist>(&conn)
        }).unwrap()
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}