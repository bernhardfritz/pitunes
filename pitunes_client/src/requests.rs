use crate::constants::GRAPHQL;
use crate::models::exports::{
    album_query, albums_query, artist_albums_query, artist_tracks_query, artists_query,
    delete_playlist_track_mutation, genre_query, genres_query, playlist_query, playlists_query,
    tracks_query, update_playlist_track_mutation,
};
use crate::models::{
    Album, AlbumQuery, AlbumsQuery, Artist, ArtistAlbumsQuery, ArtistTracksQuery, ArtistsQuery,
    DeletePlaylistTrackMutation, Genre, GenreQuery, GenresQuery, Playlist, PlaylistQuery,
    PlaylistsQuery, Track, TracksQuery, UpdatePlaylistTrackMutation,
};
use crate::Context;
use graphql_client::{GraphQLQuery, Response};
use std::convert::TryFrom;
use std::sync::Arc;

pub fn get_albums(context: &Arc<Context>) -> Vec<Album> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = AlbumsQuery::build_query(albums_query::Variables {});
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<albums_query::ResponseData> = res.json().unwrap();
    let albums = response_body.data.map(|data| data.albums).unwrap();
    albums.into_iter().map(|album| album.into()).collect()
}

pub fn get_artists(context: &Arc<Context>) -> Vec<Artist> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = ArtistsQuery::build_query(artists_query::Variables {});
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<artists_query::ResponseData> = res.json().unwrap();
    let artists = response_body.data.map(|data| data.artists).unwrap();
    artists.into_iter().map(|artist| artist.into()).collect()
}

pub fn get_genres(context: &Arc<Context>) -> Vec<Genre> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = GenresQuery::build_query(genres_query::Variables {});
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<genres_query::ResponseData> = res.json().unwrap();
    let genres = response_body.data.map(|data| data.genres).unwrap();
    genres.into_iter().map(|genre| genre.into()).collect()
}

pub fn get_playlists(context: &Arc<Context>) -> Vec<Playlist> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = PlaylistsQuery::build_query(playlists_query::Variables {});
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<playlists_query::ResponseData> = res.json().unwrap();
    let playlists = response_body.data.map(|data| data.playlists).unwrap();
    playlists
        .into_iter()
        .map(|playlist| playlist.into())
        .collect()
}

pub fn get_tracks(context: &Arc<Context>) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = TracksQuery::build_query(tracks_query::Variables {});
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<tracks_query::ResponseData> = res.json().unwrap();
    let tracks = response_body.data.map(|data| data.tracks).unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn get_tracks_of_album(context: &Arc<Context>, album: &Album) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = AlbumQuery::build_query(album_query::Variables { id: album.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<album_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.album)
        .map(|album| album.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn get_tracks_of_artist(context: &Arc<Context>, artist: &Artist) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        ArtistTracksQuery::build_query(artist_tracks_query::Variables { id: artist.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<artist_tracks_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.artist)
        .map(|artist| artist.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn get_albums_of_artist(context: &Arc<Context>, artist: &Artist) -> Vec<Album> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        ArtistAlbumsQuery::build_query(artist_albums_query::Variables { id: artist.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<artist_albums_query::ResponseData> = res.json().unwrap();
    let albums = response_body
        .data
        .map(|data| data.artist)
        .map(|artist| artist.albums)
        .unwrap();
    albums.into_iter().map(|album| album.into()).collect()
}

pub fn get_tracks_of_genre(context: &Arc<Context>, genre: &Genre) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = GenreQuery::build_query(genre_query::Variables { id: genre.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<genre_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.genre)
        .map(|genre| genre.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn get_tracks_of_playlist(context: &Arc<Context>, playlist: &Playlist) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = PlaylistQuery::build_query(playlist_query::Variables { id: playlist.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<playlist_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.playlist)
        .map(|playlist| playlist.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn update_playlist_track(
    context: &Arc<Context>,
    playlist: &Playlist,
    range_start: usize,
    insert_before: usize,
) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        UpdatePlaylistTrackMutation::build_query(update_playlist_track_mutation::Variables {
            id: playlist.id,
            playlist_track_order_input: update_playlist_track_mutation::PlaylistTrackOrderInput {
                range_start: i64::try_from(range_start).unwrap(),
                range_length: None,
                insert_before: i64::try_from(insert_before).unwrap(),
            },
        });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<update_playlist_track_mutation::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.update_playlist_track)
        .map(|playlist| playlist.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn delete_playlist_track(
    context: &Arc<Context>,
    playlist: &Playlist,
    track: &Track,
    position: Option<i64>,
) -> bool {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        DeletePlaylistTrackMutation::build_query(delete_playlist_track_mutation::Variables {
            playlist_id: playlist.id,
            track_id: track.id,
            position,
        });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<delete_playlist_track_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.delete_playlist_track)
        .unwrap()
}
