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
    query_path = "src/graphql/artist_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistQuery;

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

impl From<artist_query::ArtistQueryArtistAlbums> for albums_query::AlbumsQueryAlbums {
    fn from(album: artist_query::ArtistQueryArtistAlbums) -> albums_query::AlbumsQueryAlbums {
        albums_query::AlbumsQueryAlbums {
            id: album.id,
            name: album.name,
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
}

struct ClientlessApp<T> {
    state: T,
    items: Vec<String>, // todo static lifetime
    selected: Option<usize>,
}

struct RootView;
struct AlbumsView {
    parent: AlbumsViewParent,
    albums: Vec<albums_query::AlbumsQueryAlbums>,
}
enum AlbumsViewParent {
    RootView(ClientlessApp<RootView>),
    ArtistsView(ClientlessApp<ArtistsView>),
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
    GenresView(ClientlessApp<GenresView>),
}

impl App<RootView> {
    fn new() -> Self {
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
        }
    }
}

impl From<App<RootView>> for App<AlbumsView> {
    fn from(app: App<RootView>) -> App<AlbumsView> {
        let request_body = AlbumsQuery::build_query(albums_query::Variables {});
        let res = app
            .client
            .post("http://localhost:8080/graphql")
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<albums_query::ResponseData> = res.json().unwrap();
        let albums = response_body.data.map(|data| data.albums).unwrap();
        let items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
        App {
            state: AlbumsView {
                parent: AlbumsViewParent::RootView(ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                }),
                albums,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
        }
    }
}

impl From<App<AlbumsView>> for App<TracksView> {
    fn from(app: App<AlbumsView>) -> App<TracksView> {
        let album = &app.state.albums[app.selected.unwrap()];
        let request_body = AlbumQuery::build_query(album_query::Variables { id: album.id });
        let res = app
            .client
            .post("http://localhost:8080/graphql")
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
        }
    }
}

impl From<App<RootView>> for App<ArtistsView> {
    fn from(app: App<RootView>) -> App<ArtistsView> {
        let request_body = ArtistsQuery::build_query(artists_query::Variables {});
        let res = app
            .client
            .post("http://localhost:8080/graphql")
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
        }
    }
}

impl From<App<ArtistsView>> for App<AlbumsView> {
    fn from(app: App<ArtistsView>) -> App<AlbumsView> {
        let artist = &app.state.artists[app.selected.unwrap()];
        let request_body = ArtistQuery::build_query(artist_query::Variables { id: artist.id });
        let res = app
            .client
            .post("http://localhost:8080/graphql")
            .json(&request_body)
            .send()
            .unwrap();
        let response_body: Response<artist_query::ResponseData> = res.json().unwrap();
        let albums = response_body
            .data
            .map(|data| data.artist)
            .map(|artist| artist.albums)
            .unwrap();
        let albums: Vec<albums_query::AlbumsQueryAlbums> =
            albums.into_iter().map(|album| album.into()).collect();
        let items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
        App {
            state: AlbumsView {
                parent: AlbumsViewParent::ArtistsView(ClientlessApp {
                    state: app.state,
                    items: app.items,
                    selected: app.selected,
                }),
                albums,
            },
            items,
            selected: Some(0), // TODO: could there potentially be None items? add check to be safe
            client: app.client,
        }
    }
}

impl From<App<RootView>> for App<GenresView> {
    fn from(app: App<RootView>) -> App<GenresView> {
        let request_body = GenresQuery::build_query(genres_query::Variables {});
        let res = app
            .client
            .post("http://localhost:8080/graphql")
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
        }
    }
}

impl From<App<GenresView>> for App<TracksView> {
    fn from(app: App<GenresView>) -> App<TracksView> {
        let genre = &app.state.genres[app.selected.unwrap()];
        let request_body = GenreQuery::build_query(genre_query::Variables { id: genre.id });
        let res = app
            .client
            .post("http://localhost:8080/graphql")
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
        }
    }
}

impl From<App<RootView>> for App<TracksView> {
    fn from(app: App<RootView>) -> App<TracksView> {
        let request_body = TracksQuery::build_query(tracks_query::Variables {});
        let res = app
            .client
            .post("http://localhost:8080/graphql")
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
        }
    }
}

enum AppWrapper {
    RootView(App<RootView>),
    AlbumsView(App<AlbumsView>),
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
            AppWrapper::ArtistsView(app) => AppWrapper::AlbumsView(app.into()),
            AppWrapper::GenresView(app) => AppWrapper::GenresView(app),
            AppWrapper::TracksView(app) => AppWrapper::TracksView(app),
        }
    }

    fn backward(self) -> Self {
        match self {
            AppWrapper::RootView(app) => AppWrapper::RootView(app),
            AppWrapper::AlbumsView(app) => {
                match app.state.parent {
                    AlbumsViewParent::RootView(clientless_app) => AppWrapper::RootView(App {
                        state: clientless_app.state,
                        items: clientless_app.items,
                        selected: clientless_app.selected,
                        client: app.client,
                    }),
                    AlbumsViewParent::ArtistsView(clientless_app) => AppWrapper::ArtistsView(App {
                        state: clientless_app.state,
                        items: clientless_app.items,
                        selected: clientless_app.selected,
                        client: app.client,
                    }),
                } // TODO remove impl From<_> for App<RootView> and do same for others
            }
            AppWrapper::ArtistsView(app) => AppWrapper::RootView(App {
                state: app.state.parent.state,
                items: app.state.parent.items,
                selected: app.state.parent.selected,
                client: app.client,
            }),
            AppWrapper::GenresView(app) => AppWrapper::RootView(App {
                state: app.state.parent.state,
                items: app.state.parent.items,
                selected: app.state.parent.selected,
                client: app.client,
            }),
            AppWrapper::TracksView(app) => match app.state.parent {
                TracksViewParent::RootView(clientless_app) => AppWrapper::RootView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                }),
                TracksViewParent::AlbumsView(clientless_app) => AppWrapper::AlbumsView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                }),
                TracksViewParent::GenresView(clientless_app) => AppWrapper::GenresView(App {
                    state: clientless_app.state,
                    items: clientless_app.items,
                    selected: clientless_app.selected,
                    client: app.client,
                }),
            },
        }
    }

    fn get_items(&self) -> &Vec<String> {
        match self {
            AppWrapper::RootView(app) => &app.items,
            AppWrapper::AlbumsView(app) => &app.items,
            AppWrapper::ArtistsView(app) => &app.items,
            AppWrapper::GenresView(app) => &app.items,
            AppWrapper::TracksView(app) => &app.items,
        }
    }

    fn get_selected(&self) -> Option<usize> {
        match self {
            AppWrapper::RootView(app) => app.selected,
            AppWrapper::AlbumsView(app) => app.selected,
            AppWrapper::ArtistsView(app) => app.selected,
            AppWrapper::GenresView(app) => app.selected,
            AppWrapper::TracksView(app) => app.selected,
        }
    }

    fn set_selected(&mut self, selected: Option<usize>) {
        match self {
            AppWrapper::RootView(app) => app.selected = selected,
            AppWrapper::AlbumsView(app) => app.selected = selected,
            AppWrapper::ArtistsView(app) => app.selected = selected,
            AppWrapper::GenresView(app) => app.selected = selected,
            AppWrapper::TracksView(app) => app.selected = selected,
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout); // TODO: consider crossterm https://docs.rs/tui/0.8.0/tui/index.html#adding-tui-as-a-dependency
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app_wrapper = AppWrapper::RootView(App::new());
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
