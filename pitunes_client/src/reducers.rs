use std::convert::TryFrom;
use std::time::Instant;

use if_chain::if_chain;
use redux_rs::{combine_reducers, Reducer};
use termion::event::Key;
use tui::widgets::ListState;

use crate::constants::{ALBUMS, ALL_TRACKS, ARTISTS, GENRES, PLAYLISTS, TRACKS};
use crate::models::Track;
use crate::requests::{
    delete_playlist_track, get_albums, get_albums_of_artist, get_artists, get_genres,
    get_playlists, get_tracks, get_tracks_of_album, get_tracks_of_artist, get_tracks_of_genre,
    get_tracks_of_playlist, update_album, update_artist, update_genre, update_playlist,
    update_playlist_track,
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
                let index = if let Some(selected) = list_state.selected() {
                    Some(if selected == 0 {
                        items.len() - 1
                    } else {
                        selected - 1
                    })
                } else {
                    None
                };
                let list_state = {
                    let mut list_state = ListState::default();
                    list_state.select(index);
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
                let index = if let Some(selected) = list_state.selected() {
                    Some(if selected >= items.len() - 1 {
                        0
                    } else {
                        selected + 1
                    })
                } else {
                    None
                };
                let list_state = {
                    let mut list_state = ListState::default();
                    list_state.select(index);
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
                if list_state.selected().is_some() {
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
                    let mut play_instant_guard = state.context.play_instant_lock.write().unwrap();
                    *play_instant_guard = Some(Instant::now());
                } else {
                    sink_guard.pause();
                    let play_instant_guard = state.context.play_instant_lock.read().unwrap();
                    if let Some(play_instant) = *play_instant_guard {
                        let mut lazy_elapsed_guard =
                            state.context.lazy_elapsed_lock.write().unwrap();
                        *lazy_elapsed_guard += play_instant.elapsed();
                    }
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
    let new_state = if let View::Edit {
        input_fields,
        selected,
    } = &state.view
    {
        match action {
            Key::Char('\n') => None,
            Key::Char('\t') => {
                let selected = if let Some(selected) = *selected {
                    Some(if selected >= input_fields.len() - 1 {
                        0
                    } else {
                        selected + 1
                    })
                } else {
                    None
                };
                Some(State {
                    view: View::Edit {
                        input_fields: input_fields.clone(),
                        selected,
                    },
                    ..state.clone()
                })
            }
            Key::BackTab => {
                let selected = if let Some(selected) = *selected {
                    Some(if selected == 0 {
                        input_fields.len() - 1
                    } else {
                        selected - 1
                    })
                } else {
                    None
                };
                Some(State {
                    view: View::Edit {
                        input_fields: input_fields.clone(),
                        selected,
                    },
                    ..state.clone()
                })
            }
            Key::Char(c) => {
                let mut input_fields = input_fields.clone();
                if let Some(selected) = *selected {
                    input_fields[selected].1.push(*c);
                }
                Some(State {
                    view: View::Edit {
                        input_fields,
                        selected: selected.clone(),
                    },
                    ..state.clone()
                })
            }
            Key::Backspace => {
                let mut input_fields = input_fields.clone();
                if let Some(selected) = *selected {
                    input_fields[selected].1.pop();
                }
                Some(State {
                    view: View::Edit {
                        input_fields,
                        selected: selected.clone(),
                    },
                    ..state.clone()
                })
            }
            Key::Esc => {
                if let Some(last) = state.history.last() {
                    Some(last.clone())
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
                                    let index = if albums.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
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
                                    let index = if artists.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
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
                                    let index = if genres.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
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
                                    let index = if playlists.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
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
                                    let index = if tracks.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
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
                            let index = if tracks.is_empty() { None } else { Some(0) };
                            list_state.select(index);
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
                Key::Char('e') => {
                    if let Some(selected) = list_state.selected() {
                        let album = &albums[selected];
                        let history = {
                            let mut history = state.history.clone();
                            if state.add_to_history {
                                history.push(state.clone());
                            }
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![(String::from("Name"), album.name.clone())], // TODO: would be cool to reuse views in order to have some sort of chooser dialogue instead of entering ids manually
                                selected: Some(0),
                            },
                            history,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            View::Edit {
                input_fields,
                selected: _,
            } => match action {
                Key::Char('\n') => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state, items: _ } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        then {
                            update_album(&state.context, &albums[selected], &input_fields[0].1[..]);
                            Some(REDUCER(second_last, &Key::Char('\n')))
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            },
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
        match &state.view {
            View::List {
                list_state,
                items: _,
            } => match action {
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
                            let index = if tracks.is_empty() { None } else { Some(0) };
                            list_state.select(index);
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
                Key::Char('e') => {
                    if let Some(selected) = list_state.selected() {
                        if selected > 0 {
                            let album = &albums[selected - 1];
                            let history = {
                                let mut history = state.history.clone();
                                if state.add_to_history {
                                    history.push(state.clone());
                                }
                                history
                            };
                            Some(State {
                                view: View::Edit {
                                    input_fields: vec![(String::from("Name"), album.name.clone())], // TODO: would be cool to reuse views in order to have some sort of chooser dialogue instead of entering ids manually
                                    selected: Some(0),
                                },
                                history,
                                ..state.clone()
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
            View::Edit {
                input_fields,
                selected: _,
            } => match action {
                Key::Char('\n') => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state, items: _ } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        then {
                            update_album(&state.context, &albums[selected - 1], &input_fields[0].1[..]);
                            Some(REDUCER(second_last, &Key::Char('\n')))
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            },
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
        match &state.view {
            View::List {
                list_state,
                items: _,
            } => match action {
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
                Key::Char('e') => {
                    if let Some(selected) = list_state.selected() {
                        let artist = &artists[selected];
                        let history = {
                            let mut history = state.history.clone();
                            if state.add_to_history {
                                history.push(state.clone());
                            }
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![(String::from("Name"), artist.name.clone())], // TODO: would be cool to reuse views in order to have some sort of chooser dialogue instead of entering ids manually
                                selected: Some(0),
                            },
                            history,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            View::Edit {
                input_fields,
                selected: _,
            } => match action {
                Key::Char('\n') => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state, items: _ } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        then {
                            update_artist(&state.context, &artists[selected], &input_fields[0].1[..]);
                            Some(REDUCER(second_last, &Key::Char('\n')))
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            },
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
        match &state.view {
            View::List {
                list_state,
                items: _,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let genre = &genres[selected];
                        let tracks = get_tracks_of_genre(&state.context, genre);
                        let list_state = {
                            let mut list_state = ListState::default();
                            let index = if tracks.is_empty() { None } else { Some(0) };
                            list_state.select(index);
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
                Key::Char('e') => {
                    if let Some(selected) = list_state.selected() {
                        let genre = &genres[selected];
                        let history = {
                            let mut history = state.history.clone();
                            if state.add_to_history {
                                history.push(state.clone());
                            }
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![(String::from("Name"), genre.name.clone())], // TODO: would be cool to reuse views in order to have some sort of chooser dialogue instead of entering ids manually
                                selected: Some(0),
                            },
                            history,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            View::Edit {
                input_fields,
                selected: _,
            } => match action {
                Key::Char('\n') => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state, items: _ } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        then {
                            update_genre(&state.context, &genres[selected], &input_fields[0].1[..]);
                            Some(REDUCER(second_last, &Key::Char('\n')))
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            },
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
        match &state.view {
            View::List {
                list_state,
                items: _,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let playlist = &playlists[selected];
                        let tracks = get_tracks_of_playlist(&state.context, playlist);
                        let list_state = {
                            let mut list_state = ListState::default();
                            let index = if tracks.is_empty() { None } else { Some(0) };
                            list_state.select(index);
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
                Key::Char('e') => {
                    if let Some(selected) = list_state.selected() {
                        let playlist = &playlists[selected];
                        let history = {
                            let mut history = state.history.clone();
                            if state.add_to_history {
                                history.push(state.clone());
                            }
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![(String::from("Name"), playlist.name.clone())], // TODO: would be cool to reuse views in order to have some sort of chooser dialogue instead of entering ids manually
                                selected: Some(0),
                            },
                            history,
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            View::Edit {
                input_fields,
                selected: _,
            } => match action {
                Key::Char('\n') => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state, items: _ } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        then {
                            update_playlist(&state.context, &playlists[selected], &input_fields[0].1[..]);
                            Some(REDUCER(second_last, &Key::Char('\n')))
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            },
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
                    let mut queue: Vec<Track> = tracks.clone();
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
