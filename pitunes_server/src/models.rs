use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use dataloader::{cached::Loader, BatchFn};
use diesel::prelude::*;
use futures::executor::block_on;
use serde::Serialize;

use crate::{
    db::SqlitePool,
    graphql_schema::RequestContext,
    schema::{albums, artists, genres, playlists, playlists_tracks, tracks},
};

#[derive(Clone)]
pub struct AlbumBatcher {
    pub pool: Arc<SqlitePool>,
}

#[async_trait]
impl BatchFn<i32, Album> for AlbumBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Album> {
        let conn = self.pool.get().unwrap();
        let albums = albums::table
            .filter(albums::id.eq_any(keys))
            .load::<Album>(&conn)
            .unwrap();
        {
            let mut album_hashmap = HashMap::new();
            for album in albums {
                album_hashmap.insert(album.id, album);
            }
            album_hashmap
        }
    }
}

pub type AlbumLoader = Loader<i32, Album, AlbumBatcher>;

#[derive(Clone)]
pub struct ArtistBatcher {
    pub pool: Arc<SqlitePool>,
}

#[async_trait]
impl BatchFn<i32, Artist> for ArtistBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Artist> {
        let conn = self.pool.get().unwrap();
        let artists = artists::table
            .filter(artists::id.eq_any(keys))
            .load::<Artist>(&conn)
            .unwrap();
        {
            let mut artist_hashmap = HashMap::new();
            for artist in artists {
                artist_hashmap.insert(artist.id, artist);
            }
            artist_hashmap
        }
    }
}

pub type ArtistLoader = Loader<i32, Artist, ArtistBatcher>;

#[derive(Clone)]
pub struct GenreBatcher {
    pub pool: Arc<SqlitePool>,
}

#[async_trait]
impl BatchFn<i32, Genre> for GenreBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Genre> {
        let conn = self.pool.get().unwrap();
        let genres = genres::table
            .filter(genres::id.eq_any(keys))
            .load::<Genre>(&conn)
            .unwrap();
        {
            let mut genre_hashmap = HashMap::new();
            for genre in genres {
                genre_hashmap.insert(genre.id, genre);
            }
            genre_hashmap
        }
    }
}

pub type GenreLoader = Loader<i32, Genre, GenreBatcher>;

#[derive(Identifiable, Queryable, Clone)]
pub struct Album {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = RequestContext)]
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

    pub fn tracks(&self, context: &RequestContext) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(Track::belonging_to(self).load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "albums"]
pub struct AlbumInput {
    pub name: String,
}

#[derive(Identifiable, Queryable, Clone)]
pub struct Artist {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = RequestContext)]
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

    pub fn albums(&self, context: &RequestContext) -> juniper::FieldResult<Vec<Album>> {
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

    pub fn tracks(&self, context: &RequestContext) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(Track::belonging_to(self).load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "artists"]
pub struct ArtistInput {
    pub name: String,
}

#[derive(Identifiable, Queryable, Clone)]
pub struct Genre {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[juniper::object(Context = RequestContext)]
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

    pub fn tracks(&self, context: &RequestContext) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(Track::belonging_to(self).load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[changeset_options(treat_none_as_null = "true")]
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
    pub duration: i32,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub track_number: Option<i32>,
}

#[juniper::object(Context = RequestContext)]
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

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn album(&self, context: &RequestContext) -> juniper::FieldResult<Option<Album>> {
        if let Some(album_id) = self.album_id {
            Ok(Some(block_on(context.album_loader.load(album_id))))
        } else {
            Ok(None)
        }
    }

    pub fn artist(&self, context: &RequestContext) -> juniper::FieldResult<Option<Artist>> {
        if let Some(artist_id) = self.artist_id {
            Ok(Some(block_on(context.artist_loader.load(artist_id))))
        } else {
            Ok(None)
        }
    }

    pub fn genre(&self, context: &RequestContext) -> juniper::FieldResult<Option<Genre>> {
        let conn = context.pool.get()?;
        if let Some(genre_id) = self.genre_id {
            Ok(Some(genres::table.find(genre_id).get_result(&conn)?))
        } else {
            Ok(None)
        }
    }

    pub fn track_number(&self) -> Option<i32> {
        self.track_number
    }
}

#[derive(Insertable)]
#[table_name = "tracks"]
pub struct TrackInputInternal {
    pub name: String,
    pub duration: i32,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub track_number: Option<i32>,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "tracks"]
pub struct TrackInput {
    pub name: String,
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

#[juniper::object(Context = RequestContext)]
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

    pub fn tracks(&self, context: &RequestContext) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(PlaylistTrack::belonging_to(self)
            .inner_join(tracks::table)
            .select(tracks::all_columns)
            .order_by(playlists_tracks::position.asc())
            .load::<Track>(&conn)?)
    }
}

#[derive(Insertable, AsChangeset, juniper::GraphQLInputObject)]
#[changeset_options(treat_none_as_null = "true")]
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
