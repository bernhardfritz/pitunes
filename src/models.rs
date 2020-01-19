use crate::graphql_schema::Context;
use crate::schema::{albums, artists, genres, tracks};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Identifiable, Queryable)]
pub struct Album {
    pub id: i32,
    name: String,
}

#[juniper::object(Context = Context)]
impl Album {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn tracks(&self, context: &Context) -> Vec<Track> {
        let conn = context.pool.get().unwrap();
        Track::belonging_to(self)
            .load::<Track>(&conn)
            .expect("Error loading tracks")
    }
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "albums"]
pub struct NewAlbum {
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "albums"]
pub struct AlbumChangeset {
    pub id: i32,
    name: Option<String>,
}

#[derive(Identifiable, Queryable)]
pub struct Artist {
    pub id: i32,
    name: String,
}

#[juniper::object(Context = Context)]
impl Artist {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn albums(&self, context: &Context) -> Vec<Album> {
        use self::albums::dsl::*;
        let conn = context.pool.get().unwrap();
        let album_ids: Vec<i32> = Track::belonging_to(self)
            .select(tracks::album_id)
            .distinct()
            .filter(tracks::album_id.is_not_null())
            .load::<Option<i32>>(&conn)
            .unwrap()
            .into_iter()
            .flatten()
            .collect();
        albums
            .filter(id.eq_any(album_ids))
            .load::<Album>(&conn)
            .unwrap()
    }

    pub fn tracks(&self, context: &Context) -> Vec<Track> {
        let conn = context.pool.get().unwrap();
        Track::belonging_to(self)
            .load::<Track>(&conn)
            .expect("Error loading tracks")
    }
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "artists"]
pub struct NewArtist {
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "artists"]
pub struct ArtistChangeset {
    pub id: i32,
    name: Option<String>,
}

#[derive(Identifiable, Queryable)]
pub struct Genre {
    pub id: i32,
    name: String,
}

#[juniper::object(Context = Context)]
impl Genre {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn tracks(&self, context: &Context) -> Vec<Track> {
        let conn = context.pool.get().unwrap();
        Track::belonging_to(self)
            .load::<Track>(&conn)
            .expect("Error loading tracks")
    }
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "genres"]
pub struct NewGenre {
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "genres"]
pub struct GenreChangeset {
    pub id: i32,
    name: Option<String>,
}

#[derive(Identifiable, Associations, Queryable, Serialize)]
#[belongs_to(Album)]
#[belongs_to(Artist)]
#[belongs_to(Genre)]
pub struct Track {
    id: i32,
    created_at: NaiveDateTime,
    name: String,
    duration: Option<i32>,
    album_id: Option<i32>,
    artist_id: Option<i32>,
    genre_id: Option<i32>,
    track_number: Option<i32>,
}

#[juniper::object(Context = Context)]
impl Track {
    fn id(&self) -> i32 {
        self.id
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn name(&self) -> &str {
        &self.name[..]
    }

    fn duration(&self) -> Option<i32> {
        self.duration
    }

    fn album_id(&self) -> Option<i32> {
        self.album_id
    }

    fn artist_id(&self) -> Option<i32> {
        self.artist_id
    }

    fn genre_id(&self) -> Option<i32> {
        self.genre_id
    }

    fn track_number(&self) -> Option<i32> {
        self.track_number
    }
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "tracks"]
pub struct NewTrack {
    pub name: String,
    pub duration: Option<i32>,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub track_number: Option<i32>,
}
