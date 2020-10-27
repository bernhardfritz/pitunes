mod constants;
#[allow(dead_code)]
mod util;
// mod http_stream_reader;
mod models;
mod requests;
mod state_machine;

use std::{
    cmp, env,
    io::{self, Cursor, Stdout, Write},
    sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use clap::{self, value_t};
use constants::{ALBUMS, ARTISTS, GENRES, PLAYLISTS, STATIC, TRACKS};
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use failure::Error;
// use http_stream_reader::HttpStreamReader;
use models::{Album, Artist, Genre, IdName, Playlist, Track};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use util::stateful_list::StatefulList;

use crate::{
    state_machine::StateMachine,
    util::{
        event::{Event, Events},
        my_gauge::MyGauge,
    },
};

pub struct Context {
    server_url: String,
    api_key: String,
    client: reqwest::blocking::Client,
    handle: rodio::OutputStreamHandle,
    sink_lock: RwLock<rodio::Sink>,
    queue_lock: RwLock<Vec<Track>>,
    play_instant_lock: RwLock<Option<Instant>>,
    lazy_elapsed_lock: RwLock<Duration>,
}

// Look away, I'm hideous!
pub fn play_queue(context: Arc<Context>, queue: Vec<Track>) {
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
            *sink_guard = rodio::Sink::try_new(&context.handle).unwrap();
        }
        Some(thread::spawn(move || loop {
            let url;
            {
                let queue_guard = context.queue_lock.read().unwrap();
                url = queue_guard
                    .first()
                    .map(|track| format!("{}/{}/{}.mp3", context.server_url, STATIC, track.id));
            }
            if let Some(url) = url {
                // TODO: HttpStreamReader should not be passed directly to the Decoder as this results in audible delays while chunks are downloaded
                /*let source =
                rodio::Decoder::new(HttpStreamReader::new(url, context.api_key.to_string()))
                    .unwrap();*/
                // download full track until issue with partial downloads is resolved
                let cursor = {
                    let mut res = context
                        .client
                        .get(&url)
                        .bearer_auth(&context.api_key[..])
                        .send()
                        .unwrap();
                    let mut buf = vec![];
                    std::io::copy(&mut res, &mut buf).unwrap();
                    Cursor::new(buf)
                };
                let source = rodio::Decoder::new(cursor).unwrap();
                {
                    let sink_guard = context.sink_lock.read().unwrap();
                    sink_guard.append(source);
                    {
                        let mut play_instant_guard = context.play_instant_lock.write().unwrap();
                        *play_instant_guard = Some(Instant::now());
                    }
                    {
                        let mut lazy_elapsed_lock = context.lazy_elapsed_lock.write().unwrap();
                        *lazy_elapsed_lock = Duration::new(0, 0);
                    }
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

pub struct AlbumsState {
    stateful_list: StatefulList<Album>,
}

impl HasStatefulList for AlbumsState {
    type Item = Album;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct ArtistsState {
    stateful_list: StatefulList<Artist>,
}

impl HasStatefulList for ArtistsState {
    type Item = Artist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct GenresState {
    stateful_list: StatefulList<Genre>,
}

impl HasStatefulList for GenresState {
    type Item = Genre;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct PlaylistsState {
    stateful_list: StatefulList<Playlist>,
}

impl HasStatefulList for PlaylistsState {
    type Item = Playlist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct PromptState {
    message: String,
}

pub struct RootState {
    stateful_list: StatefulList<RootItem>,
}

pub struct RootItem {
    name: String,
}

impl IdName for RootItem {
    fn id(&self) -> i64 {
        0
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

impl From<&str> for RootItem {
    fn from(name: &str) -> Self {
        RootItem {
            name: String::from(name),
        }
    }
}

impl HasStatefulList for RootState {
    type Item = RootItem;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TracksState {
    stateful_list: StatefulList<Track>,
}

impl HasStatefulList for TracksState {
    type Item = Track;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub trait HasStatefulList {
    type Item: IdName;

    fn stateful_list(&self) -> &StatefulList<Self::Item>;
    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item>;
}
pub enum State {
    Albums(AlbumsState),
    Artists(ArtistsState),
    Genres(GenresState),
    Playlists(PlaylistsState),
    Prompt(PromptState),
    Root(RootState),
    Tracks(TracksState),
}

fn create_layout_with_bottom(
    context: &Context,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Option<Vec<Rect>> {
    let play_instant_guard = context.play_instant_lock.read().unwrap();
    let play_instant = (*play_instant_guard)?;
    let queue_guard = context.queue_lock.read().unwrap();
    let first = queue_guard.first()?;
    let lazy_elapsed_guard = context.lazy_elapsed_lock.read().unwrap();
    let sink_guard = context.sink_lock.read().unwrap();
    let elapsed = if sink_guard.is_paused() {
        *lazy_elapsed_guard
    } else {
        *lazy_elapsed_guard + play_instant.elapsed()
    };
    let elapsed_minutes = elapsed.as_secs() / 60;
    let elapsed_seconds = elapsed.as_secs() % 60;
    let duration = Duration::from_millis(first.duration as u64);
    let duration_minutes = duration.as_secs() / 60;
    let duration_seconds = duration.as_secs() % 60;
    let title = format!(" {} ", first.name);
    let percent = cmp::min(100, elapsed.as_millis() * 100 / duration.as_millis()) as u16;
    let label = format!(
        "{}:{:0>2} / {}:{:0>2}",
        elapsed_minutes, elapsed_seconds, duration_minutes, duration_seconds
    );
    let size = f.size();
    let constraints = vec![Constraint::Min(0), Constraint::Length(3)];
    let chunks = Layout::default()
        .constraints(constraints.as_ref())
        .split(size);
    let bottom_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(&title[..]);
    f.render_widget(bottom_block, chunks[1]);
    let bottom_chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(chunks[1]);
    let my_gauge = MyGauge::default().percent(percent).label(&label[..]);
    f.render_widget(my_gauge, bottom_chunks[0]);
    Some(chunks)
}

fn create_layout(context: &Context, f: &mut Frame<CrosstermBackend<Stdout>>) -> Vec<Rect> {
    if let Some(chunks) = create_layout_with_bottom(context, f) {
        chunks
    } else {
        let size = f.size();
        let constraints = vec![Constraint::Min(0)];
        Layout::default()
            .constraints(constraints.as_ref())
            .split(size)
    }
}

fn main() -> Result<(), Error> {
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
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut events = Events::new();

    let client = reqwest::blocking::Client::new();
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink_lock = RwLock::new(rodio::Sink::new_idle().0);
    let queue_lock = RwLock::new(vec![]);
    let play_instant_lock = RwLock::new(None);
    let lazy_elapsed_lock = RwLock::new(Duration::new(0, 0));

    let context = Arc::new(Context {
        server_url,
        api_key,
        client,
        handle,
        sink_lock,
        queue_lock,
        play_instant_lock,
        lazy_elapsed_lock,
    });

    let mut state_machine = StateMachine {
        context: context.clone(),
        state: State::Root(RootState {
            stateful_list: StatefulList::with_items(vec![
                RootItem::from(ALBUMS),
                RootItem::from(ARTISTS),
                RootItem::from(GENRES),
                RootItem::from(PLAYLISTS),
                RootItem::from(TRACKS),
            ]),
        }),
        undo: Vec::new(),
        redo: Vec::new(),
    };

    loop {
        terminal.draw(|f| {
            let chunks = create_layout(&context, f);
            state_machine.render(f, chunks[0]);
        })?;

        events.ignore_events(true);
        state_machine.inputless_transition();
        events.ignore_events(false);

        if let Event::Input(key) = events.next()? {
            match key {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                } => break,
                _ => (),
            }
            state_machine.transition(&key);
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
