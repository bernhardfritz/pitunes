#[macro_use]
extern crate clap;

#[allow(dead_code)]
mod event;

use graphql_client::{GraphQLQuery, Response};
use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::style::{Modifier, Style};
use tui::widgets::{SelectableList, Widget};
use tui::Terminal;

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
    fn from(album: artist_albums_query::ArtistAlbumsQueryArtistAlbums) -> albums_query::AlbumsQueryAlbums {
        albums_query::AlbumsQueryAlbums {
            id: album.id,
            name: album.name,
        }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracks> for tracks_query::TracksQueryTracks {
    fn from(track: artist_tracks_query::ArtistTracksQueryArtistTracks) -> tracks_query::TracksQueryTracks {
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

use crate::event::{Event, Events};

struct App<T> {
    state: T,
    items: Vec<String>, // todo static lifetime
    selected: Option<usize>,
    client: reqwest::blocking::Client,
    graphql_endpoint: String,
}

struct ClientlessApp<T> {
    state: T,
    items: Vec<String>, // todo static lifetime
    selected: Option<usize>,
}

struct RootView;
struct AlbumsView {
    parent: ClientlessApp<RootView>,
    albums: Vec<albums_query::AlbumsQueryAlbums>,
}
struct ArtistView {
    parent: ClientlessApp<ArtistsView>,
    albums: Vec<albums_query::AlbumsQueryAlbums>,
}
struct ArtistsView {
    parent: ClientlessApp<RootView>,
    artists: Vec<artists_query::ArtistsQueryArtists>,
}
struct GenresView {
    parent: ClientlessApp<RootView>,
    genres: Vec<genres_query::GenresQueryGenres>,
}
struct TracksView {
    parent: TracksViewParent,
    tracks: Vec<tracks_query::TracksQueryTracks>,
}
enum TracksViewParent {
    RootView(ClientlessApp<RootView>),
    AlbumsView(ClientlessApp<AlbumsView>),
    ArtistView(ClientlessApp<ArtistView>),
    GenresView(ClientlessApp<GenresView>),
}

impl App<RootView> {
    fn new(graphql_endpoint: String) -> Self {
        App {
            state: RootView {},
            items: vec![
                String::from("Albums"),
                String::from("Artists"),
                String::from("Genres"),
                String::from("Tracks"),
            ],
            selected: Some(0),
            client: reqwest::blocking::Client::new(),
            graphql_endpoint,
        }
    }
}

impl From<App<RootView>> for App<AlbumsView> {
    fn from(app: App<RootView>) -> App<AlbumsView> {
        let request_body = AlbumsQuery::build_query(albums_query::Variables {});
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<albums_query::ResponseData> = res.json().unwrap();
        let albums = response_body.data.map(|data| data.albums).unwrap();
        let items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
        App {
            state: AlbumsView {
                parent: ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                },
                albums,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

impl From<App<AlbumsView>> for App<TracksView> {
    fn from(app: App<AlbumsView>) -> App<TracksView> {
        let album = &app.state.albums[app.selected.unwrap()];
        let request_body = AlbumQuery::build_query(album_query::Variables { id: album.id });
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
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
        App {
            state: TracksView {
                parent: TracksViewParent::AlbumsView(ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                }),
                tracks,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

impl From<App<RootView>> for App<ArtistsView> {
    fn from(app: App<RootView>) -> App<ArtistsView> {
        let request_body = ArtistsQuery::build_query(artists_query::Variables {});
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<artists_query::ResponseData> = res.json().unwrap();
        let artists = response_body.data.map(|data| data.artists).unwrap();
        let items: Vec<String> = artists.iter().map(|artist| artist.name.clone()).collect();
        App {
            state: ArtistsView {
                parent: ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                },
                artists,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

impl From<App<ArtistView>> for App<TracksView> {
    fn from(app: App<ArtistView>) -> App<TracksView> {
        let selected = app.selected.unwrap();
        if selected == 0 {
            let artist = &app.state.parent.state.artists[app.state.parent.selected.unwrap()];
            let request_body = ArtistTracksQuery::build_query(artist_tracks_query::Variables { id: artist.id });
            let res = app
                .client
                .post(&app.graphql_endpoint[..])
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
            App {
                state: TracksView {
                    parent: TracksViewParent::ArtistView(ClientlessApp {
                        state: app.state,
                        items: app.items,
                        selected: app.selected,
                    }),
                    tracks,
                },
                items,
                selected: Some(0), // TODO: could there potentially be None items? add check to be safe
                client: app.client,
                graphql_endpoint: app.graphql_endpoint,
            }
        } else {
            let album = &app.state.albums[app.selected.unwrap() - 1]; // -1 due to "All tracks" item
            let request_body = AlbumQuery::build_query(album_query::Variables { id: album.id });
            let res = app
                .client
                .post(&app.graphql_endpoint[..])
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
            App {
                state: TracksView {
                    parent: TracksViewParent::ArtistView(ClientlessApp {
                        state: app.state,
                        items: app.items,
                        selected: app.selected,
                    }),
                    tracks,
                },
                items,
                selected: Some(0), // TODO: could there potentially be None items? add check to be safe
                client: app.client,
                graphql_endpoint: app.graphql_endpoint,
            }
        }
    }
}

impl From<App<ArtistsView>> for App<ArtistView> {
    fn from(app: App<ArtistsView>) -> App<ArtistView> {
        let artist = &app.state.artists[app.selected.unwrap()];
        let request_body = ArtistAlbumsQuery::build_query(artist_albums_query::Variables { id: artist.id });
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<artist_albums_query::ResponseData> = res.json().unwrap();
        let albums = response_body
            .data
            .map(|data| data.artist)
            .map(|artist| artist.albums)
            .unwrap();
        let albums: Vec<albums_query::AlbumsQueryAlbums> =
            albums.into_iter().map(|album| album.into()).collect();
        let mut items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
        items.insert(0, String::from("All tracks"));
        App {
            state: ArtistView {
                parent: ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                },
                albums,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

impl From<App<RootView>> for App<GenresView> {
    fn from(app: App<RootView>) -> App<GenresView> {
        let request_body = GenresQuery::build_query(genres_query::Variables {});
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<genres_query::ResponseData> = res.json().unwrap();
        let genres = response_body.data.map(|data| data.genres).unwrap();
        let items: Vec<String> = genres.iter().map(|genre| genre.name.clone()).collect();
        App {
            state: GenresView {
                parent: ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                },
                genres,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

impl From<App<GenresView>> for App<TracksView> {
    fn from(app: App<GenresView>) -> App<TracksView> {
        let genre = &app.state.genres[app.selected.unwrap()];
        let request_body = GenreQuery::build_query(genre_query::Variables { id: genre.id });
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
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
        App {
            state: TracksView {
                parent: TracksViewParent::GenresView(ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                }),
                tracks,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

impl From<App<RootView>> for App<TracksView> {
    fn from(app: App<RootView>) -> App<TracksView> {
        let request_body = TracksQuery::build_query(tracks_query::Variables {});
        let res = app
            .client
            .post(&app.graphql_endpoint[..])
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<tracks_query::ResponseData> = res.json().unwrap();
        let tracks = response_body.data.map(|data| data.tracks).unwrap();
        let items: Vec<String> = tracks
            .iter()
            .map(|track| track.name.clone()) // TODO maybe reference
            .collect();
        App {
            state: TracksView {
                parent: TracksViewParent::RootView(ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                }),
                tracks,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
            graphql_endpoint: app.graphql_endpoint,
        }
    }
}

enum AppWrapper {
    RootView(App<RootView>),
    AlbumsView(App<AlbumsView>),
    ArtistView(App<ArtistView>),
    ArtistsView(App<ArtistsView>),
    GenresView(App<GenresView>),
    TracksView(App<TracksView>),
}

impl AppWrapper {
    fn forward(self) -> Self {
        match self {
            AppWrapper::RootView(app) => {
                if let Some(selected) = app.selected {
                    if app.items[selected] == "Albums" {
                        AppWrapper::AlbumsView(app.into())
                    } else if app.items[selected] == "Artists" {
                        AppWrapper::ArtistsView(app.into())
                    } else if app.items[selected] == "Genres" {
                        AppWrapper::GenresView(app.into())
                    } else if app.items[selected] == "Tracks" {
                        AppWrapper::TracksView(app.into())
                    } else {
                        AppWrapper::RootView(app)
                    }
                } else {
                    AppWrapper::RootView(app)
                }
            }
            AppWrapper::AlbumsView(app) => AppWrapper::TracksView(app.into()),
            AppWrapper::ArtistView(app) => AppWrapper::TracksView(app.into()),
            AppWrapper::ArtistsView(app) => AppWrapper::ArtistView(app.into()),
            AppWrapper::GenresView(app) => AppWrapper::GenresView(app),
            AppWrapper::TracksView(app) => AppWrapper::TracksView(app),
        }
    }

    fn backward(self) -> Self {
        match self {
            AppWrapper::RootView(app) => AppWrapper::RootView(app),
            AppWrapper::AlbumsView(app) => AppWrapper::RootView(App {
                state: app.state.parent.state,
                items: app.state.parent.items,
                selected: app.state.parent.selected,
                client: app.client,
                graphql_endpoint: app.graphql_endpoint,
            }),
            AppWrapper::ArtistView(app) => AppWrapper::ArtistsView(App {
                state: app.state.parent.state,
                items: app.state.parent.items,
                selected: app.state.parent.selected,
                client: app.client,
                graphql_endpoint: app.graphql_endpoint,
            }),
            AppWrapper::ArtistsView(app) => AppWrapper::RootView(App {
                state: app.state.parent.state,
                items: app.state.parent.items,
                selected: app.state.parent.selected,
                client: app.client,
                graphql_endpoint: app.graphql_endpoint,
            }),
            AppWrapper::GenresView(app) => AppWrapper::RootView(App {
                state: app.state.parent.state,
                items: app.state.parent.items,
                selected: app.state.parent.selected,
                client: app.client,
                graphql_endpoint: app.graphql_endpoint,
            }),
            AppWrapper::TracksView(app) => match app.state.parent {
                TracksViewParent::RootView(clientless_app) => AppWrapper::RootView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                    graphql_endpoint: app.graphql_endpoint,
                }),
                TracksViewParent::AlbumsView(clientless_app) => AppWrapper::AlbumsView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                    graphql_endpoint: app.graphql_endpoint,
                }),
                TracksViewParent::ArtistView(clientless_app) => AppWrapper::ArtistView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                    graphql_endpoint: app.graphql_endpoint,
                }),
                TracksViewParent::GenresView(clientless_app) => AppWrapper::GenresView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                    graphql_endpoint: app.graphql_endpoint,
                }),
            },
        }
    }

    fn get_items(&self) -> &Vec<String> {
        match self {
            AppWrapper::RootView(app) => &app.items,
            AppWrapper::AlbumsView(app) => &app.items,
            AppWrapper::ArtistView(app) => &app.items,
            AppWrapper::ArtistsView(app) => &app.items,
            AppWrapper::GenresView(app) => &app.items,
            AppWrapper::TracksView(app) => &app.items,
        }
    }

    fn get_selected(&self) -> Option<usize> {
        match self {
            AppWrapper::RootView(app) => app.selected,
            AppWrapper::AlbumsView(app) => app.selected,
            AppWrapper::ArtistView(app) => app.selected,
            AppWrapper::ArtistsView(app) => app.selected,
            AppWrapper::GenresView(app) => app.selected,
            AppWrapper::TracksView(app) => app.selected,
        }
    }

    fn set_selected(&mut self, selected: Option<usize>) {
        match self {
            AppWrapper::RootView(app) => app.selected = selected,
            AppWrapper::AlbumsView(app) => app.selected = selected,
            AppWrapper::ArtistView(app) => app.selected = selected,
            AppWrapper::ArtistsView(app) => app.selected = selected,
            AppWrapper::GenresView(app) => app.selected = selected,
            AppWrapper::TracksView(app) => app.selected = selected,
        }
    }
}

fn main() -> Result<(), failure::Error> {
    let matches = clap::App::new("piTunes client")
        .version("0.1.0")
        .about("A client that allows you to browse and play songs from your personal music collection hosted by a piTunes server")
        .author("Bernhard Fritz <bernhard.e.fritz@gmail.com>")
        .arg(
            clap::Arg::with_name("SERVER")
                .help("piTunes server to connect to")
                .required(true)
                .index(1)
        )
        .get_matches();
    let mut graphql_endpoint = value_t!(matches, "SERVER", String).unwrap();
    graphql_endpoint.push_str("/graphql");

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout); // TODO: consider crossterm https://docs.rs/tui/0.8.0/tui/index.html#adding-tui-as-a-dependency
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app_wrapper = AppWrapper::RootView(App::new(graphql_endpoint));
    let highlight_style = Style::default().modifier(Modifier::BOLD);
    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            SelectableList::default()
                .items(app_wrapper.get_items())
                .select(app_wrapper.get_selected())
                .highlight_style(highlight_style)
                .render(&mut f, size)
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Backspace => {
                    app_wrapper = app_wrapper.backward();
                }
                Key::Up => {
                    app_wrapper.set_selected(if let Some(selected) = app_wrapper.get_selected() {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(app_wrapper.get_items().len() - 1)
                        }
                    } else {
                        Some(0)
                    })
                }
                Key::Down => {
                    app_wrapper.set_selected(if let Some(selected) = app_wrapper.get_selected() {
                        if selected >= app_wrapper.get_items().len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    })
                }
                Key::Char('\n') => {
                    app_wrapper = app_wrapper.forward();
                }
                Key::Char('q') => {
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
