#[macro_use]
extern crate clap;

#[allow(dead_code)]
mod event;

mod http_stream_reader;
use http_stream_reader::HttpStreamReader;

#[allow(dead_code)]
mod selectable_list_2;
use selectable_list_2::SelectableList2;

use dotenv::dotenv;
use graphql_client::{GraphQLQuery, Response};
use std::env;
use std::io;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

const GRAPHQL: &str = "graphql";
const STATIC: &str = "static";

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

impl From<album_query::AlbumQueryAlbumTracks> for tracks_query::TracksQueryTracks {
    fn from(track: album_query::AlbumQueryAlbumTracks) -> tracks_query::TracksQueryTracks {
        tracks_query::TracksQueryTracks {
            id: track.id,
            name: track.name,
        }
    }
}

impl From<artist_albums_query::ArtistAlbumsQueryArtistAlbums> for albums_query::AlbumsQueryAlbums {
    fn from(
        album: artist_albums_query::ArtistAlbumsQueryArtistAlbums,
    ) -> albums_query::AlbumsQueryAlbums {
        albums_query::AlbumsQueryAlbums {
            id: album.id,
            name: album.name,
        }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracks> for tracks_query::TracksQueryTracks {
    fn from(
        track: artist_tracks_query::ArtistTracksQueryArtistTracks,
    ) -> tracks_query::TracksQueryTracks {
        tracks_query::TracksQueryTracks {
            id: track.id,
            name: track.name,
        }
    }
}

impl From<genre_query::GenreQueryGenreTracks> for tracks_query::TracksQueryTracks {
    fn from(track: genre_query::GenreQueryGenreTracks) -> tracks_query::TracksQueryTracks {
        tracks_query::TracksQueryTracks {
            id: track.id,
            name: track.name,
        }
    }
}

impl From<playlist_query::PlaylistQueryPlaylistTracks> for tracks_query::TracksQueryTracks {
    fn from(track: playlist_query::PlaylistQueryPlaylistTracks) -> tracks_query::TracksQueryTracks {
        tracks_query::TracksQueryTracks {
            id: track.id,
            name: track.name,
        }
    }
}

use crate::event::{Event, Events};

enum State {
    Albums {
        albums: Vec<albums_query::AlbumsQueryAlbums>,
    },
    Artist {
        artist_id: i64,
        albums: Vec<albums_query::AlbumsQueryAlbums>,
    },
    Artists {
        artists: Vec<artists_query::ArtistsQueryArtists>,
    },
    Genres {
        genres: Vec<genres_query::GenresQueryGenres>,
    },
    Playlists {
        playlists: Vec<playlists_query::PlaylistsQueryPlaylists>,
    },
    Root,
    Tracks {
        tracks: Vec<tracks_query::TracksQueryTracks>,
    },
}

struct App {
    state: State,
    items: Vec<String>,
    selected: Option<usize>,
}

fn get_albums(context: &Arc<Context>) -> App {
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
    let items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Albums { albums },
        items,
        selected,
    }
}

fn get_artists(context: &Arc<Context>) -> App {
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
    let items: Vec<String> = artists.iter().map(|artist| artist.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Artists { artists },
        items,
        selected,
    }
}

fn get_genres(context: &Arc<Context>) -> App {
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
    let items: Vec<String> = genres.iter().map(|genre| genre.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Genres { genres },
        items,
        selected,
    }
}

fn get_playlists(context: &Arc<Context>) -> App {
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
    let items: Vec<String> = playlists
        .iter()
        .map(|playlist| playlist.name.clone())
        .collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Playlists { playlists },
        items,
        selected,
    }
}

fn get_tracks(context: &Arc<Context>) -> App {
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
    let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Tracks { tracks },
        items,
        selected,
    }
}

fn get_tracks_of_album(context: &Arc<Context>, album: &albums_query::AlbumsQueryAlbums) -> App {
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
    let tracks: Vec<tracks_query::TracksQueryTracks> =
        tracks.into_iter().map(|track| track.into()).collect();
    let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Tracks { tracks },
        items,
        selected,
    }
}

fn get_tracks_of_artist(context: &Arc<Context>, artist_id: i64) -> App {
    let url = format!("{}/{}", context.server_url, GRAPHQL);
    let request_body =
        ArtistTracksQuery::build_query(artist_tracks_query::Variables { id: artist_id });
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
    let tracks: Vec<tracks_query::TracksQueryTracks> =
        tracks.into_iter().map(|track| track.into()).collect();
    let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Tracks { tracks },
        items,
        selected,
    }
}

fn get_artist(context: &Arc<Context>, artist: &artists_query::ArtistsQueryArtists) -> App {
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
    let artist = response_body.data.map(|data| data.artist).unwrap();
    let artist_id = artist.id;
    let albums: Vec<albums_query::AlbumsQueryAlbums> = artist
        .albums
        .into_iter()
        .map(|album| album.into())
        .collect();
    let mut items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
    items.insert(0, String::from("All tracks"));
    App {
        state: State::Artist { artist_id, albums },
        items,
        selected: Some(0),
    }
}

fn get_tracks_of_genre(context: &Arc<Context>, genre: &genres_query::GenresQueryGenres) -> App {
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
    let tracks: Vec<tracks_query::TracksQueryTracks> =
        tracks.into_iter().map(|track| track.into()).collect();
    let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Tracks { tracks },
        items,
        selected,
    }
}

fn get_tracks_of_playlist(
    context: &Arc<Context>,
    playlist: &playlists_query::PlaylistsQueryPlaylists,
) -> App {
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
    let tracks: Vec<tracks_query::TracksQueryTracks> =
        tracks.into_iter().map(|track| track.into()).collect();
    let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
    let selected = if items.is_empty() { None } else { Some(0) };
    App {
        state: State::Tracks { tracks },
        items,
        selected,
    }
}

fn play_queue(
    context: Arc<Context>,
    queue: Vec<i64>,
    join_handle: Option<JoinHandle<()>>,
) -> Option<JoinHandle<()>> {
    {
        let mut queue_guard = context.queue_lock.write().unwrap();
        queue_guard.clear();
    }
    {
        let sink_guard = context.sink_lock.read().unwrap();
        sink_guard.stop();
    }
    if let Some(join_handle) = join_handle {
        join_handle.join().unwrap();
    }
    if queue.is_empty() {
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
    }
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
    terminal.hide_cursor()?;
    terminal.clear().unwrap();

    let highlight_style = Style::default().modifier(Modifier::BOLD);
    let events = Events::new();

    let client = reqwest::blocking::Client::new();
    let device = rodio::default_output_device().unwrap();
    let sink_lock = RwLock::new(rodio::Sink::new_idle().0);
    let queue_lock = RwLock::new(vec![]);

    let context = Arc::new(Context {
        server_url,
        api_key,
        client,
        device,
        sink_lock,
        queue_lock,
    });

    let mut join_handle: Option<JoinHandle<()>> = None;

    let mut stack = Vec::new();
    stack.push(App {
        state: State::Root,
        items: vec![
            String::from("Albums"),
            String::from("Artists"),
            String::from("Genres"),
            String::from("Playlists"),
            String::from("Tracks"),
        ],
        selected: Some(0),
    });

    loop {
        if let Some(last) = stack.last() {
            let active = if let State::Tracks { tracks } = &last.state {
                let queue_guard = context.queue_lock.read().unwrap();
                if let Some(first) = queue_guard.first() {
                    tracks.iter().position(|track| track.id == *first)
                } else {
                    None
                }
            } else {
                None
            };

            terminal.draw(|mut f| {
                let size = f.size();
                Block::default()
                    .borders(Borders::ALL)
                    .title(" π ")
                    .render(&mut f, size);
                let chunks = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(2)
                    .split(f.size());
                SelectableList2::default()
                    .items(&last.items)
                    .select(last.selected)
                    .highlight_symbol(">")
                    .active(active)
                    .active_style(highlight_style)
                    .render(&mut f, chunks[0]);
            })?;
        }

        match events.next()? {
            Event::Input(input) => match input {
                Key::Esc => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                }
                Key::Up => {
                    if let Some(last) = stack.last_mut() {
                        last.selected = if let Some(selected) = last.selected {
                            if selected > 0 {
                                Some(selected - 1)
                            } else {
                                Some(last.items.len() - 1)
                            }
                        } else {
                            Some(0)
                        }
                    }
                }
                Key::Down => {
                    if let Some(last) = stack.last_mut() {
                        last.selected = if let Some(selected) = last.selected {
                            if selected >= last.items.len() - 1 {
                                Some(0)
                            } else {
                                Some(selected + 1)
                            }
                        } else {
                            Some(0)
                        }
                    }
                }
                Key::Char('\n') => {
                    let app = if let Some(last) = stack.last() {
                        match &last.state {
                            State::Albums { albums } => {
                                if let Some(selected) = last.selected {
                                    Some(get_tracks_of_album(&context, &albums[selected]))
                                } else {
                                    None
                                }
                            }
                            State::Artist { artist_id, albums } => {
                                if let Some(selected) = last.selected {
                                    if selected == 0 {
                                        // All tracks
                                        Some(get_tracks_of_artist(&context, *artist_id))
                                    } else {
                                        Some(get_tracks_of_album(&context, &albums[selected - 1]))
                                    }
                                } else {
                                    None
                                }
                            }
                            State::Artists { artists } => {
                                if let Some(selected) = last.selected {
                                    Some(get_artist(&context, &artists[selected]))
                                } else {
                                    None
                                }
                            }
                            State::Genres { genres } => {
                                if let Some(selected) = last.selected {
                                    Some(get_tracks_of_genre(&context, &genres[selected]))
                                } else {
                                    None
                                }
                            }
                            State::Playlists { playlists } => {
                                if let Some(selected) = last.selected {
                                    Some(get_tracks_of_playlist(&context, &playlists[selected]))
                                } else {
                                    None
                                }
                            }
                            State::Root => {
                                if let Some(selected) = last.selected {
                                    match &last.items[selected][..] {
                                        "Albums" => Some(get_albums(&context)),
                                        "Artists" => Some(get_artists(&context)),
                                        "Genres" => Some(get_genres(&context)),
                                        "Playlists" => Some(get_playlists(&context)),
                                        "Tracks" => Some(get_tracks(&context)),
                                        _ => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            State::Tracks { tracks } => {
                                if let Some(selected) = last.selected {
                                    let mut queue: Vec<i64> =
                                        tracks.iter().map(|track| track.id).collect();
                                    queue.rotate_left(selected);
                                    join_handle = play_queue(context.clone(), queue, join_handle);
                                }
                                None
                            }
                        }
                    } else {
                        None
                    };
                    if let Some(app) = app {
                        stack.push(app);
                    }
                }
                Key::Char(' ') => {
                    let sink_guard = context.sink_lock.read().unwrap();
                    if sink_guard.is_paused() {
                        sink_guard.play();
                    } else {
                        sink_guard.pause();
                    }
                }
                Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    terminal.clear().unwrap();

    Ok(())
}
