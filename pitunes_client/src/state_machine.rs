use std::{io::Stdout, mem, sync::Arc};

use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use crate::{
    constants::{ALBUMS, ALL_TRACKS, ARTISTS, GENRES, PLAYLISTS, TRACKS},
    models::{
        Album, AlbumQuery, AlbumTracksQuery, AlbumsQuery, Artist, ArtistAlbumsQuery, ArtistQuery,
        ArtistTracksQuery, ArtistsQuery, CreateAlbumMutation, CreateArtistMutation,
        CreateGenreMutation, CreatePlaylistMutation, DeletePlaylistMutation,
        DeletePlaylistTrackMutation, Genre, GenreQuery, GenreTracksQuery, GenresQuery, IdName,
        Playlist, PlaylistTracksQuery, PlaylistsQuery, Track, TracksQuery, UpdateAlbumMutation,
        UpdateArtistMutation, UpdateGenreMutation, UpdatePlaylistMutation,
        UpdatePlaylistTrackMutation, UpdateTrackMutation,
    },
    play_queue,
    requests::{
        create_album, create_artist, create_genre, create_playlist, delete_playlist,
        delete_playlist_track, get_album, get_albums, get_albums_of_artist, get_artist,
        get_artists, get_genre, get_genres, get_playlists, get_tracks, get_tracks_of_album,
        get_tracks_of_artist, get_tracks_of_genre, get_tracks_of_playlist, update_album,
        update_artist, update_genre, update_playlist, update_playlist_track, update_track,
    },
    util::{self, renderer, stateful_list::StatefulList},
    AlbumsState, ArtistsState, Context, GenresState, HasStatefulList, PlaylistsState, PromptState,
    RootState, State, TracksState,
};

pub struct StateMachine {
    pub context: Arc<Context>,
    pub state: State,
    pub undo: Vec<State>,
    pub redo: Vec<State>,
}

impl StateMachine {
    pub fn transition(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('[') => {
                if let Some(to) = self.undo.pop() {
                    self.redo.push(mem::replace(&mut self.state, to))
                }
            }
            KeyCode::Char(']') => {
                if let Some(to) = self.redo.pop() {
                    self.undo.push(mem::replace(&mut self.state, to))
                }
            }
            _ => {
                match &mut self.state {
                    State::Albums(albums_state) => {
                        StateMachine::mutate_stateful_list(albums_state, key)
                    }
                    State::Artists(artists_state) => {
                        StateMachine::mutate_stateful_list(artists_state, key)
                    }
                    State::Genres(genres_state) => {
                        StateMachine::mutate_stateful_list(genres_state, key)
                    }
                    State::Playlists(playlists_state) => {
                        StateMachine::mutate_stateful_list(playlists_state, key)
                    }
                    State::Tracks(tracks_state) => {
                        StateMachine::mutate_stateful_list(tracks_state, key)
                    }
                    State::Root(root_state) => StateMachine::mutate_stateful_list(root_state, key),
                    _ => (),
                }
                let to = match &self.state {
                    State::Albums(albums_state) => self.from_albums(albums_state, key),
                    State::Artists(artists_state) => self.from_artists(artists_state, key),
                    State::Genres(genres_state) => self.from_genres(genres_state, key),
                    State::Playlists(playlists_state) => self.from_playlists(playlists_state, key),
                    State::Tracks(tracks_state) => self.from_tracks(tracks_state, key),
                    State::Root(root_state) => self.from_root(root_state, key),
                    _ => None,
                };
                if let Some(to) = to {
                    self.undo.push(mem::replace(&mut self.state, to));
                    self.redo.clear();
                }
            }
        }
    }

    pub fn inputless_transition(&mut self) {
        let to = match &mut self.state {
            State::Prompt(_) => self.from_prompt(),
            _ => None,
        };
        if let Some(to) = to {
            self.state = to;
        }
    }

    pub fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, chunk: Rect) {
        match &self.state {
            State::Albums(albums_state) => renderer::render_top_block(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                albums_state,
            ),
            State::Artists(artists_state) => renderer::render_top_block(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                artists_state,
            ),
            State::Genres(genres_state) => renderer::render_top_block(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                genres_state,
            ),
            State::Playlists(playlists_state) => renderer::render_top_block(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                playlists_state,
            ),
            State::Prompt(prompt_state) => renderer::render_prompt(f, prompt_state),
            State::Tracks(tracks_state) => renderer::render_top_block(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                tracks_state,
            ),
            State::Root(root_state) => renderer::render_top_block(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                root_state,
            ),
        }
        match &mut self.state {
            State::Albums(albums_state) => {
                renderer::render_stateful_list(f, chunk, albums_state, None)
            }
            State::Artists(artists_state) => {
                renderer::render_stateful_list(f, chunk, artists_state, None)
            }
            State::Genres(genres_state) => {
                renderer::render_stateful_list(f, chunk, genres_state, None)
            }
            State::Playlists(playlists_state) => {
                renderer::render_stateful_list(f, chunk, playlists_state, None)
            }
            State::Tracks(tracks_state) => renderer::render_stateful_list(
                f,
                chunk,
                tracks_state,
                Some(&self.context.queue_lock),
            ),
            State::Root(root_state) => renderer::render_stateful_list(f, chunk, root_state, None),
            _ => (),
        }
    }

    fn mutate_stateful_list(has_stateful_list: &mut impl HasStatefulList, key: &KeyEvent) {
        let stateful_list = has_stateful_list.stateful_list_mut();
        match key.code {
            KeyCode::Up => stateful_list.previous(),
            KeyCode::Down => stateful_list.next(),
            KeyCode::Char(c) => {
                stateful_list.pattern.push(c);
                let old_indices = stateful_list.update_indices(&IdName::name);
                if stateful_list.indices.is_empty() {
                    stateful_list.pattern.pop();
                    stateful_list.indices = old_indices;
                }
            }
            KeyCode::Backspace => {
                if stateful_list.pattern.pop().is_some() {
                    stateful_list.update_indices(&IdName::name);
                }
            }
            KeyCode::Esc | KeyCode::Enter => {
                if !stateful_list.pattern.is_empty() {
                    stateful_list.pattern.clear();
                    let selected_index = stateful_list.selected_index();
                    stateful_list.update_indices(&IdName::name);
                    stateful_list.state.select(selected_index);
                }
            }
            _ => (),
        }
    }

    fn from_albums(&self, albums_state: &AlbumsState, key: &KeyEvent) -> Option<State> {
        let album = albums_state.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_tracks_of_album(album),
            KeyCode::F(2) => {
                if album.id > 0 {
                    self.to_prompt(format!("Album name: ({}) ", album.name()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn from_artists(&self, artists_state: &ArtistsState, key: &KeyEvent) -> Option<State> {
        let artist = artists_state.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_albums_of_artist(artist),
            KeyCode::F(2) => self.to_prompt(format!("Artist name: ({}) ", artist.name())),
            _ => None,
        }
    }

    fn from_genres(&self, genres_state: &GenresState, key: &KeyEvent) -> Option<State> {
        let genre = genres_state.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_tracks_of_genre(genre),
            KeyCode::F(2) => self.to_prompt(format!("Genre name: ({}) ", genre.name())),
            _ => None,
        }
    }

    fn from_playlists(&self, playlists_state: &PlaylistsState, key: &KeyEvent) -> Option<State> {
        let playlist = playlists_state.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_tracks_of_playlist(playlist),
            KeyCode::F(2) => self.to_prompt(format!("Playlist name: ({}) ", playlist.name())),
            _ => None,
        }
    }

    fn from_prompt(&mut self) -> Option<State> {
        disable_raw_mode().ok()?;
        let line = util::read_line().ok()?;
        enable_raw_mode().ok()?;
        if !line.is_empty() {
            let previous_state = self.undo.last_mut()?;
            match previous_state {
                State::Albums(albums_state) => {
                    let album = albums_state.stateful_list_mut().selected_item_mut()?;
                    if album.id > 0 {
                        *album = update_album(&self.context, album, &line[..]);
                    }
                }
                State::Artists(artists_state) => {
                    let artist = artists_state.stateful_list_mut().selected_item_mut()?;
                    *artist = update_artist(&self.context, artist, &line[..]);
                }
                State::Genres(genres_state) => {
                    let genre = genres_state.stateful_list_mut().selected_item_mut()?;
                    *genre = update_genre(&self.context, genre, &line[..]);
                }
                State::Playlists(playlists_state) => {
                    let playlist = playlists_state.stateful_list_mut().selected_item_mut()?;
                    *playlist = update_playlist(&self.context, playlist, &line[..]);
                }
                State::Tracks(tracks_state) => {
                    let track = tracks_state.stateful_list_mut().selected_item_mut()?;
                    *track = update_track(&self.context, track, &line[..], &None, &None, &None);
                    // TODO improve update_track
                }
                _ => (),
            }
        }
        self.undo.pop()
    }

    fn from_root(&self, root_state: &RootState, key: &KeyEvent) -> Option<State> {
        let selected_item = root_state.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => match selected_item.name() {
                ALBUMS => self.to_albums(),
                ARTISTS => self.to_artists(),
                GENRES => self.to_genres(),
                PLAYLISTS => self.to_playlists(),
                TRACKS => self.to_tracks(),
                _ => None,
            },
            _ => None,
        }
    }

    fn from_tracks(&self, tracks_state: &TracksState, key: &KeyEvent) -> Option<State> {
        let stateful_list = tracks_state.stateful_list();
        match key.code {
            KeyCode::Enter => {
                let queue = {
                    let mut queue: Vec<Track> = stateful_list.items.clone();
                    queue.rotate_left(stateful_list.selected_index()?);
                    queue
                };
                play_queue(self.context.clone(), queue);
                None
            }
            KeyCode::F(2) => self.to_prompt(format!(
                "Track name: ({}) ",
                stateful_list.selected_item()?.name()
            )),
            _ => None,
        }
    }

    fn to_prompt(&self, message: String) -> Option<State> {
        Some(State::Prompt(PromptState { message }))
    }

    fn to_tracks_of_album(&self, album: &Album) -> Option<State> {
        let tracks = if album.id > 0 {
            get_tracks_of_album(&self.context, album)
        } else {
            get_tracks_of_artist(
                &self.context,
                &Artist {
                    id: -album.id,
                    name: album.name.clone(),
                },
            )
        };
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::with_items(tracks),
        }))
    }

    fn to_albums(&self) -> Option<State> {
        let albums = get_albums(&self.context);
        Some(State::Albums(AlbumsState {
            stateful_list: StatefulList::with_items(albums),
        }))
    }

    fn to_albums_of_artist(&self, artist: &Artist) -> Option<State> {
        let albums = {
            let mut albums = get_albums_of_artist(&self.context, artist);
            albums.insert(
                0,
                Album {
                    id: -artist.id,
                    name: String::from(ALL_TRACKS),
                },
            );
            albums
        };
        Some(State::Albums(AlbumsState {
            stateful_list: StatefulList::with_items(albums),
        }))
    }

    fn to_artists(&self) -> Option<State> {
        let artists = get_artists(&self.context);
        Some(State::Artists(ArtistsState {
            stateful_list: StatefulList::with_items(artists),
        }))
    }

    fn to_genres(&self) -> Option<State> {
        let genres = get_genres(&self.context);
        Some(State::Genres(GenresState {
            stateful_list: StatefulList::with_items(genres),
        }))
    }

    fn to_playlists(&self) -> Option<State> {
        let playlists = get_playlists(&self.context);
        Some(State::Playlists(PlaylistsState {
            stateful_list: StatefulList::with_items(playlists),
        }))
    }

    fn to_tracks(&self) -> Option<State> {
        let tracks = get_tracks(&self.context);
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::with_items(tracks),
        }))
    }

    fn to_tracks_of_genre(&self, genre: &Genre) -> Option<State> {
        let tracks = get_tracks_of_genre(&self.context, genre);
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::with_items(tracks),
        }))
    }

    fn to_tracks_of_playlist(&self, playlist: &Playlist) -> Option<State> {
        let tracks = get_tracks_of_playlist(&self.context, playlist);
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::with_items(tracks),
        }))
    }
}
