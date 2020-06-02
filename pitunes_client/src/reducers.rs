use std::convert::TryFrom;

use if_chain::if_chain;
use redux_rs::{combine_reducers, Reducer};
use termion::event::Key;
use tui::widgets::ListState;

use crate::constants::{ALBUMS, ALL_TRACKS, ARTISTS, GENRES, PLAYLISTS, TRACKS};
use crate::requests::{
    delete_playlist_track, get_albums, get_albums_of_artist, get_artists, get_genres,
    get_playlists, get_tracks, get_tracks_of_album, get_tracks_of_artist, get_tracks_of_genre,
    get_tracks_of_playlist, update_playlist_track,
};
use crate::{play_queue, Model, State, View};

pub const REDUCER: Reducer<State, Key> = combine_reducers!(
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
