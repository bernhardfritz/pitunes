use std::convert::TryFrom;
use std::time::Instant;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use if_chain::if_chain;
use redux_rs::{combine_reducers, Reducer};
use termion::event::Key;
use tui::widgets::ListState;

use crate::constants::{
    ALBUMS, ALL_TRACKS, ARTISTS, CREATE_NEW_PLAYLIST, ELLIPSIS, GENRES, PLAYLISTS, TRACKS,
};
use crate::models::Track;
use crate::requests::{
    create_playlist, delete_playlist, delete_playlist_track, get_album, get_albums,
    get_albums_of_artist, get_artist, get_artists, get_genre, get_genres, get_playlists,
    get_tracks, get_tracks_of_album, get_tracks_of_artist, get_tracks_of_genre,
    get_tracks_of_playlist, update_album, update_artist, update_genre, update_playlist,
    update_playlist_track, update_track,
};
use crate::{play_queue, InputField, Model, State, View};

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
    root_reducer,
    stop_propagation_reducer
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
    let new_state = if let View::List {
        list_state,
        items,
        pattern,
        indices,
    } = &state.view
    {
        match action {
            Key::Up => {
                let index = if let Some(selected) = list_state.selected() {
                    Some(if selected == 0 {
                        indices.len() - 1
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
                        pattern: pattern.clone(),
                        indices: indices.clone(),
                    },
                    ..state.clone()
                })
            }
            Key::Down => {
                let index = if let Some(selected) = list_state.selected() {
                    Some(if selected >= indices.len() - 1 {
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
                        pattern: pattern.clone(),
                        indices: indices.clone(),
                    },
                    ..state.clone()
                })
            }
            Key::Esc => {
                let list_state = if let Some(selected) = list_state.selected() {
                    let mut list_state = ListState::default();
                    list_state.select(Some(indices[selected]));
                    list_state
                } else {
                    let mut list_state = ListState::default();
                    let index = if items.is_empty() { None } else { Some(0) };
                    list_state.select(index);
                    list_state
                };
                let pattern = None;
                let indices = (0..items.len()).collect();
                let view = View::List {
                    list_state,
                    items: items.clone(),
                    pattern,
                    indices,
                };
                Some(State {
                    view,
                    ..state.clone()
                })
            }
            Key::Char('\n') => {
                if list_state.selected().is_some() {
                    if let Model::Tracks { tracks: _ } = state.model {
                        None
                    } else {
                        if_chain! {
                            if let Some(last) = state.history.last();
                            if let View::Edit { input_fields: _, selected: _ } = last.view;
                            then {
                                None
                            } else {
                                let history = {
                                    let mut history = state.history.clone();
                                    history.push(state.clone());
                                    history
                                };
                                Some(State {
                                    history,
                                    ..state.clone()
                                })
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Key::Backspace => {
                if let Some(pattern) = pattern {
                    let (pattern, indices): (Option<String>, Vec<usize>) = {
                        let mut pattern = pattern.clone();
                        let c = pattern.pop();
                        if c.is_some() {
                            let matcher = SkimMatcherV2::default();
                            let indices = {
                                let mut indices_score: Vec<(usize, i64)> = items
                                    .iter()
                                    .enumerate()
                                    .map(|(i, item)| {
                                        (i, matcher.fuzzy_match(&item[..], &pattern[..]))
                                    })
                                    .filter(|(_i, score)| score.is_some())
                                    .map(|(i, score)| (i, score.unwrap()))
                                    .collect();
                                indices_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                                indices_score.iter().map(|(i, _score)| *i).collect()
                            };
                            (Some(pattern), indices)
                        } else {
                            let indices = (0..items.len()).collect();
                            (None, indices)
                        }
                    };
                    let list_state = {
                        let mut list_state = list_state.clone();
                        if indices.is_empty() {
                            list_state.select(None);
                        } else {
                            list_state.select(Some(0));
                        }
                        list_state
                    };
                    let view = View::List {
                        list_state: list_state,
                        items: items.clone(),
                        pattern,
                        indices,
                    };
                    Some(State {
                        view,
                        ..state.clone()
                    })
                } else {
                    if let Some(last) = state.history.last() {
                        let view = if let View::List {
                            list_state,
                            items,
                            pattern: _,
                            indices,
                        } = &last.view
                        {
                            let list_state = if let Some(selected) = list_state.selected() {
                                let mut list_state = ListState::default();
                                list_state.select(Some(indices[selected]));
                                list_state
                            } else {
                                let mut list_state = ListState::default();
                                let index = if items.is_empty() { None } else { Some(0) };
                                list_state.select(index);
                                list_state
                            };
                            let pattern = None;
                            let indices = (0..items.len()).collect();
                            View::List {
                                list_state,
                                items: items.clone(),
                                pattern,
                                indices,
                            }
                        } else {
                            last.view.clone()
                        };
                        Some(State {
                            view,
                            ..last.clone()
                        })
                    } else {
                        None
                    }
                }
            }
            Key::Char(c) => {
                if let Some(pattern) = pattern {
                    let pattern = {
                        let mut pattern = pattern.clone();
                        pattern.push(*c);
                        pattern
                    };
                    let matcher = SkimMatcherV2::default();
                    let indices: Vec<usize> = {
                        let mut indices_score: Vec<(usize, i64)> = items
                            .iter()
                            .enumerate()
                            .map(|(i, item)| (i, matcher.fuzzy_match(&item[..], &pattern[..])))
                            .filter(|(_i, score)| score.is_some())
                            .map(|(i, score)| (i, score.unwrap()))
                            .collect();
                        indices_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        indices_score.iter().map(|(i, _score)| *i).collect()
                    };
                    let list_state = {
                        let mut list_state = list_state.clone();
                        if indices.is_empty() {
                            list_state.select(None);
                        } else {
                            list_state.select(Some(0));
                        }
                        list_state
                    };
                    let view = View::List {
                        list_state,
                        items: items.clone(),
                        pattern: Some(pattern),
                        indices,
                    };
                    Some(State {
                        view,
                        ..state.clone()
                    })
                } else {
                    match c {
                        ' ' => {
                            let sink_guard = state.context.sink_lock.read().unwrap();
                            if sink_guard.is_paused() {
                                sink_guard.play();
                                let mut play_instant_guard =
                                    state.context.play_instant_lock.write().unwrap();
                                *play_instant_guard = Some(Instant::now());
                            } else {
                                sink_guard.pause();
                                let play_instant_guard =
                                    state.context.play_instant_lock.read().unwrap();
                                if let Some(play_instant) = *play_instant_guard {
                                    let mut lazy_elapsed_guard =
                                        state.context.lazy_elapsed_lock.write().unwrap();
                                    *lazy_elapsed_guard += play_instant.elapsed();
                                }
                            }
                            Some(state.clone())
                        }
                        '/' => {
                            let pattern = Some(String::new());
                            let view = View::List {
                                list_state: list_state.clone(),
                                items: items.clone(),
                                pattern,
                                indices: indices.clone(),
                            };
                            Some(State {
                                view,
                                ..state.clone()
                            })
                        }
                        _ => None,
                    }
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

fn edit_reducer(state: &State, action: &Key) -> State {
    let new_state = if let View::Edit {
        input_fields,
        selected,
    } = &state.view
    {
        match action {
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
                    if let InputField::Text { key: _, value } = &mut input_fields[selected] {
                        value.push(*c);
                    }
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
                    if let InputField::Text { key: _, value } = &mut input_fields[selected] {
                        value.pop();
                    }
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

fn root_reducer_albums(state: &State) -> State {
    let albums = get_albums(&state.context);
    let list_state = {
        let mut list_state = ListState::default();
        let index = if albums.is_empty() { None } else { Some(0) };
        list_state.select(index);
        list_state
    };
    let items: Vec<String> = albums.iter().map(|album| album.name.clone()).collect();
    let pattern = None;
    let indices = (0..items.len()).collect();
    State {
        model: Model::Albums { albums },
        view: View::List {
            list_state,
            items,
            pattern,
            indices,
        },
        ..state.clone()
    }
}

fn root_reducer_artists(state: &State) -> State {
    let artists = get_artists(&state.context);
    let list_state = {
        let mut list_state = ListState::default();
        let index = if artists.is_empty() { None } else { Some(0) };
        list_state.select(index);
        list_state
    };
    let items: Vec<String> = artists.iter().map(|artist| artist.name.clone()).collect();
    let pattern = None;
    let indices = (0..items.len()).collect();
    State {
        model: Model::Artists { artists },
        view: View::List {
            list_state,
            items,
            pattern,
            indices,
        },
        ..state.clone()
    }
}

fn root_reducer_genres(state: &State) -> State {
    let genres = get_genres(&state.context);
    let list_state = {
        let mut list_state = ListState::default();
        let index = if genres.is_empty() { None } else { Some(0) };
        list_state.select(index);
        list_state
    };
    let items: Vec<String> = genres.iter().map(|genre| genre.name.clone()).collect();
    let pattern = None;
    let indices = (0..items.len()).collect();
    State {
        model: Model::Genres { genres },
        view: View::List {
            list_state,
            items,
            pattern,
            indices,
        },
        ..state.clone()
    }
}

fn root_reducer_playlists(state: &State) -> State {
    let playlists = get_playlists(&state.context);
    let list_state = {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        list_state
    };
    let items: Vec<String> = {
        let mut items: Vec<String> = playlists
            .iter()
            .map(|playlist| playlist.name.clone())
            .collect();
        items.insert(0, String::from(CREATE_NEW_PLAYLIST));
        items
    };
    let pattern = None;
    let indices = (0..items.len()).collect();
    State {
        model: Model::Playlists { playlists },
        view: View::List {
            list_state,
            items,
            pattern,
            indices,
        },
        ..state.clone()
    }
}

fn root_reducer_tracks(state: &State) -> State {
    let tracks = get_tracks(&state.context);
    let list_state = {
        let mut list_state = ListState::default();
        let index = if tracks.is_empty() { None } else { Some(0) };
        list_state.select(index);
        list_state
    };
    let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
    let pattern = None;
    let indices = (0..items.len()).collect();
    State {
        model: Model::Tracks { tracks },
        view: View::List {
            list_state,
            items,
            pattern,
            indices,
        },
        ..state.clone()
    }
}

fn root_reducer(state: &State, action: &Key) -> State {
    let new_state = if let Model::Root = state.model {
        if let View::List {
            list_state,
            items,
            pattern: _,
            indices,
        } = &state.view
        {
            match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let item = &items[indices[selected]];
                        match &item[..] {
                            ALBUMS => Some(root_reducer_albums(state)),
                            ARTISTS => Some(root_reducer_artists(state)),
                            GENRES => Some(root_reducer_genres(state)),
                            PLAYLISTS => Some(root_reducer_playlists(state)),
                            TRACKS => Some(root_reducer_tracks(state)),
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
    if state.stop_propagation {
        return state.clone();
    }
    let new_state = if let Model::Albums { albums } = &state.model {
        match &state.view {
            View::List {
                list_state,
                items: _,
                pattern: _,
                indices,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let album = &albums[indices[selected]];
                        if_chain! {
                            if let Some(last) = state.history.last();
                            if let View::Edit { input_fields, selected } = &last.view;
                            then {
                                let input_fields = {
                                    let mut input_fields = input_fields.clone();
                                    if let InputField::Chooser { key, value: _, id: _ } = &input_fields[2] {
                                        input_fields[2] = InputField::Chooser { key: key.clone(), value: album.name.clone(), id: Some(album.id) };
                                    }
                                    input_fields
                                };
                                Some(State {
                                    view: View::Edit { input_fields, selected: selected.clone() },
                                    ..last.clone()
                                })
                            } else {
                                let tracks = get_tracks_of_album(&state.context, &album);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    let index = if tracks.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
                                    list_state
                                };
                                let items: Vec<String> =
                                    tracks.iter().map(|track| track.name.clone()).collect();
                                let pattern = None;
                                let indices = (0..items.len()).collect();
                                Some(State {
                                    model: Model::Tracks { tracks },
                                    view: View::List {
                                        list_state,
                                        items,
                                        pattern,
                                        indices,
                                    },
                                    ..state.clone()
                                })
                            }
                        }
                    } else {
                        None
                    }
                }
                Key::F(2) => {
                    if let Some(selected) = list_state.selected() {
                        let album = &albums[indices[selected]];
                        let history = {
                            let mut history = state.history.clone();
                            history.push(state.clone());
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![InputField::Text {
                                    key: String::from("Name"),
                                    value: album.name.clone(),
                                }],
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
                        if let View::List { list_state, items: _, pattern: _, indices } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        if let InputField::Text { key: _, value } = &input_fields[0];
                        then {
                            update_album(&state.context, &albums[indices[selected]], &value[..]);
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
                pattern: _,
                indices,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let tracks = if indices[selected] == 0 {
                            get_tracks_of_artist(&state.context, artist)
                        } else {
                            let album = &albums[indices[selected] - 1];
                            get_tracks_of_album(&state.context, album)
                        };
                        let list_state = {
                            let mut list_state = ListState::default();
                            let index = if tracks.is_empty() { None } else { Some(0) };
                            list_state.select(index);
                            list_state
                        };
                        let items: Vec<String> =
                            tracks.iter().map(|track| track.name.clone()).collect();
                        let pattern = None;
                        let indices = (0..items.len()).collect();
                        Some(State {
                            model: Model::Tracks { tracks },
                            view: View::List {
                                list_state,
                                items,
                                pattern,
                                indices,
                            },
                            ..state.clone()
                        })
                    } else {
                        None
                    }
                }
                Key::F(2) => {
                    if let Some(selected) = list_state.selected() {
                        if indices[selected] > 0 {
                            let album = &albums[indices[selected] - 1];
                            let history = {
                                let mut history = state.history.clone();
                                history.push(state.clone());
                                history
                            };
                            Some(State {
                                view: View::Edit {
                                    input_fields: vec![InputField::Text {
                                        key: String::from("Name"),
                                        value: album.name.clone(),
                                    }],
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
                        if let View::List { list_state, items: _, pattern: _, indices } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        if let InputField::Text { key: _, value } = &input_fields[0];
                        then {
                            update_album(&state.context, &albums[indices[selected] - 1], &value[..]);
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
    if state.stop_propagation {
        return state.clone();
    }
    let new_state = if let Model::Artists { artists } = &state.model {
        match &state.view {
            View::List {
                list_state,
                items: _,
                pattern: _,
                indices,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let artist = artists[indices[selected]].clone();
                        if_chain! {
                            if let Some(last) = state.history.last();
                            if let View::Edit { input_fields, selected } = &last.view;
                            then {
                                let input_fields = {
                                    let mut input_fields = input_fields.clone();
                                    if let InputField::Chooser { key, value: _, id: _ } = &input_fields[1] {
                                        input_fields[1] = InputField::Chooser { key: key.clone(), value: artist.name.clone(), id: Some(artist.id) };
                                    }
                                    input_fields
                                };
                                Some(State {
                                    view: View::Edit { input_fields, selected: selected.clone() },
                                    ..last.clone()
                                })
                            } else {
                                let albums = get_albums_of_artist(&state.context, &artist);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    list_state.select(Some(0));
                                    list_state
                                };
                                let items: Vec<String> = {
                                    let mut items: Vec<String> =
                                        albums.iter().map(|album| album.name.clone()).collect();
                                    items.insert(0, String::from(ALL_TRACKS));
                                    items
                                };
                                let pattern = None;
                                let indices = (0..items.len()).collect();
                                Some(State {
                                    model: Model::Artist { artist, albums },
                                    view: View::List {
                                        list_state,
                                        items,
                                        pattern,
                                        indices,
                                    },
                                    ..state.clone()
                                })
                            }
                        }
                    } else {
                        None
                    }
                }
                Key::F(2) => {
                    if let Some(selected) = list_state.selected() {
                        let artist = &artists[indices[selected]];
                        let history = {
                            let mut history = state.history.clone();
                            history.push(state.clone());
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![InputField::Text {
                                    key: String::from("Name"),
                                    value: artist.name.clone(),
                                }],
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
                        if let View::List { list_state, items: _, pattern: _, indices } = &last.view;
                        if let Some(list_state_selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        if let InputField::Text { key: _, value } = &input_fields[0];
                        then {
                            update_artist(&state.context, &artists[indices[list_state_selected]], &value[..]);
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
    if state.stop_propagation {
        return state.clone();
    }
    let new_state = if let Model::Genres { genres } = &state.model {
        match &state.view {
            View::List {
                list_state,
                items: _,
                pattern: _,
                indices,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        let genre = &genres[indices[selected]];
                        if_chain! {
                            if let Some(last) = state.history.last();
                            if let View::Edit { input_fields, selected } = &last.view;
                            then {
                                let input_fields = {
                                    let mut input_fields = input_fields.clone();
                                    if let InputField::Chooser { key, value: _, id: _ } = &input_fields[3] {
                                        input_fields[3] = InputField::Chooser { key: key.clone(), value: genre.name.clone(), id: Some(genre.id) };
                                    }
                                    input_fields
                                };
                                Some(State {
                                    view: View::Edit { input_fields, selected: selected.clone() },
                                    ..last.clone()
                                })
                            } else {
                                let tracks = get_tracks_of_genre(&state.context, genre);
                                let list_state = {
                                    let mut list_state = ListState::default();
                                    let index = if tracks.is_empty() { None } else { Some(0) };
                                    list_state.select(index);
                                    list_state
                                };
                                let items: Vec<String> =
                                    tracks.iter().map(|track| track.name.clone()).collect();
                                let pattern = None;
                                let indices = (0..items.len()).collect();
                                Some(State {
                                    model: Model::Tracks { tracks },
                                    view: View::List {
                                        list_state,
                                        items,
                                        pattern,
                                        indices,
                                    },
                                    ..state.clone()
                                })
                            }
                        }
                    } else {
                        None
                    }
                }
                Key::F(2) => {
                    if let Some(selected) = list_state.selected() {
                        let genre = &genres[indices[selected]];
                        let history = {
                            let mut history = state.history.clone();
                            history.push(state.clone());
                            history
                        };
                        Some(State {
                            view: View::Edit {
                                input_fields: vec![InputField::Text {
                                    key: String::from("Name"),
                                    value: genre.name.clone(),
                                }],
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
                        if let View::List { list_state, items: _, pattern: _, indices } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        if let InputField::Text { key: _, value } = &input_fields[0];
                        then {
                            update_genre(&state.context, &genres[indices[selected]], &value[..]);
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
                pattern,
                indices,
            } => match action {
                Key::Char('\n') => {
                    if let Some(selected) = list_state.selected() {
                        if indices[selected] == 0 {
                            Some(State {
                                view: View::Edit {
                                    input_fields: vec![InputField::Text {
                                        key: String::from("Name"),
                                        value: String::new(),
                                    }],
                                    selected: Some(0),
                                },
                                ..state.clone()
                            })
                        } else {
                            let playlist = &playlists[indices[selected] - 1];
                            let tracks = get_tracks_of_playlist(&state.context, playlist);
                            let list_state = {
                                let mut list_state = ListState::default();
                                let index = if tracks.is_empty() { None } else { Some(0) };
                                list_state.select(index);
                                list_state
                            };
                            let items: Vec<String> =
                                tracks.iter().map(|track| track.name.clone()).collect();
                            let pattern = None;
                            let indices = (0..items.len()).collect();
                            Some(State {
                                model: Model::Tracks { tracks },
                                view: View::List {
                                    list_state,
                                    items,
                                    pattern,
                                    indices,
                                },
                                ..state.clone()
                            })
                        }
                    } else {
                        None
                    }
                }
                Key::F(2) => {
                    if let Some(selected) = list_state.selected() {
                        if indices[selected] > 0 {
                            let playlist = &playlists[indices[selected] - 1];
                            let history = {
                                let mut history = state.history.clone();
                                history.push(state.clone());
                                history
                            };
                            Some(State {
                                view: View::Edit {
                                    input_fields: vec![InputField::Text {
                                        key: String::from("Name"),
                                        value: playlist.name.clone(),
                                    }],
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
                Key::Char('d') => {
                    if_chain! {
                        if pattern.is_none(); // disregard key events while in search mode
                        if let Some(selected) = list_state.selected();
                        if indices[selected] > 0;
                        let playlist = &playlists[indices[selected] - 1];
                        let deleted = delete_playlist(&state.context, playlist);
                        if deleted;
                        if let Some(last) = state.history.last();
                        then {
                            Some(REDUCER(last, &Key::Char('\n')))
                        } else {
                            None
                        }
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
                        if let View::List { list_state, items: _, pattern: _, indices } = &last.view;
                        if let Some(selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        if let InputField::Text { key: _, value } = &input_fields[0];
                        then {
                            if indices[selected] == 0 {
                                create_playlist(&state.context, &value[..]);
                            } else {
                                update_playlist(&state.context, &playlists[indices[selected] - 1], &value[..]);
                            }
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
    let new_state = if let Model::Tracks { tracks } = &state.model {
        match &state.view {
            View::List {
                list_state,
                items,
                pattern,
                indices,
            } => {
                if let Some(track_index) = list_state.selected() {
                    match action {
                        Key::Char('\n') => {
                            let mut queue: Vec<Track> = tracks.clone();
                            queue.rotate_left(indices[track_index]);
                            play_queue(state.context.clone(), queue);
                            let list_state = {
                                let mut list_state = ListState::default();
                                list_state.select(Some(indices[track_index]));
                                list_state
                            };
                            let pattern = None;
                            let indices = (0..items.len()).collect();
                            let view = View::List {
                                list_state,
                                items: items.clone(),
                                pattern,
                                indices,
                            };
                            Some(State {
                                view,
                                ..state.clone()
                            })
                        }
                        Key::F(2) => {
                            let track = &tracks[indices[track_index]];
                            let history = {
                                let mut history = state.history.clone();
                                history.push(state.clone());
                                history
                            };
                            let artist = track
                                .artist_id
                                .map(|artist_id| get_artist(&state.context, artist_id));
                            let album = track
                                .album_id
                                .map(|album_id| get_album(&state.context, album_id));
                            let genre = track
                                .genre_id
                                .map(|genre_id| get_genre(&state.context, genre_id));
                            Some(State {
                                view: View::Edit {
                                    input_fields: vec![
                                        InputField::Text {
                                            key: String::from("Name"),
                                            value: track.name.clone(),
                                        },
                                        InputField::Chooser {
                                            key: String::from("Artist"),
                                            value: artist.map_or_else(
                                                || String::from(ELLIPSIS),
                                                |artist| artist.name.clone(),
                                            ),
                                            id: track.artist_id,
                                        },
                                        InputField::Chooser {
                                            key: String::from("Album"),
                                            value: album.map_or_else(
                                                || String::from(ELLIPSIS),
                                                |album| album.name.clone(),
                                            ),
                                            id: track.album_id,
                                        },
                                        InputField::Chooser {
                                            key: String::from("Genre"),
                                            value: genre.map_or_else(
                                                || String::from(ELLIPSIS),
                                                |genre| genre.name.clone(),
                                            ),
                                            id: track.genre_id,
                                        },
                                    ],
                                    selected: Some(0),
                                },
                                history,
                                ..state.clone()
                            })
                        }
                        _ => {
                            if_chain! {
                                if let Some(last) = state.history.last();
                                if let View::List { list_state: prev_list_state, items: _, pattern: _, indices: prev_indices } = &last.view;
                                if pattern.is_none(); // disregard key events while in search mode
                                then {
                                    match &last.model {
                                        Model::Playlists { playlists } => {
                                            if let Some(playlist_index) = prev_list_state.selected() {
                                                let playlist = &playlists[prev_indices[playlist_index]];
                                                match action {
                                                    Key::Char('d') => {
                                                        let track = &tracks[track_index]; // track_index can be used directly as we are not in search mode
                                                        let position = Some(i64::try_from(track_index).unwrap());
                                                        let deleted = delete_playlist_track(&state.context, playlist, track, position);
                                                        if deleted {
                                                            Some(REDUCER(last, &Key::Char('\n')))
                                                        } else {
                                                            None
                                                        }
                                                    }
                                                    Key::Char('j') => {
                                                        if track_index > 0 { // track_index can be used directly as we are not in search mode
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
                                                            let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
                                                            let pattern = None;
                                                            let indices = (0..items.len()).collect();
                                                            Some(State {
                                                                model: Model::Tracks { tracks },
                                                                view: View::List { list_state, items, pattern, indices },
                                                                ..state.clone()
                                                            })
                                                        } else {
                                                            None
                                                        }
                                                    }
                                                    Key::Char('k') => {
                                                        if track_index < tracks.len() - 1 { // track_index can be used directly as we are not in search mode
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
                                                            let items: Vec<String> = tracks.iter().map(|track| track.name.clone()).collect();
                                                            let pattern = None;
                                                            let indices = (0..items.len()).collect();
                                                            Some(State {
                                                                model: Model::Tracks { tracks },
                                                                view: View::List { list_state, items, pattern, indices },
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
            }
            View::Edit {
                input_fields,
                selected,
            } => match action {
                Key::Char('\n') => {
                    if_chain! {
                        if let Some(last) = state.history.last();
                        if let View::List { list_state, items: _, pattern: _, indices } = &last.view;
                        if let Some(list_state_selected) = list_state.selected();
                        if let Some(second_last) = last.history.last();
                        if let InputField::Text { key: _, value: name } = &input_fields[0];
                        if let InputField::Chooser { key: _, value: _, id: artist_id } = &input_fields[1];
                        if let InputField::Chooser { key: _, value: _, id: album_id } = &input_fields[2];
                        if let InputField::Chooser { key: _, value: _, id: genre_id } = &input_fields[3];
                        then {
                            let history = {
                                let mut history = state.history.clone();
                                history.push(state.clone());
                                history
                            };
                            let stop_propagation = true;
                            match selected {
                                Some(1) => Some(State { history, stop_propagation, ..root_reducer_artists(state) }),
                                Some(2) => Some(State { history, stop_propagation, ..root_reducer_albums(state) }),
                                Some(3) => Some(State { history, stop_propagation, ..root_reducer_genres(state) }),
                                _ => {
                                    update_track(&state.context, &tracks[indices[list_state_selected]], &name[..], album_id, artist_id, genre_id);
                                    Some(REDUCER(second_last, &Key::Char('\n')))
                                }
                            }
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

fn stop_propagation_reducer(state: &State, _action: &Key) -> State {
    return State {
        stop_propagation: false,
        ..state.clone()
    };
}
