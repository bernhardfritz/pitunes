use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use dataloader::{cached::Loader, BatchFn};
use diesel::prelude::*;
use futures::executor::block_on;
use oorandom::Rand32;

use crate::{
    db::SqlitePool,
    external_id::ExternalId,
    graphql_schema::RequestContext,
    schema::{albums, artists, genres, playlists, playlists_tracks, prngs, tracks, users},
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
    pub fn id(&self) -> juniper::ID {
        ExternalId::from(self.id).0
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

#[derive(Insertable)]
#[table_name = "albums"]
pub struct NewAlbum {
    pub id: i32,
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
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
    pub fn id(&self) -> juniper::ID {
        ExternalId::from(self.id).0
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

#[derive(Insertable)]
#[table_name = "artists"]
pub struct NewArtist {
    pub id: i32,
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
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
    pub fn id(&self) -> juniper::ID {
        ExternalId::from(self.id).0
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

#[derive(Insertable)]
#[table_name = "genres"]
pub struct NewGenre {
    pub id: i32,
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "genres"]
pub struct GenreInput {
    pub name: String,
}

#[derive(Identifiable, Associations, Queryable)]
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
    pub fn id(&self) -> juniper::ID {
        ExternalId::from(self.id).0
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
            Ok(Some(block_on(context.genre_loader.load(genre_id))))
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
pub struct NewTrack {
    pub id: i32,
    pub name: String,
    pub duration: i32,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub track_number: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct TrackInput {
    pub name: String,
    pub album_id: Option<juniper::ID>,
    pub artist_id: Option<juniper::ID>,
    pub genre_id: Option<juniper::ID>,
    pub track_number: Option<i32>,
}

#[derive(AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "tracks"]
pub struct TrackChangeset {
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
    pub fn id(&self) -> juniper::ID {
        ExternalId::from(self.id).0
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

#[derive(Insertable)]
#[table_name = "playlists"]
pub struct NewPlaylist {
    pub id: i32,
    pub name: String,
}

#[derive(AsChangeset, juniper::GraphQLInputObject)]
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

#[derive(Insertable)]
#[table_name = "playlists_tracks"]
pub struct NewPlaylistTrack {
    pub track_id: i32,
    pub position: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct PlaylistTrackInput {
    #[graphql(name = "id")]
    pub track_id: juniper::ID,
    pub position: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct PlaylistTrackOrderInput {
    pub range_start: i32,
    pub range_length: Option<i32>,
    pub insert_before: i32,
}

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub password: Option<String>,
}

#[derive(AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Prng {
    pub id: i32,
    pub state: i64,
    pub inc: i64,
}

impl From<Rand32> for Prng {
    fn from(rand32: Rand32) -> Self {
        let (state, inc) = rand32.state();
        Prng {
            id: 1,
            state: i64::from_le_bytes(state.to_le_bytes()),
            inc: i64::from_le_bytes(inc.to_le_bytes()),
        }
    }
}

impl From<Prng> for Rand32 {
    fn from(prng: Prng) -> Self {
        let state = u64::from_le_bytes(prng.state.to_le_bytes());
        let inc = u64::from_le_bytes(prng.inc.to_le_bytes());
        let state = (state, inc);
        Rand32::from_state(state)
    }
}
