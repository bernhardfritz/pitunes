use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

use diesel::prelude::*;

use crate::{
    db::SqlitePool,
    external_id::ExternalId,
    models::{
        Album, AlbumBatcher, AlbumInput, AlbumLoader, Artist, ArtistBatcher, ArtistInput,
        ArtistLoader, Genre, GenreBatcher, GenreInput, GenreLoader, NewAlbum, NewArtist, NewGenre,
        NewPlaylist, NewPlaylistTrack, Playlist, PlaylistInput, PlaylistTrack, PlaylistTrackInput,
        PlaylistTrackOrderInput, Track, TrackChangeset, TrackInput, UserChangeset, UserInput,
    },
    prng,
    schema::{albums, artists, genres, playlists, playlists_tracks, tracks, users},
};

#[derive(Clone)]
pub struct RequestContext {
    pub pool: Arc<SqlitePool>,
    pub tracks_dir: PathBuf,
    pub album_loader: AlbumLoader,
    pub artist_loader: ArtistLoader,
    pub genre_loader: GenreLoader,
}

impl RequestContext {
    pub fn new(pool: SqlitePool, tracks_dir: PathBuf) -> RequestContext {
        let pool = Arc::new(pool);
        let album_loader = AlbumLoader::new(AlbumBatcher { pool: pool.clone() });
        let artist_loader = ArtistLoader::new(ArtistBatcher { pool: pool.clone() });
        let genre_loader = GenreLoader::new(GenreBatcher { pool: pool.clone() });
        RequestContext {
            pool,
            tracks_dir,
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
    fn album(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<Album> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(albums::table.find(id).get_result(&conn)?)
    }

    fn albums(context: &RequestContext) -> juniper::FieldResult<Vec<Album>> {
        let conn = context.pool.get()?;
        Ok(albums::table.load::<Album>(&conn)?)
    }

    fn artist(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<Artist> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(artists::table.find(id).get_result(&conn)?)
    }

    fn artists(context: &RequestContext) -> juniper::FieldResult<Vec<Artist>> {
        let conn = context.pool.get()?;
        Ok(artists::table.load::<Artist>(&conn)?)
    }

    fn genre(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<Genre> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(genres::table.find(id).get_result(&conn)?)
    }

    fn genres(context: &RequestContext) -> juniper::FieldResult<Vec<Genre>> {
        let conn = context.pool.get()?;
        Ok(genres::table.load::<Genre>(&conn)?)
    }

    fn track(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<Track> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(tracks::table.find(id).get_result(&conn)?)
    }

    fn tracks(context: &RequestContext) -> juniper::FieldResult<Vec<Track>> {
        let conn = context.pool.get()?;
        Ok(tracks::table.load::<Track>(&conn)?)
    }

    fn playlist(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<Playlist> {
        let id: i32 = ExternalId(id).try_into()?;
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
    fn create_album(context: &RequestContext, input: AlbumInput) -> juniper::FieldResult<Album> {
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let new_album = NewAlbum {
                id: prng::rand_i32(&conn)?,
                name: input.name,
            };
            diesel::insert_into(albums::table)
                .values(&new_album)
                .execute(&conn)?;
            Ok(albums::table.find(new_album.id).get_result(&conn)?)
        })
    }

    fn update_album(
        context: &RequestContext,
        id: juniper::ID,
        input: AlbumInput,
    ) -> juniper::FieldResult<Album> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            diesel::update(albums::table.find(id))
                .set(&input)
                .execute(&conn)?;
            Ok(albums::table.find(id).get_result(&conn)?)
        })
    }

    fn delete_album(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<bool> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(diesel::delete(albums::table.find(id)).execute(&conn)? == 1)
    }

    fn create_artist(context: &RequestContext, input: ArtistInput) -> juniper::FieldResult<Artist> {
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let new_artist = NewArtist {
                id: prng::rand_i32(&conn)?,
                name: input.name,
            };
            diesel::insert_into(artists::table)
                .values(&new_artist)
                .execute(&conn)?;
            Ok(artists::table.find(new_artist.id).get_result(&conn)?)
        })
    }

    fn update_artist(
        context: &RequestContext,
        id: juniper::ID,
        input: ArtistInput,
    ) -> juniper::FieldResult<Artist> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            diesel::update(artists::table.find(id))
                .set(&input)
                .execute(&conn)?;
            Ok(artists::table.find(id).get_result(&conn)?)
        })
    }

    fn delete_artist(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<bool> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(diesel::delete(artists::table.find(id)).execute(&conn)? == 1)
    }

    fn create_genre(context: &RequestContext, input: GenreInput) -> juniper::FieldResult<Genre> {
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let new_genre = NewGenre {
                id: prng::rand_i32(&conn)?,
                name: input.name,
            };
            diesel::insert_into(genres::table)
                .values(&new_genre)
                .execute(&conn)?;
            Ok(genres::table.find(new_genre.id).get_result(&conn)?)
        })
    }

    fn update_genre(
        context: &RequestContext,
        id: juniper::ID,
        input: GenreInput,
    ) -> juniper::FieldResult<Genre> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            diesel::update(genres::table.find(id))
                .set(&input)
                .execute(&conn)?;
            Ok(genres::table.find(id).get_result(&conn)?)
        })
    }

    fn delete_genre(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<bool> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(diesel::delete(genres::table.find(id)).execute(&conn)? == 1)
    }

    fn update_track(
        context: &RequestContext,
        id: juniper::ID,
        input: TrackInput,
    ) -> juniper::FieldResult<Track> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let album_id = if let Some(album_id) = input.album_id {
                Some(ExternalId(album_id).try_into()?)
            } else {
                None
            };
            let artist_id = if let Some(artist_id) = input.artist_id {
                Some(ExternalId(artist_id).try_into()?)
            } else {
                None
            };
            let genre_id = if let Some(genre_id) = input.genre_id {
                Some(ExternalId(genre_id).try_into()?)
            } else {
                None
            };
            let track_changeset = TrackChangeset {
                name: input.name,
                album_id,
                artist_id,
                genre_id,
                track_number: input.track_number,
            };
            diesel::update(tracks::table.find(id))
                .set(&track_changeset)
                .execute(&conn)?;
            Ok(tracks::table.find(id).get_result(&conn)?)
        })
    }

    fn delete_track(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<bool> {
        let filepath = {
            let mut filepath = context.tracks_dir.clone();
            filepath.push(&id[..]);
            filepath.set_extension("mp3");
            filepath
        };
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        let deleted = diesel::delete(tracks::table.find(id)).execute(&conn)? == 1;
        if deleted {
            std::fs::remove_file(filepath)?;
        }
        Ok(deleted)
    }

    fn create_playlist(
        context: &RequestContext,
        input: PlaylistInput,
    ) -> juniper::FieldResult<Playlist> {
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let new_playlist = NewPlaylist {
                id: prng::rand_i32(&conn)?,
                name: input.name,
            };
            diesel::insert_into(playlists::table)
                .values(&new_playlist)
                .execute(&conn)?;
            Ok(playlists::table.find(new_playlist.id).get_result(&conn)?)
        })
    }

    fn update_playlist(
        context: &RequestContext,
        id: juniper::ID,
        input: PlaylistInput,
    ) -> juniper::FieldResult<Playlist> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            diesel::update(playlists::table.find(id))
                .set(&input)
                .execute(&conn)?;
            Ok(playlists::table.find(id).get_result(&conn)?)
        })
    }

    fn delete_playlist(context: &RequestContext, id: juniper::ID) -> juniper::FieldResult<bool> {
        let id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        Ok(diesel::delete(playlists::table.find(id)).execute(&conn)? == 1)
    }

    fn create_playlist_track(
        context: &RequestContext,
        id: juniper::ID, // playlist_id
        input: PlaylistTrackInput,
    ) -> juniper::FieldResult<Playlist> {
        let playlist_id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let count: i64 = playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(playlist_id))
                .count()
                .get_result(&conn)?;
            let count: i32 = i32::try_from(count)?;
            let playlist_track_input = PlaylistTrackInput {
                position: input.position.or_else(|| Some(count)),
                ..input
            };
            let position = playlist_track_input.position.unwrap();
            if position < 0 || count < position {
                Err(diesel::result::Error::RollbackTransaction)?;
            }
            if position != count {
                diesel::update(
                    playlists_tracks::table
                        .filter(playlists_tracks::playlist_id.eq(playlist_id))
                        .filter(playlists_tracks::position.ge(position)),
                )
                .set(playlists_tracks::position.eq(playlists_tracks::position + 1))
                .execute(&conn)?;
            }
            let new_playlist_track = NewPlaylistTrack {
                track_id: ExternalId(playlist_track_input.track_id).try_into()?,
                position: playlist_track_input.position,
            };
            diesel::insert_into(playlists_tracks::table)
                .values((
                    playlists_tracks::playlist_id.eq(playlist_id),
                    &new_playlist_track,
                ))
                .execute(&conn)?;
            Ok(playlists::table.find(playlist_id).get_result(&conn)?)
        })
    }

    fn update_playlist_track(
        context: &RequestContext,
        id: juniper::ID, // playlist_id
        input: PlaylistTrackOrderInput,
    ) -> juniper::FieldResult<Playlist> {
        let playlist_id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let range_start = usize::try_from(input.range_start)?;
            let range_length = usize::try_from(input.range_length.unwrap_or(1))?;
            if range_length < 1 {
                Err(diesel::result::Error::RollbackTransaction)?;
            }
            let insert_before = usize::try_from(input.insert_before)?;
            if range_start < insert_before && insert_before < range_start + range_length {
                Err(diesel::result::Error::RollbackTransaction)?;
            }
            let mut playlist_tracks = playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(playlist_id))
                .order(playlists_tracks::position.asc())
                .load::<PlaylistTrack>(&conn)?;
            let len = playlist_tracks.len();
            if range_start > len - 1 || range_start + range_length > len || insert_before > len {
                Err(diesel::result::Error::RollbackTransaction)?;
            }
            if insert_before < range_start {
                let slice = &mut playlist_tracks[insert_before..range_start + range_length];
                slice.rotate_left(range_start - insert_before);
                for (i, playlist_track) in slice.iter().enumerate() {
                    let delta = i32::try_from(i)?
                        - (playlist_track.position - i32::try_from(insert_before)?);
                    diesel::update(playlists_tracks::table.find(playlist_track.id))
                        .set(playlists_tracks::position.eq(playlists_tracks::position + delta))
                        .execute(&conn)?;
                }
            } else if insert_before > range_start + range_length {
                let slice = &mut playlist_tracks[range_start..insert_before];
                slice.rotate_right(insert_before - (range_start + range_length));
                for (i, playlist_track) in slice.iter().enumerate() {
                    let delta =
                        i32::try_from(i)? - (playlist_track.position - i32::try_from(range_start)?);
                    diesel::update(playlists_tracks::table.find(playlist_track.id))
                        .set(playlists_tracks::position.eq(playlists_tracks::position + delta))
                        .execute(&conn)?;
                }
            }
            Ok(playlists::table.find(playlist_id).get_result(&conn)?)
        })
    }

    fn delete_playlist_track(
        context: &RequestContext,
        id: juniper::ID, // playlist_id
        input: PlaylistTrackInput,
    ) -> juniper::FieldResult<Playlist> {
        let playlist_id: i32 = ExternalId(id).try_into()?;
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let new_playlist_track = NewPlaylistTrack {
                track_id: ExternalId(input.track_id).try_into()?,
                position: input.position,
            };
            let deleted = if let Some(position) = new_playlist_track.position {
                diesel::delete(
                    playlists_tracks::table
                        .filter(playlists_tracks::playlist_id.eq(playlist_id))
                        .filter(playlists_tracks::track_id.eq(new_playlist_track.track_id))
                        .filter(playlists_tracks::position.eq(position)),
                )
                .execute(&conn)?
                    == 1
            } else {
                diesel::delete(
                    playlists_tracks::table
                        .filter(playlists_tracks::playlist_id.eq(playlist_id))
                        .filter(playlists_tracks::track_id.eq(new_playlist_track.track_id)),
                )
                .execute(&conn)?
                    >= 1
            };
            if !deleted {
                Err(diesel::result::Error::RollbackTransaction)?;
            }
            let playlist_tracks = playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(playlist_id))
                .order(playlists_tracks::position.asc())
                .load::<PlaylistTrack>(&conn)?;
            for (i, playlist_track) in playlist_tracks.iter().enumerate() {
                let i = i32::try_from(i)?;
                if playlist_track.position != i {
                    diesel::update(playlists_tracks::table.find(playlist_track.id))
                        .set(playlists_tracks::position.eq(i))
                        .execute(&conn)?;
                }
            }
            Ok(playlists::table.find(playlist_id).get_result(&conn)?)
        })
    }

    fn update_user(
        context: &RequestContext,
        username: String,
        input: UserInput,
    ) -> juniper::FieldResult<bool> {
        let conn = context.pool.get()?;
        conn.transaction::<_, juniper::FieldError, _>(|| {
            let user_changeset = UserChangeset {
                password: Vec::from(Sha256::digest(input.password.as_bytes()).as_slice()),
            };
            diesel::update(users::table.find(username))
                .set(&user_changeset)
                .execute(&conn)?;
            Ok(true)
        })
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
