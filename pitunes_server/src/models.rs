use crate::graphql_schema::Context;
use crate::schema::{albums, artists, genres, playlists, playlists_tracks, tracks};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Identifiable, Queryable)]
pub struct Album {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = Context)]
impl Album {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn tracks(&self, context: &Context) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(Track::belonging_to(self).load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "albums"]
pub struct AlbumInput {
    pub name: String,
}

#[derive(Identifiable, Queryable)]
pub struct Artist {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = Context)]
impl Artist {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn albums(&self, context: &Context) -> juniper::FieldResult<Vec<Album>> {
        let conn = context.pool.get()?;
        let album_ids: Vec<i32> = Track::belonging_to(self)
            .select(tracks::album_id)
            .distinct()
            .filter(tracks::album_id.is_not_null())
            .load::<Option<i32>>(&conn)?
            .into_iter()
            .flatten()
            .collect();
        Ok(albums::table
            .filter(albums::id.eq_any(album_ids))
            .load::<Album>(&conn)?)
    }

    pub fn tracks(&self, context: &Context) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(Track::belonging_to(self).load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "artists"]
pub struct ArtistInput {
    pub name: String,
}

#[derive(Identifiable, Queryable)]
pub struct Genre {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = Context)]
impl Genre {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn tracks(&self, context: &Context) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(Track::belonging_to(self).load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "genres"]
pub struct GenreInput {
    pub name: String,
}

#[derive(Identifiable, Associations, Queryable, Serialize)]
#[belongs_to(Album)]
#[belongs_to(Artist)]
#[belongs_to(Genre)]
pub struct Track {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub duration: Option<i32>,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub track_number: Option<i32>,
}

#[juniper::object(Context = Context)]
impl Track {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn duration(&self) -> Option<i32> {
        self.duration
    }

    pub fn album_id(&self) -> Option<i32> {
        self.album_id
    }

    pub fn artist_id(&self) -> Option<i32> {
        self.artist_id
    }

    pub fn genre_id(&self) -> Option<i32> {
        self.genre_id
    }

    pub fn track_number(&self) -> Option<i32> {
        self.track_number
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "tracks"]
pub struct TrackInput {
    pub name: String,
    pub duration: Option<i32>,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub track_number: Option<i32>,
}

#[derive(Identifiable, Queryable)]
pub struct Playlist {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = Context)]
impl Playlist {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn tracks(&self, context: &Context) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(PlaylistTrack::belonging_to(self)
            .inner_join(tracks::table)
            .select(tracks::all_columns)
            .order_by(playlists_tracks::position.asc())
            .load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[table_name = "playlists"]
pub struct PlaylistInput {
    pub name: String,
}

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(Playlist)]
#[belongs_to(Track)]
#[table_name = "playlists_tracks"]
pub struct PlaylistTrack {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub playlist_id: i32,
    pub track_id: i32,
    pub position: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name = "playlists_tracks"]
pub struct PlaylistTrackInput {
    pub track_id: i32,
    pub position: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct PlaylistTrackOrderInput {
    pub range_start: i32,
    pub range_length: Option<i32>,
    pub insert_before: i32,
}
