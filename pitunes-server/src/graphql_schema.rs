use std::convert::TryFrom;
use std::sync::Arc;

use diesel::prelude::*;

use crate::{
    db::SqlitePool,
    models::{
        Album, AlbumBatcher, AlbumInput, AlbumLoader, Artist, ArtistBatcher, ArtistInput,
        ArtistLoader, Genre, GenreBatcher, GenreInput, GenreLoader, Playlist, PlaylistInput,
        PlaylistTrack, PlaylistTrackInput, PlaylistTrackOrderInput, Track, TrackInput,
    },
    schema::{albums, artists, genres, playlists, playlists_tracks, tracks},
};

#[derive(Clone)]
pub struct RequestContext {
    pub pool: Arc<SqlitePool>,
    pub album_loader: AlbumLoader,
    pub artist_loader: ArtistLoader,
    pub genre_loader: GenreLoader,
}

impl RequestContext {
    pub fn new(pool: SqlitePool) -> RequestContext {
        let pool = Arc::new(pool);
        let album_loader = AlbumLoader::new(AlbumBatcher { pool: pool.clone() });
        let artist_loader = ArtistLoader::new(ArtistBatcher { pool: pool.clone() });
        let genre_loader = GenreLoader::new(GenreBatcher { pool: pool.clone() });
        RequestContext {
            pool,
            album_loader,
            artist_loader,
            genre_loader,
        }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for RequestContext {}

pub struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = RequestContext,
)]
impl Query {
    fn album(context: &RequestContext, id: i32) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        Ok(albums::table.find(id).get_result(&conn)?)
    }

    fn albums(context: &RequestContext) -> juniper::FieldResult<Vec<Album>> {
        let conn = context.pool.get()?;
        Ok(albums::table.load::<Album>(&conn)?)
    }

    fn artist(context: &RequestContext, id: i32) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        Ok(artists::table.find(id).get_result(&conn)?)
    }

    fn artists(context: &RequestContext) -> juniper::FieldResult<Vec<Artist>> {
        let conn = context.pool.get()?;
        Ok(artists::table.load::<Artist>(&conn)?)
    }

    fn genre(context: &RequestContext, id: i32) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        Ok(genres::table.find(id).get_result(&conn)?)
    }

    fn genres(context: &RequestContext) -> juniper::FieldResult<Vec<Genre>> {
        let conn = context.pool.get()?;
        Ok(genres::table.load::<Genre>(&conn)?)
    }

    fn track(context: &RequestContext, id: i32) -> juniper::FieldResult<Track> {
        let conn = context.pool.get()?;
        Ok(tracks::table.find(id).get_result(&conn)?)
    }

    fn tracks(context: &RequestContext) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(tracks::table.load::<Track>(&conn)?)
    }

    fn playlist(context: &RequestContext, id: i32) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(playlists::table.find(id).get_result(&conn)?)
    }

    fn playlists(context: &RequestContext) -> juniper::FieldResult<Vec<Playlist>> {
        let conn = context.pool.get()?;
        Ok(playlists::table.load::<Playlist>(&conn)?)
    }
}

pub struct Mutation;

#[juniper::object(Context = RequestContext)]
impl Mutation {
    fn create_album(
        context: &RequestContext,
        album_input: AlbumInput,
    ) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(albums::table)
                .values(&album_input)
                .execute(&conn)?;
            albums::table.order(albums::id.desc()).first::<Album>(&conn)
        })?)
    }

    fn update_album(
        context: &RequestContext,
        id: i32,
        album_input: AlbumInput,
    ) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(albums::table.find(id))
                .set(&album_input)
                .execute(&conn)?;
            albums::table.find(id).get_result(&conn)
        })?)
    }

    fn delete_album(context: &RequestContext, id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(albums::table.find(id)).execute(&conn)? == 1)
    }

    fn create_artist(
        context: &RequestContext,
        artist_input: ArtistInput,
    ) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(artists::table)
                .values(&artist_input)
                .execute(&conn)?;
            artists::table
                .order(artists::id.desc())
                .first::<Artist>(&conn)
        })?)
    }

    fn update_artist(
        context: &RequestContext,
        id: i32,
        artist_input: ArtistInput,
    ) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(artists::table.find(id))
                .set(&artist_input)
                .execute(&conn)?;
            artists::table.find(id).get_result(&conn)
        })?)
    }

    fn delete_artist(context: &RequestContext, id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(artists::table.find(id)).execute(&conn)? == 1)
    }

    fn create_genre(
        context: &RequestContext,
        genre_input: GenreInput,
    ) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(genres::table)
                .values(&genre_input)
                .execute(&conn)?;
            genres::table.order(genres::id.desc()).first::<Genre>(&conn)
        })?)
    }

    fn update_genre(
        context: &RequestContext,
        id: i32,
        genre_input: GenreInput,
    ) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(genres::table.find(id))
                .set(&genre_input)
                .execute(&conn)?;
            genres::table.find(id).get_result(&conn)
        })?)
    }

    fn delete_genre(context: &RequestContext, id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(genres::table.find(id)).execute(&conn)? == 1)
    }

    fn update_track(
        context: &RequestContext,
        id: i32,
        track_input: TrackInput,
    ) -> juniper::FieldResult<Track> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(tracks::table.find(id))
                .set(&track_input)
                .execute(&conn)?;
            tracks::table.find(id).get_result(&conn)
        })?)
    }

    fn delete_track(context: &RequestContext, id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        let deleted = diesel::delete(tracks::table.find(id)).execute(&conn)? == 1;
        if deleted {
            let filepath = format!("./tracks/{}.mp3", id);
            std::fs::remove_file(filepath)?;
        }
        Ok(deleted)
    }

    fn create_playlist(
        context: &RequestContext,
        playlist_input: PlaylistInput,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(playlists::table)
                .values(&playlist_input)
                .execute(&conn)?;
            playlists::table
                .order(playlists::id.desc())
                .first::<Playlist>(&conn)
        })?)
    }

    fn update_playlist(
        context: &RequestContext,
        id: i32,
        playlist_input: PlaylistInput,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(playlists::table.find(id))
                .set(&playlist_input)
                .execute(&conn)?;
            playlists::table.find(id).get_result(&conn)
        })?)
    }

    fn delete_playlist(context: &RequestContext, id: i32) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        Ok(diesel::delete(playlists::table.find(id)).execute(&conn)? == 1)
    }

    fn create_playlist_track(
        context: &RequestContext,
        id: i32, // playlist_id
        playlist_track_input: PlaylistTrackInput,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            let count: i64 = playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(id))
                .count()
                .get_result(&conn)?;
            let count: i32 =
                i32::try_from(count).map_err(|_| diesel::result::Error::RollbackTransaction)?;
            let playlist_track_input = PlaylistTrackInput {
                position: playlist_track_input.position.or_else(|| Some(count)),
                ..playlist_track_input
            };
            let position = playlist_track_input.position.unwrap();
            if position < 0 || count < position {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            if position != count {
                diesel::update(
                    playlists_tracks::table
                        .filter(playlists_tracks::playlist_id.eq(id))
                        .filter(playlists_tracks::position.ge(position)),
                )
                .set(playlists_tracks::position.eq(playlists_tracks::position + 1))
                .execute(&conn)?;
            }
            diesel::insert_into(playlists_tracks::table)
                .values((playlists_tracks::playlist_id.eq(id), playlist_track_input))
                .execute(&conn)?;
            playlists::table.find(id).get_result(&conn)
        })?)
    }

    fn update_playlist_track(
        context: &RequestContext,
        id: i32, // playlist_id
        playlist_track_order_input: PlaylistTrackOrderInput,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        let playlist_track_order_input = PlaylistTrackOrderInput {
            range_length: playlist_track_order_input.range_length.or_else(|| Some(1)),
            ..playlist_track_order_input
        };
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            let range_start = usize::try_from(playlist_track_order_input.range_start)
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;
            let range_length = usize::try_from(playlist_track_order_input.range_length.unwrap())
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;
            if range_length < 1 {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            let insert_before = usize::try_from(playlist_track_order_input.insert_before)
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;
            if range_start < insert_before && insert_before < range_start + range_length {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            let mut playlist_tracks = playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(id))
                .order(playlists_tracks::position.asc())
                .load::<PlaylistTrack>(&conn)?;
            let len = playlist_tracks.len();
            if range_start > len - 1 || range_start + range_length > len || insert_before > len {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            if insert_before < range_start {
                let slice = &mut playlist_tracks[insert_before..range_start + range_length];
                slice.rotate_left(range_start - insert_before);
                for (i, playlist_track) in slice.iter().enumerate() {
                    let delta = i32::try_from(i)
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?
                        - (playlist_track.position
                            - i32::try_from(insert_before)
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?);
                    diesel::update(playlists_tracks::table.find(playlist_track.id))
                        .set(playlists_tracks::position.eq(playlists_tracks::position + delta))
                        .execute(&conn)?;
                }
            } else if insert_before > range_start + range_length {
                let slice = &mut playlist_tracks[range_start..insert_before];
                slice.rotate_right(insert_before - (range_start + range_length));
                for (i, playlist_track) in slice.iter().enumerate() {
                    let delta = i32::try_from(i)
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?
                        - (playlist_track.position
                            - i32::try_from(range_start)
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?);
                    diesel::update(playlists_tracks::table.find(playlist_track.id))
                        .set(playlists_tracks::position.eq(playlists_tracks::position + delta))
                        .execute(&conn)?;
                }
            }
            playlists::table.find(id).get_result(&conn)
        })?)
    }

    fn delete_playlist_track(
        context: &RequestContext,
        id: i32, // playlist_id
        playlist_track_input: PlaylistTrackInput,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            let deleted = if let Some(position) = playlist_track_input.position {
                diesel::delete(
                    playlists_tracks::table
                        .filter(playlists_tracks::playlist_id.eq(id))
                        .filter(playlists_tracks::track_id.eq(playlist_track_input.track_id))
                        .filter(playlists_tracks::position.eq(position)),
                )
                .execute(&conn)?
                    == 1
            } else {
                diesel::delete(
                    playlists_tracks::table
                        .filter(playlists_tracks::playlist_id.eq(id))
                        .filter(playlists_tracks::track_id.eq(playlist_track_input.track_id)),
                )
                .execute(&conn)?
                    >= 1
            };
            if !deleted {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            let playlist_tracks = playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(id))
                .order(playlists_tracks::position.asc())
                .load::<PlaylistTrack>(&conn)?;
            for (i, playlist_track) in playlist_tracks.iter().enumerate() {
                let i = i32::try_from(i).map_err(|_| diesel::result::Error::RollbackTransaction)?;
                if playlist_track.position != i {
                    diesel::update(playlists_tracks::table.find(playlist_track.id))
                        .set(playlists_tracks::position.eq(i))
                        .execute(&conn)?;
                }
            }
            playlists::table.find(id).get_result(&conn)
        })?)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
