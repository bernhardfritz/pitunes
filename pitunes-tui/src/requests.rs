use std::convert::TryFrom;
use std::sync::Arc;

use graphql_client::{GraphQLQuery, Response};

use crate::{
    constants::GRAPHQL,
    models::{
        exports::{
            album_query, album_tracks_query, albums_query, artist_albums_query, artist_query,
            artist_tracks_query, artists_query, create_album_mutation, create_artist_mutation,
            create_genre_mutation, create_playlist_mutation, create_playlist_track_mutation,
            delete_album_mutation, delete_artist_mutation, delete_genre_mutation,
            delete_playlist_mutation, delete_playlist_track_mutation, genre_query,
            genre_tracks_query, genres_query, playlist_tracks_query, playlists_query, track_query,
            tracks_query, update_album_mutation, update_artist_mutation, update_genre_mutation,
            update_playlist_mutation, update_playlist_track_mutation, update_track_mutation,
        },
        Album, AlbumQuery, AlbumTracksQuery, AlbumsQuery, Artist, ArtistAlbumsQuery, ArtistQuery,
        ArtistTracksQuery, ArtistsQuery, CreateAlbumMutation, CreateArtistMutation,
        CreateGenreMutation, CreatePlaylistMutation, CreatePlaylistTrackMutation,
        DeleteAlbumMutation, DeleteArtistMutation, DeleteGenreMutation, DeletePlaylistMutation,
        DeletePlaylistTrackMutation, Genre, GenreQuery, GenreTracksQuery, GenresQuery, Playlist,
        PlaylistTracksQuery, PlaylistsQuery, Track, TrackQuery, TracksQuery, UpdateAlbumMutation,
        UpdateArtistMutation, UpdateGenreMutation, UpdatePlaylistMutation,
        UpdatePlaylistTrackMutation, UpdateTrackMutation,
    },
    Context,
};

pub fn read_album(context: &Arc<Context>, id: i64) -> Album {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = AlbumQuery::build_query(album_query::Variables { id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<album_query::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.album)
        .map(|album| album.into())
        .unwrap()
}

pub fn read_albums(context: &Arc<Context>) -> Vec<Album> {
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

pub fn update_album(context: &Arc<Context>, variables: update_album_mutation::Variables) -> Album {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = UpdateAlbumMutation::build_query(variables);
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<update_album_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.update_album)
        .map(|album| album.into())
        .unwrap()
}

pub fn delete_album(context: &Arc<Context>, album: &Album) -> bool {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        DeleteAlbumMutation::build_query(delete_album_mutation::Variables { id: album.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<delete_album_mutation::ResponseData> = res.json().unwrap();
    response_body.data.map(|data| data.delete_album).unwrap()
}

pub fn read_artist(context: &Arc<Context>, id: i64) -> Artist {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = ArtistQuery::build_query(artist_query::Variables { id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<artist_query::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.artist)
        .map(|artist| artist.into())
        .unwrap()
}

pub fn read_artists(context: &Arc<Context>) -> Vec<Artist> {
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

pub fn update_artist(
    context: &Arc<Context>,
    variables: update_artist_mutation::Variables,
) -> Artist {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = UpdateArtistMutation::build_query(variables);
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<update_artist_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.update_artist)
        .map(|artist| artist.into())
        .unwrap()
}

pub fn delete_artist(context: &Arc<Context>, artist: &Artist) -> bool {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        DeleteArtistMutation::build_query(delete_artist_mutation::Variables { id: artist.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<delete_artist_mutation::ResponseData> = res.json().unwrap();
    response_body.data.map(|data| data.delete_artist).unwrap()
}

pub fn read_genre(context: &Arc<Context>, id: i64) -> Genre {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = GenreQuery::build_query(genre_query::Variables { id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<genre_query::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.genre)
        .map(|genre| genre.into())
        .unwrap()
}

pub fn read_genres(context: &Arc<Context>) -> Vec<Genre> {
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

pub fn update_genre(context: &Arc<Context>, variables: update_genre_mutation::Variables) -> Genre {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = UpdateGenreMutation::build_query(variables);
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<update_genre_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.update_genre)
        .map(|genre| genre.into())
        .unwrap()
}

pub fn delete_genre(context: &Arc<Context>, genre: &Genre) -> bool {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        DeleteGenreMutation::build_query(delete_genre_mutation::Variables { id: genre.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<delete_genre_mutation::ResponseData> = res.json().unwrap();
    response_body.data.map(|data| data.delete_genre).unwrap()
}

pub fn create_album(context: &Arc<Context>, name: &str) -> Album {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = CreateAlbumMutation::build_query(create_album_mutation::Variables {
        album_input: create_album_mutation::AlbumInput {
            name: String::from(name),
        },
    });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<create_album_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.create_album)
        .map(|album| album.into())
        .unwrap()
}

pub fn create_artist(context: &Arc<Context>, name: &str) -> Artist {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = CreateArtistMutation::build_query(create_artist_mutation::Variables {
        artist_input: create_artist_mutation::ArtistInput {
            name: String::from(name),
        },
    });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<create_artist_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.create_artist)
        .map(|artist| artist.into())
        .unwrap()
}

pub fn create_genre(context: &Arc<Context>, name: &str) -> Genre {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = CreateGenreMutation::build_query(create_genre_mutation::Variables {
        genre_input: create_genre_mutation::GenreInput {
            name: String::from(name),
        },
    });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<create_genre_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.create_genre)
        .map(|genre| genre.into())
        .unwrap()
}

pub fn create_playlist(context: &Arc<Context>, name: &str) -> Playlist {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = CreatePlaylistMutation::build_query(create_playlist_mutation::Variables {
        playlist_input: create_playlist_mutation::PlaylistInput {
            name: String::from(name),
        },
    });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<create_playlist_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.create_playlist)
        .map(|playlist| playlist.into())
        .unwrap()
}

pub fn delete_playlist(context: &Arc<Context>, playlist: &Playlist) -> bool {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = DeletePlaylistMutation::build_query(delete_playlist_mutation::Variables {
        id: playlist.id,
    });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<delete_playlist_mutation::ResponseData> = res.json().unwrap();
    response_body.data.map(|data| data.delete_playlist).unwrap()
}

pub fn read_playlists(context: &Arc<Context>) -> Vec<Playlist> {
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

pub fn update_playlist(
    context: &Arc<Context>,
    variables: update_playlist_mutation::Variables,
) -> Playlist {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = UpdatePlaylistMutation::build_query(variables);
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<update_playlist_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.update_playlist)
        .map(|playlist| playlist.into())
        .unwrap()
}

pub fn read_track(context: &Arc<Context>, id: i64) -> Track {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = TrackQuery::build_query(track_query::Variables { id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<track_query::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.track)
        .map(|track| track.into())
        .unwrap()
}

pub fn read_tracks(context: &Arc<Context>) -> Vec<Track> {
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

pub fn update_track(context: &Arc<Context>, variables: update_track_mutation::Variables) -> Track {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body = UpdateTrackMutation::build_query(variables);
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<update_track_mutation::ResponseData> = res.json().unwrap();
    response_body
        .data
        .map(|data| data.update_track)
        .map(|track| track.into())
        .unwrap()
}

pub fn read_tracks_of_album(context: &Arc<Context>, album: &Album) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        AlbumTracksQuery::build_query(album_tracks_query::Variables { id: album.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<album_tracks_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.album)
        .map(|album| album.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn read_tracks_of_artist(context: &Arc<Context>, artist: &Artist) -> Vec<Track> {
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

pub fn read_albums_of_artist(context: &Arc<Context>, artist: &Artist) -> Vec<Album> {
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

pub fn read_tracks_of_genre(context: &Arc<Context>, genre: &Genre) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        GenreTracksQuery::build_query(genre_tracks_query::Variables { id: genre.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<genre_tracks_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.genre)
        .map(|genre| genre.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn read_tracks_of_playlist(context: &Arc<Context>, playlist: &Playlist) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        PlaylistTracksQuery::build_query(playlist_tracks_query::Variables { id: playlist.id });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<playlist_tracks_query::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.playlist)
        .map(|playlist| playlist.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}

pub fn create_playlist_track(
    context: &Arc<Context>,
    playlist: &Playlist,
    track: &Track,
) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        CreatePlaylistTrackMutation::build_query(create_playlist_track_mutation::Variables {
            id: playlist.id,
            playlist_track_input: create_playlist_track_mutation::PlaylistTrackInput {
                id: track.id,
                position: None,
            },
        });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<create_playlist_track_mutation::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.create_playlist_track)
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
    position: Option<usize>,
) -> Vec<Track> {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        DeletePlaylistTrackMutation::build_query(delete_playlist_track_mutation::Variables {
            id: playlist.id,
            playlist_track_input: delete_playlist_track_mutation::PlaylistTrackInput {
                id: track.id,
                position: position.map(|p| p as i64),
            },
        });
    let res = context
        .client
        .post(&url)
        .bearer_auth(&context.api_key[..])
        .json(&request_body)
        .send()
        .unwrap();
    let response_body: Response<delete_playlist_track_mutation::ResponseData> = res.json().unwrap();
    let tracks = response_body
        .data
        .map(|data| data.delete_playlist_track)
        .map(|playlist| playlist.tracks)
        .unwrap();
    tracks.into_iter().map(|track| track.into()).collect()
}
