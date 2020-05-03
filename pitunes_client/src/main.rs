#[allow(dead_code)]
mod event;

mod http_stream_reader;
use http_stream_reader::HttpStreamReader;
use std::sync::Mutex;

use clap::{self, value_t};
use dotenv::dotenv;
use graphql_client::{GraphQLQuery, Response};
use if_chain::if_chain;
use redux_rs::{combine_reducers, Reducer, Store};
use std::convert::TryFrom;
use std::env;
use std::io::{self, Write};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Text};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

const GRAPHQL: &str = "graphql";
const STATIC: &str = "static";

const PI_SYMBOL: &str = "π";
const ALBUMS: &str = "Albums";
const ARTISTS: &str = "Artists";
const GENRES: &str = "Genres";
const PLAYLISTS: &str = "Playlists";
const TRACKS: &str = "Tracks";
const ALL_TRACKS: &str = "All tracks";

const REDUCER: Reducer<State, Key> = combine_reducers!(
    State,
    &Key,
    global_reducer,
    list_reducer,
    edit_reducer,
    tracks_reducer,
    albums_reducer,
    artist_reducer,
    artists_reducer,
    genres_reducer,
    playlists_reducer,
    root_reducer
);

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/album_query.graphql",
    response_derives = "Debug"
)]
pub struct AlbumQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/albums_query.graphql",
    response_derives = "Debug"
)]
pub struct AlbumsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artist_albums_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistAlbumsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artist_tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistTracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artists_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/genre_query.graphql",
    response_derives = "Debug"
)]
pub struct GenreQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/genres_query.graphql",
    response_derives = "Debug"
)]
pub struct GenresQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/playlist_query.graphql",
    response_derives = "Debug"
)]
pub struct PlaylistQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/playlists_query.graphql",
    response_derives = "Debug"
)]
pub struct PlaylistsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/track_query.graphql",
    response_derives = "Debug"
)]
pub struct TrackQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct TracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_playlist_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdatePlaylistTrackMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_playlist_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeletePlaylistTrackMutation;

#[derive(Clone)]
struct Album {
    id: i64,
    name: String,
}

#[derive(Clone)]
struct Artist {
    id: i64,
    name: String,
}

#[derive(Clone)]
struct Genre {
    id: i64,
    name: String,
}

#[derive(Clone)]
struct Playlist {
    id: i64,
    name: String,
}

#[derive(Clone)]
struct Track {
    id: i64,
    name: String,
}

impl From<album_query::AlbumQueryAlbumTracks> for Track {
    fn from(
        album_query::AlbumQueryAlbumTracks { id, name }: album_query::AlbumQueryAlbumTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<albums_query::AlbumsQueryAlbums> for Album {
    fn from(
        albums_query::AlbumsQueryAlbums { id, name }: albums_query::AlbumsQueryAlbums,
    ) -> Album {
        Album { id, name }
    }
}

impl From<artist_albums_query::ArtistAlbumsQueryArtistAlbums> for Album {
    fn from(
        artist_albums_query::ArtistAlbumsQueryArtistAlbums { id, name }: artist_albums_query::ArtistAlbumsQueryArtistAlbums,
    ) -> Album {
        Album { id, name }
    }
}

impl From<artists_query::ArtistsQueryArtists> for Artist {
    fn from(
        artists_query::ArtistsQueryArtists { id, name }: artists_query::ArtistsQueryArtists,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracks> for Track {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracks { id, name }: artist_tracks_query::ArtistTracksQueryArtistTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<genre_query::GenreQueryGenreTracks> for Track {
    fn from(
        genre_query::GenreQueryGenreTracks { id, name }: genre_query::GenreQueryGenreTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<genres_query::GenresQueryGenres> for Genre {
    fn from(
        genres_query::GenresQueryGenres { id, name }: genres_query::GenresQueryGenres,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<playlist_query::PlaylistQueryPlaylistTracks> for Track {
    fn from(
        playlist_query::PlaylistQueryPlaylistTracks { id, name }: playlist_query::PlaylistQueryPlaylistTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<playlists_query::PlaylistsQueryPlaylists> for Playlist {
    fn from(
        playlists_query::PlaylistsQueryPlaylists { id, name }: playlists_query::PlaylistsQueryPlaylists,
    ) -> Playlist {
        Playlist { id, name }
    }
}

impl From<tracks_query::TracksQueryTracks> for Track {
    fn from(
        tracks_query::TracksQueryTracks { id, name }: tracks_query::TracksQueryTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks>
    for Track
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks { id, name }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks,
    ) -> Track {
        Track { id, name }
    }
}

use crate::event::{Event, Events};

#[derive(Clone)]
enum Model {
    Albums { albums: Vec<Album> },
    Artist { artist: Artist, albums: Vec<Album> },
    Artists { artists: Vec<Artist> },
    Genres { genres: Vec<Genre> },
    Playlists { playlists: Vec<Playlist> },
    Root,
    Tracks { tracks: Vec<Track> },
}

#[derive(Clone)]
enum View {
    List {
        list_state: ListState,
        items: Vec<String>,
    },
    Edit {
        input_field: String, // TODO somehow encode form fields
    },
}

#[derive(Clone)]
struct State {
    context: Arc<Context>,
    break_condition: bool,
    model: Model,
    view: View,
    history: Vec<State>,
    add_to_history: bool,
}

fn global_reducer(state: &State, action: &Key) -> State {
    match action {
        Key::Ctrl('c') => State {
            break_condition: true,
            ..state.clone()
        },
        _ => state.clone(),
    }
}

fn list_reducer(state: &State, action: &Key) -> State {
    let new_state = if let View::List { list_state, items } = &state.view {
        match action {
            Key::Up => {
                let i = match list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            items.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                let list_state = {
                    let mut list_state = ListState::default();
                    list_state.select(Some(i));
                    list_state
                };
                Some(State {
                    view: View::List {
                        list_state,
                        items: items.clone(),
                    },
                    ..state.clone()
                })
            }
            Key::Down => {
                let i = match list_state.selected() {
                    Some(i) => {
                        if i >= items.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                let list_state = {
                    let mut list_state = ListState::default();
                    list_state.select(Some(i));
                    list_state
                };
                Some(State {
                    view: View::List {
                        list_state,
                        items: items.clone(),
                    },
                    ..state.clone()
                })
            }
            Key::Char('\n') => {
                if state.add_to_history {
                    let mut history = state.history.clone();
                    history.push(state.clone());
                    Some(State {
                        history,
                        ..state.clone()
                    })
                } else {
                    None
                }
            }
            Key::Backspace => {
                if let Some(last) = state.history.last() {
                    Some(last.clone())
                } else {
                    None
                }
            }
            Key::Char(' ') => {
                let sink_guard = state.context.sink_lock.read().unwrap();
                if sink_guard.is_paused() {
                    sink_guard.play();
                } else {
                    sink_guard.pause();
                }
                Some(state.clone())
            }
            _ => None,
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn edit_reducer(state: &State, action: &Key) -> State {
    let new_state = if let View::Edit { input_field } = &state.view {
        match action {
            Key::Char(c) => {
                let mut input_field = input_field.clone();
                input_field.push(*c);
                Some(State {
                    view: View::Edit { input_field },
                    ..state.clone()
                })
            }
            Key::Backspace => {
                let mut input_field = input_field.clone();
                input_field.pop();
                Some(State {
                    view: View::Edit { input_field },
                    ..state.clone()
                })
            }
            Key::Esc => {
                if let Some(last) = state.history.last() {
                    Some(REDUCER(last, &Key::Char('\n')))
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn root_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Root = state.model {
        if let View::List { list_state, items } = &state.view {
            match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let item = &items[selected];
                        match &item[..] {
                            ALBUMS => {
                                let albums = get_albums(&state.context);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    list_state.select(Some(0));
                                    list_state
                                };
                                let items = albums.iter().map(|album| album.name.clone()).collect();
                                Some(State {
                                    model: Model::Albums { albums },
                                    view: View::List { list_state, items },
                                    ..state.clone()
                                })
                            }
                            ARTISTS => {
                                let artists = get_artists(&state.context);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    list_state.select(Some(0));
                                    list_state
                                };
                                let items =
                                    artists.iter().map(|artist| artist.name.clone()).collect();
                                Some(State {
                                    model: Model::Artists { artists },
                                    view: View::List { list_state, items },
                                    ..state.clone()
                                })
                            }
                            GENRES => {
                                let genres = get_genres(&state.context);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    list_state.select(Some(0));
                                    list_state
                                };
                                let items = genres.iter().map(|genre| genre.name.clone()).collect();
                                Some(State {
                                    model: Model::Genres { genres },
                                    view: View::List { list_state, items },
                                    ..state.clone()
                                })
                            }
                            PLAYLISTS => {
                                let playlists = get_playlists(&state.context);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    list_state.select(Some(0));
                                    list_state
                                };
                                let items = playlists
                                    .iter()
                                    .map(|playlist| playlist.name.clone())
                                    .collect();
                                Some(State {
                                    model: Model::Playlists { playlists },
                                    view: View::List { list_state, items },
                                    ..state.clone()
                                })
                            }
                            TRACKS => {
                                let tracks = get_tracks(&state.context);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    list_state.select(Some(0));
                                    list_state
                                };
                                let items = tracks.iter().map(|track| track.name.clone()).collect();
                                Some(State {
                                    model: Model::Tracks { tracks },
                                    view: View::List { list_state, items },
                                    add_to_history: false,
                                    ..state.clone()
                                })
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };
    if let Some(state) = new_state {
        state
    } else {
        state.clone()
    }
}

fn albums_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Albums { albums } = &state.model {
        match &state.view {
            View::List {
                list_state,
                items: _,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let album = &albums[selected];
                        let tracks = get_tracks_of_album(&state.context, &album);
                        let list_state = {
                            let mut list_state = ListState::default();
                            list_state.select(Some(0));
                            list_state
                        };
                        let items = tracks.iter().map(|track| track.name.clone()).collect();
                        Some(State {
                            model: Model::Tracks { tracks },
                            view: View::List { list_state, items },
                            add_to_history: false,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                Key::Char('e') => Some(State {
                    view: View::Edit {
                        input_field: String::from("Placeholder"),
                    },
                    ..state.clone()
                }),
                _ => None,
            },
            View::Edit { input_field: _ } => {
                match action {
                    Key::Char('\n') => {
                        // TODO do something with input_field
                        None
                    }
                    _ => None,
                }
            }
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn artist_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Artist { artist, albums } = &state.model {
        if let View::List {
            list_state,
            items: _,
        } = &state.view
        {
            match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let tracks = if selected == 0 {
                            get_tracks_of_artist(&state.context, artist)
                        } else {
                            let album = &albums[selected - 1];
                            get_tracks_of_album(&state.context, album)
                        };
                        let list_state = {
                            let mut list_state = ListState::default();
                            list_state.select(Some(0));
                            list_state
                        };
                        let items = tracks.iter().map(|track| track.name.clone()).collect();
                        Some(State {
                            model: Model::Tracks { tracks },
                            view: View::List { list_state, items },
                            add_to_history: false,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn artists_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Artists { artists } = &state.model {
        if let View::List {
            list_state,
            items: _,
        } = &state.view
        {
            match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let artist = artists[selected].clone();
                        let albums = get_albums_of_artist(&state.context, &artist);
                        let list_state = {
                            let mut list_state = ListState::default();
                            list_state.select(Some(0));
                            list_state
                        };
                        let items = {
                            let mut items: Vec<String> =
                                albums.iter().map(|album| album.name.clone()).collect();
                            items.insert(0, String::from(ALL_TRACKS));
                            items
                        };
                        Some(State {
                            model: Model::Artist { artist, albums },
                            view: View::List { list_state, items },
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn genres_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Genres { genres } = &state.model {
        if let View::List {
            list_state,
            items: _,
        } = &state.view
        {
            match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let genre = &genres[selected];
                        let tracks = get_tracks_of_genre(&state.context, genre);
                        let list_state = {
                            let mut list_state = ListState::default();
                            list_state.select(Some(0));
                            list_state
                        };
                        let items = tracks.iter().map(|track| track.name.clone()).collect();
                        Some(State {
                            model: Model::Tracks { tracks },
                            view: View::List { list_state, items },
                            add_to_history: false,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn playlists_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Playlists { playlists } = &state.model {
        if let View::List {
            list_state,
            items: _,
        } = &state.view
        {
            match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let playlist = &playlists[selected];
                        let tracks = get_tracks_of_playlist(&state.context, playlist);
                        let list_state = {
                            let mut list_state = ListState::default();
                            list_state.select(Some(0));
                            list_state
                        };
                        let items = tracks.iter().map(|track| track.name.clone()).collect();
                        Some(State {
                            model: Model::Tracks { tracks },
                            view: View::List { list_state, items },
                            add_to_history: false,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn tracks_reducer(state: &State, action: &Key) -> State {
    let new_state = if_chain! {
        if let Model::Tracks { tracks } = &state.model;
        if let View::List { list_state: tracks_list_state, items: _ } = &state.view;
        if let Some(track_index) = tracks_list_state.selected();
        then {
            match action {
                Key::Char('\n') => {
                    let mut queue: Vec<i64> = tracks.iter().map(|track| track.id).collect();
                    queue.rotate_left(track_index);
                    play_queue(state.context.clone(), queue);
                    None
                }
                _ => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state: prev_list_state, items: _ } = &last.view;
                        then {
                            match &last.model {
                                Model::Playlists { playlists } => {
                                    if let Some(playlist_index) = prev_list_state.selected() {
                                        let playlist = &playlists[playlist_index];
                                        match action {
                                            Key::Char('d') => {
                                                let track = &tracks[track_index];
                                                let position = Some(i64::try_from(track_index).unwrap());
                                                let deleted = delete_playlist_track(&state.context, playlist, track, position);
                                                if deleted {
                                                    Some(REDUCER(last, &Key::Char('\n')))
                                                } else {
                                                    None
                                                }
                                            }
                                            Key::Char('j') => {
                                                if track_index > 0 {
                                                    let range_start = track_index;
                                                    let insert_before = track_index - 1;
                                                    let tracks = update_playlist_track(
                                                        &state.context,
                                                        &playlist,
                                                        range_start,
                                                        insert_before,
                                                    );
                                                    let list_state = {
                                                        let mut list_state = ListState::default();
                                                        let selected = Some(
                                                            if range_start == insert_before || range_start + 1 == insert_before {
                                                                range_start
                                                            } else if range_start < insert_before {
                                                                range_start + 1
                                                            } else {
                                                                range_start - 1
                                                            },
                                                        );
                                                        list_state.select(selected);
                                                        list_state
                                                    };
                                                    let items = tracks.iter().map(|track| track.name.clone()).collect();
                                                    Some(State {
                                                        model: Model::Tracks { tracks },
                                                        view: View::List { list_state, items },
                                                        add_to_history: false,
                                                        ..state.clone()
                                                    })
                                                } else {
                                                    None
                                                }
                                            }
                                            Key::Char('k') => {
                                                if track_index < tracks.len() - 1 {
                                                    let range_start = track_index;
                                                    let insert_before = track_index + 2;
                                                    let tracks = update_playlist_track(
                                                        &state.context,
                                                        &playlist,
                                                        range_start,
                                                        insert_before,
                                                    );
                                                    let list_state = {
                                                        let mut list_state = ListState::default();
                                                        let selected = Some(
                                                            if range_start == insert_before || range_start + 1 == insert_before {
                                                                range_start
                                                            } else if range_start < insert_before {
                                                                range_start + 1
                                                            } else {
                                                                range_start - 1
                                                            },
                                                        );
                                                        list_state.select(selected);
                                                        list_state
                                                    };
                                                    let items = tracks.iter().map(|track| track.name.clone()).collect();
                                                    Some(State {
                                                        model: Model::Tracks { tracks },
                                                        view: View::List { list_state, items },
                                                        add_to_history: false,
                                                        ..state.clone()
                                                    })
                                                } else {
                                                    None
                                                }
                                            }
                                            _ => None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                _ => None
                            }
                        } else {
                            None
                        }
                    }
                }
            }
        } else {
            None
        }
    };
    if let Some(new_state) = new_state {
        new_state
    } else {
        state.clone()
    }
}

fn get_albums(context: &Arc<Context>) -> Vec<Album> {
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

fn get_artists(context: &Arc<Context>) -> Vec<Artist> {
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

fn get_genres(context: &Arc<Context>) -> Vec<Genre> {
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

fn get_playlists(context: &Arc<Context>) -> Vec<Playlist> {
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

fn get_tracks(context: &Arc<Context>) -> Vec<Track> {
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

fn get_tracks_of_album(context: &Arc<Context>, album: &Album) -> Vec<Track> {
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

fn get_tracks_of_artist(context: &Arc<Context>, artist: &Artist) -> Vec<Track> {
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

fn get_albums_of_artist(context: &Arc<Context>, artist: &Artist) -> Vec<Album> {
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

fn get_tracks_of_genre(context: &Arc<Context>, genre: &Genre) -> Vec<Track> {
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

fn get_tracks_of_playlist<'a>(context: &Arc<Context>, playlist: &Playlist) -> Vec<Track> {
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

fn update_playlist_track(
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

fn delete_playlist_track(
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

// Look away, I'm hideous!
fn play_queue(context: Arc<Context>, queue: Vec<i64>) {
    thread_local!(static JOIN_HANDLE_MUTEX: Mutex<Option<JoinHandle<()>>> = Mutex::new(None));
    {
        let mut queue_guard = context.queue_lock.write().unwrap();
        queue_guard.clear();
    }
    {
        let sink_guard = context.sink_lock.read().unwrap();
        sink_guard.stop();
    }
    let join_handle = JOIN_HANDLE_MUTEX.with(|join_handle_mutex| {
        let mut join_handle_guard = join_handle_mutex.lock().unwrap();
        std::mem::replace(&mut *join_handle_guard, None)
    });
    if let Some(join_handle) = join_handle {
        join_handle.join().unwrap();
    }
    let join_handle = if queue.is_empty() {
        None
    } else {
        {
            let mut queue_guard = context.queue_lock.write().unwrap();
            *queue_guard = queue;
        }
        {
            let mut sink_guard = context.sink_lock.write().unwrap();
            *sink_guard = rodio::Sink::new(&context.device);
        }
        Some(thread::spawn(move || loop {
            let url;
            {
                let queue_guard = context.queue_lock.read().unwrap();
                url = queue_guard
                    .first()
                    .map(|first| format!("{}/{}/{}.mp3", context.server_url, STATIC, first));
            }
            if let Some(url) = url {
                let source =
                    rodio::Decoder::new(HttpStreamReader::new(url, context.api_key.to_string()))
                        .unwrap();
                {
                    let sink_guard = context.sink_lock.read().unwrap();
                    sink_guard.append(source);
                    sink_guard.sleep_until_end();
                }
                {
                    let mut queue_guard = context.queue_lock.write().unwrap();
                    if !queue_guard.is_empty() {
                        queue_guard.rotate_left(1);
                    }
                }
            } else {
                break;
            }
        }))
    };
    JOIN_HANDLE_MUTEX.with(|join_handle_mutex| {
        let mut join_handle_guard = join_handle_mutex.lock().unwrap();
        *join_handle_guard = join_handle;
    });
}

struct Context {
    server_url: String,
    api_key: String,
    client: reqwest::blocking::Client,
    device: rodio::Device,
    sink_lock: RwLock<rodio::Sink>,
    queue_lock: RwLock<Vec<i64>>,
}

fn main() -> Result<(), failure::Error> {
    let matches = clap::App::new("piTunes client")
        .version("0.1.0")
        .about("A client that allows you to browse and play songs from your personal music collection hosted by a piTunes server")
        .author("Bernhard Fritz <bernhard.e.fritz@gmail.com>")
        .arg(
            clap::Arg::with_name("server-url")
                .help("piTunes server to connect to")
                .required(true)
                .index(1)
        )
        .get_matches();
    let server_url = value_t!(matches, "server-url", String).unwrap();
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("Environment variable API_KEY is not present");

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout); // TODO: consider crossterm https://docs.rs/tui/0.8.0/tui/index.html#adding-tui-as-a-dependency
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let client = reqwest::blocking::Client::new();
    let device = rodio::default_output_device().unwrap();
    let sink_lock = RwLock::new(rodio::Sink::new_idle().0);
    let queue_lock = RwLock::new(vec![]);

    let root_title = format!("{} @ {}", PI_SYMBOL, server_url);

    let mut store = {
        let initial_state = {
            let context = Arc::new(Context {
                server_url,
                api_key,
                client,
                device,
                sink_lock,
                queue_lock,
            });
            let break_condition = false;
            let model = Model::Root;
            let view = {
                let list_state = {
                    let mut list_state = ListState::default();
                    list_state.select(Some(0));
                    list_state
                };
                let items = vec![
                    String::from(ALBUMS),
                    String::from(ARTISTS),
                    String::from(GENRES),
                    String::from(PLAYLISTS),
                    String::from(TRACKS),
                ];
                View::List { list_state, items }
            };
            let history = Vec::new();
            let add_to_history = true;
            State {
                context,
                break_condition,
                model,
                view,
                history,
                add_to_history,
            }
        };
        Store::new(REDUCER, initial_state)
    };

    loop {
        let state = store.state();

        if state.break_condition {
            break;
        }

        let active = if let Model::Tracks { tracks } = &state.model {
            let queue_guard = state.context.queue_lock.read().unwrap();
            if let Some(first) = queue_guard.first() {
                tracks.iter().position(|track| track.id == *first)
            } else {
                None
            }
        } else {
            None
        };

        let title = {
            let mut title = String::from(" ");
            title.push_str(&root_title[..]);
            for state in &state.history {
                if let View::List { list_state, items } = &state.view {
                    if let Some(selected) = list_state.selected() {
                        title.push_str(" / ");
                        title.push_str(&items[selected][..]);
                    }
                }
            }
            title.push_str(" ");
            title
        };

        terminal.draw(|mut f| {
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(&title[..]);
            f.render_widget(block, size);
            match &state.view {
                View::List { list_state, items } => {
                    let chunks = Layout::default()
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .horizontal_margin(3)
                        .vertical_margin(2)
                        .split(f.size());
                    let highlight_modifier = if let Some(selected) = list_state.selected() {
                        if let Some(active) = active {
                            if selected == active {
                                Modifier::REVERSED | Modifier::BOLD
                            } else {
                                Modifier::REVERSED
                            }
                        } else {
                            Modifier::REVERSED
                        }
                    } else {
                        Modifier::REVERSED
                    };
                    let list = List::new(items.iter().enumerate().map(|(i, item)| {
                        if let Some(active) = active {
                            if active == i {
                                Text::styled(item, Style::default().modifier(Modifier::BOLD))
                            } else {
                                Text::raw(item)
                            }
                        } else {
                            Text::raw(item)
                        }
                    }))
                    .highlight_style(Style::default().modifier(highlight_modifier));
                    f.render_stateful_widget(list, chunks[0], &mut list_state.clone());
                }
                View::Edit { input_field } => {
                    let chunks = Layout::default()
                        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                        .horizontal_margin(3)
                        .vertical_margin(2)
                        .split(f.size());
                    let text = [Text::raw(input_field)];
                    let paragraph = Paragraph::new(text.iter()).block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("Label")
                            .title_style(Style::default().modifier(Modifier::BOLD)), // TODO: current block should have a bold title to indicate "focus"
                    );
                    f.render_widget(paragraph, chunks[0]);
                }
            }
        })?;

        match &state.view {
            View::List {
                list_state: _,
                items: _,
            } => terminal.hide_cursor()?,
            View::Edit { input_field } => {
                terminal.show_cursor()?;
                // Put the cursor back inside the input box
                write!(
                    terminal.backend_mut(),
                    "{}",
                    Goto(5 + UnicodeWidthStr::width(&input_field[..]) as u16, 4)
                )?;
                // stdout is buffered, flush it to see the effect immediately when hitting backspace
                io::stdout().flush().ok();
            }
        }

        if let Event::Input(input) = events.next()? {
            store.dispatch(input);
        }
    }

    terminal.clear()?;

    Ok(())
}
