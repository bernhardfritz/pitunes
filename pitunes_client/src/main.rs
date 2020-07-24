mod constants;
#[allow(dead_code)]
mod event;
mod http_stream_reader;
mod models;
#[allow(dead_code)]
mod my_gauge;
mod reducers;
mod requests;

use std::cmp;
use std::env;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use clap::{self, value_t};
use constants::{ALBUMS, ARTISTS, GENRES, PI_SYMBOL, PLAYLISTS, STATIC, TRACKS};
use dotenv::dotenv;
use http_stream_reader::HttpStreamReader;
use models::{Album, Artist, Genre, Playlist, Track};
use reducers::REDUCER;
use redux_rs::Store;
use termion::cursor::Goto;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Text};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

use crate::event::{Event, Events};
use crate::my_gauge::MyGauge;

pub struct Context {
    server_url: String,
    api_key: String,
    client: reqwest::blocking::Client,
    device: rodio::Device,
    sink_lock: RwLock<rodio::Sink>,
    queue_lock: RwLock<Vec<Track>>,
    play_instant_lock: RwLock<Option<Instant>>,
    lazy_elapsed_lock: RwLock<Duration>,
}

#[derive(Clone)]
pub enum Model {
    Albums { albums: Vec<Album> },
    Artist { artist: Artist, albums: Vec<Album> },
    Artists { artists: Vec<Artist> },
    Genres { genres: Vec<Genre> },
    Playlists { playlists: Vec<Playlist> },
    Root,
    Tracks { tracks: Vec<Track> },
}

#[derive(Clone)]
pub enum View {
    List {
        list_state: ListState,
        items: Vec<String>,
    },
    Edit {
        input_fields: Vec<(String, String)>,
        selected: Option<usize>,
    },
}

#[derive(Clone)]
pub struct State {
    context: Arc<Context>,
    break_condition: bool,
    model: Model,
    view: View,
    history: Vec<State>,
}

struct BottomState {
    title: String,
    percent: u16,
    label: String,
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
            *sink_guard = rodio::Sink::new(&context.device);
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
                let source =
                    rodio::Decoder::new(HttpStreamReader::new(url, context.api_key.to_string()))
                        .unwrap();
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
    let play_instant_lock = RwLock::new(None);
    let lazy_elapsed_lock = RwLock::new(Duration::new(0, 0));

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
                play_instant_lock,
                lazy_elapsed_lock,
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
            State {
                context,
                break_condition,
                model,
                view,
                history,
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
                tracks.iter().position(|track| track.id == first.id)
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
                        title.push_str(" ─ ");
                        title.push_str(&items[selected][..]);
                    }
                }
            }
            title.push_str(" ");
            title
        };

        terminal.draw(|mut f| {
            let size = f.size();
            let play_instant_guard = state.context.play_instant_lock.read().unwrap();
            let bottom_state = if let Some(play_instant) = *play_instant_guard {
                let queue_guard = state.context.queue_lock.read().unwrap();
                if let Some(first) = queue_guard.first() {
                    let lazy_elapsed_guard = state.context.lazy_elapsed_lock.read().unwrap();
                    let sink_guard = state.context.sink_lock.read().unwrap();
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
                    let percent =
                        cmp::min(100, elapsed.as_millis() * 100 / duration.as_millis()) as u16;
                    let label = format!(
                        "{}:{:0>2} / {}:{:0>2}",
                        elapsed_minutes, elapsed_seconds, duration_minutes, duration_seconds
                    );
                    Some(BottomState {
                        title,
                        percent,
                        label,
                    })
                } else {
                    None
                }
            } else {
                None
            };
            let constraints = if bottom_state.is_some() {
                vec![Constraint::Min(0), Constraint::Length(3)]
            } else {
                vec![Constraint::Min(0)]
            };
            let chunks = Layout::default()
                .constraints(constraints.as_ref())
                .split(size);
            let top_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(&title[..]);
            f.render_widget(top_block, chunks[0]);
            let top_chunks = Layout::default()
                .constraints([Constraint::Min(0)].as_ref())
                .horizontal_margin(3)
                .vertical_margin(2)
                .split(chunks[0]);
            match &state.view {
                View::List { list_state, items } => {
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
                    f.render_stateful_widget(list, top_chunks[0], &mut list_state.clone());
                }
                View::Edit {
                    input_fields,
                    selected,
                } => {
                    let constraints = vec![Constraint::Length(3); input_fields.len() + 1];
                    let top_inner_chunks = Layout::default()
                        .constraints(&constraints[..])
                        .split(top_chunks[0]);
                    for (i, input_field) in input_fields.iter().enumerate() {
                        let text = [Text::raw(&input_field.1[..])];
                        let block = {
                            let block = Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .title(&input_field.0[..]);
                            if let Some(selected) = *selected {
                                if selected == i {
                                    block.title_style(Style::default().modifier(Modifier::BOLD))
                                } else {
                                    block
                                }
                            } else {
                                block
                            }
                        };
                        let paragraph = Paragraph::new(text.iter()).block(block);
                        f.render_widget(paragraph, top_inner_chunks[i]);
                    }
                }
            }
            if let Some(bottom_state) = bottom_state {
                let bottom_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(&bottom_state.title[..]);
                f.render_widget(bottom_block, chunks[1]);
                let bottom_chunks = Layout::default()
                    .constraints([Constraint::Min(0)].as_ref())
                    .horizontal_margin(2)
                    .vertical_margin(1)
                    .split(chunks[1]);
                let my_gauge = MyGauge::default()
                    .percent(bottom_state.percent)
                    .label(&bottom_state.label[..]);
                f.render_widget(my_gauge, bottom_chunks[0]);
            }
        })?;

        match &state.view {
            View::List {
                list_state: _,
                items: _,
            } => terminal.hide_cursor()?,
            View::Edit {
                input_fields,
                selected,
            } => {
                if let Some(selected) = *selected {
                    terminal.show_cursor()?;
                    // Put the cursor back inside the input box
                    write!(
                        terminal.backend_mut(),
                        "{}",
                        Goto(
                            5 + UnicodeWidthStr::width(&input_fields[selected].1[..]) as u16,
                            4 + 3 * selected as u16
                        )
                    )?;
                    // stdout is buffered, flush it to see the effect immediately when hitting backspace
                    io::stdout().flush().ok();
                }
            }
        }

        if let Event::Input(input) = events.next()? {
            store.dispatch(input);
        }
    }

    terminal.clear()?;

    Ok(())
}
