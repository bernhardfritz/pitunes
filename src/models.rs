use crate::schema::{albums, artists, genres, tracks};
use crate::graphql_schema::Context;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Identifiable, Queryable)]
pub struct Album {
    id: i32,
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
    name: String,
}

#[derive(Identifiable, Queryable)]
pub struct Artist {
    id: i32,
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

    pub fn tracks(&self, context: &Context) -> Vec<Track> {
        let conn = context.pool.get().unwrap();
        Track::belonging_to(self)
            .load::<Track>(&conn)
            .expect("Error loading tracks")
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
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "artists"]
pub struct NewArtist {
    name: String,
}

#[derive(Identifiable, Queryable, juniper::GraphQLObject)]
pub struct Genre {
    id: i32,
    name: String,
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "genres"]
pub struct NewGenre {
    name: String,
}

#[derive(Identifiable, Associations, Queryable, juniper::GraphQLObject)]
#[belongs_to(Album)]
#[belongs_to(Artist)]
#[belongs_to(Genre)]
pub struct Track {
    id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    name: String,
    duration: i32,
    album_id: Option<i32>,
    artist_id: Option<i32>,
    genre_id: Option<i32>,
    track_number: Option<i32>,
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "tracks"]
pub struct NewTrack {
    name: String,
    duration: i32,
    album_id: Option<i32>,
    artist_id: Option<i32>,
    genre_id: Option<i32>,
    track_number: Option<i32>,
}