use std::{io::Stdout, mem, sync::Arc};

use crossterm::event::{KeyCode, KeyEvent};
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use crate::{
    constants::{ALBUMS, ALL_TRACKS, ARTISTS, GENRES, PLAYLISTS, TRACKS},
    models::{
        Album, AlbumQuery, AlbumTracksQuery, AlbumsQuery, Artist, ArtistAlbumsQuery, ArtistQuery,
        ArtistTracksQuery, ArtistsQuery, CreateAlbumMutation, CreateArtistMutation,
        CreateGenreMutation, CreatePlaylistMutation, DeletePlaylistMutation,
        DeletePlaylistTrackMutation, Genre, GenreQuery, GenreTracksQuery, GenresQuery, IdName,
        Playlist, PlaylistTracksQuery, PlaylistsQuery, Track, TrackInputBuilder, TracksQuery,
        UpdateAlbumMutation, UpdateArtistMutation, UpdateGenreMutation, UpdatePlaylistMutation,
        UpdatePlaylistTrackMutation, UpdateTrackMutation,
    },
    play_queue, renderer,
    requests::{
        create_album, create_artist, create_genre, create_playlist, create_playlist_track,
        delete_playlist, delete_playlist_track, get_album, get_albums, get_albums_of_artist,
        get_artist, get_artists, get_genre, get_genres, get_playlists, get_track, get_tracks,
        get_tracks_of_album, get_tracks_of_artist, get_tracks_of_genre, get_tracks_of_playlist,
        update_album, update_artist, update_genre, update_playlist, update_playlist_track,
        update_track,
    },
    states::{
        AddToPlaylistPromptState, AlbumsState, ArtistsState, GenresState, HasStatefulList,
        PlaylistsState, PromptState, RootState, State, TrackAlbumPromptState,
        TrackArtistPromptState, TrackGenrePromptState, TrackNumberPromptState, TracksState,
    },
    util::stateful_list::StatefulList,
    Context,
};

pub struct StateMachine {
    pub context: Arc<Context>,
    pub state: State,
    pub undo: Vec<State>,
    pub redo: Vec<State>,
}

impl StateMachine {
    pub fn transition(&mut self, key: &KeyEvent) {
        match &self.state {
            State::Albums(_)
            | State::Artists(_)
            | State::Genres(_)
            | State::Playlists(_)
            | State::Root(_)
            | State::Tracks(_) => match key.code {
                KeyCode::Char('[') => {
                    if let Some(to) = self.undo.pop() {
                        self.redo.push(mem::replace(&mut self.state, to))
                    }
                    return;
                }
                KeyCode::Char(']') => {
                    if let Some(to) = self.redo.pop() {
                        self.undo.push(mem::replace(&mut self.state, to))
                    }
                    return;
                }
                _ => (),
            },
            _ => (),
        }
        match &mut self.state {
            State::AddToPlaylistPrompt(add_to_playlist_prompt_state) => {
                StateMachine::mutate_stateful_list(add_to_playlist_prompt_state, key);
                let to = self.from_add_to_playlist_prompt(key);
                if let Some(to) = to {
                    self.state = to;
                    return;
                }
            }
            State::Albums(albums_state) => StateMachine::mutate_stateful_list(albums_state, key),
            State::Artists(artists_state) => StateMachine::mutate_stateful_list(artists_state, key),
            State::Genres(genres_state) => StateMachine::mutate_stateful_list(genres_state, key),
            State::Playlists(playlists_state) => {
                StateMachine::mutate_stateful_list(playlists_state, key)
            }
            State::Prompt(prompt_state) => {
                StateMachine::mutate_prompt(prompt_state, key);
                let to = self.from_prompt(key);
                if let Some(to) = to {
                    self.state = to;
                    return;
                }
            }
            State::Root(root_state) => StateMachine::mutate_stateful_list(root_state, key),
            State::TrackAlbumPrompt(track_album_prompt_state) => {
                StateMachine::mutate_stateful_list(track_album_prompt_state, key);
            }
            State::TrackArtistPrompt(track_artist_prompt_state) => {
                StateMachine::mutate_stateful_list(track_artist_prompt_state, key);
            }
            State::TrackGenrePrompt(track_genre_prompt_state) => {
                StateMachine::mutate_stateful_list(track_genre_prompt_state, key);
                let to = self.from_track_genre_prompt(key);
                if let Some(to) = to {
                    self.state = to;
                    return;
                }
            }
            State::TrackNumberPrompt(track_number_prompt_state) => {
                StateMachine::mutate_track_number_prompt(track_number_prompt_state, key);
            }
            State::Tracks(tracks_state) => StateMachine::mutate_stateful_list(tracks_state, key),
        }
        let to = match &self.state {
            State::Albums(albums_state) => self.from_albums(albums_state, key),
            State::Artists(artists_state) => self.from_artists(artists_state, key),
            State::Genres(genres_state) => self.from_genres(genres_state, key),
            State::Playlists(playlists_state) => self.from_playlists(playlists_state, key),
            State::Root(root_state) => self.from_root(root_state, key),
            State::TrackAlbumPrompt(track_album_prompt_state) => {
                self.from_track_album_prompt(track_album_prompt_state, key)
            }
            State::TrackArtistPrompt(track_artist_prompt_state) => {
                self.from_track_artist_prompt(track_artist_prompt_state, key)
            }
            State::TrackNumberPrompt(track_number_prompt_state) => {
                self.from_track_number_prompt(track_number_prompt_state, key)
            }
            State::Tracks(tracks_state) => self.from_tracks(tracks_state, key),
            _ => None,
        };
        if let Some(to) = to {
            let old_state = mem::replace(&mut self.state, to);
            match old_state {
                State::Albums(_)
                | State::Artists(_)
                | State::Genres(_)
                | State::Playlists(_)
                | State::Root(_)
                | State::Tracks(_) => {
                    self.undo.push(old_state);
                    self.redo.clear();
                }
                _ => (),
            }
        }
    }

    pub fn is_prompt_state(&self) -> bool {
        match self.state {
            State::AddToPlaylistPrompt(_) => true,
            State::Prompt(_) => true,
            State::TrackAlbumPrompt(_) => true,
            State::TrackArtistPrompt(_) => true,
            State::TrackGenrePrompt(_) => true,
            State::TrackNumberPrompt(_) => true,
            _ => false,
        }
    }

    pub fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, chunk: Rect) {
        match &mut self.state {
            State::AddToPlaylistPrompt(add_to_playlist_prompt_state) => {
                renderer::render_autocomplete_prompt(f, chunk, add_to_playlist_prompt_state)
            }
            State::Albums(albums_state) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                albums_state,
                None,
            ),
            State::Artists(artists_state) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                artists_state,
                None,
            ),
            State::Genres(genres_state) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                genres_state,
                None,
            ),
            State::Playlists(playlists_state) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                playlists_state,
                None,
            ),
            State::Prompt(prompt_state) => renderer::render_prompt(f, chunk, prompt_state),
            State::Tracks(tracks_state) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                tracks_state,
                Some(&self.context.queue_lock),
            ),
            State::TrackAlbumPrompt(track_album_prompt_state) => {
                renderer::render_autocomplete_prompt(f, chunk, track_album_prompt_state)
            }
            State::TrackArtistPrompt(track_artist_prompt_state) => {
                renderer::render_autocomplete_prompt(f, chunk, track_artist_prompt_state)
            }
            State::TrackGenrePrompt(track_genre_prompt_state) => {
                renderer::render_autocomplete_prompt(f, chunk, track_genre_prompt_state)
            }
            State::TrackNumberPrompt(track_number_prompt_state) => {
                renderer::render_prompt(f, chunk, track_number_prompt_state)
            }
            State::Root(root_state) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                root_state,
                None,
            ),
        }
    }

    fn mutate_stateful_list(has_stateful_list: &mut impl HasStatefulList, key: &KeyEvent) {
        let stateful_list = has_stateful_list.stateful_list_mut();
        match key.code {
            KeyCode::Up => stateful_list.previous(),
            KeyCode::Down => stateful_list.next(),
            KeyCode::Char(c) => {
                if stateful_list.autocomplete() {
                    if let Some(selected_item) = stateful_list.selected_item() {
                        stateful_list.pattern = String::from(selected_item.name());
                    }
                }
                stateful_list.pattern.push(c);
                let old_indices = stateful_list.update_indices(&IdName::name);
                if !stateful_list.autocomplete() && stateful_list.indices.is_empty() {
                    stateful_list.pattern.pop();
                    stateful_list.indices = old_indices;
                }
            }
            KeyCode::Backspace => {
                if stateful_list.autocomplete() {
                    if let Some(selected_item) = stateful_list.selected_item() {
                        stateful_list.pattern = String::from(selected_item.name());
                    }
                }
                if stateful_list.pattern.pop().is_some() {
                    stateful_list.update_indices(&IdName::name);
                }
            }
            KeyCode::Esc | KeyCode::Enter => {
                if stateful_list.autocomplete() {
                    // TODO
                } else {
                    if !stateful_list.pattern.is_empty() {
                        stateful_list.pattern.clear();
                        let selected_index = stateful_list.selected_index();
                        stateful_list.update_indices(&IdName::name);
                        stateful_list.state.select(selected_index);
                    }
                }
            }
            _ => (),
        }
    }

    fn mutate_prompt(prompt_state: &mut PromptState, key: &KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                prompt_state.answer.push(c);
            }
            KeyCode::Backspace => {
                prompt_state.answer.pop();
            }
            _ => (),
        }
    }

    fn mutate_track_number_prompt(
        track_number_prompt_state: &mut TrackNumberPromptState,
        key: &KeyEvent,
    ) {
        match key.code {
            KeyCode::Char(c) => {
                if track_number_prompt_state.answer.is_empty() {
                    if let '1'..='9' = c {
                        track_number_prompt_state.answer.push(c)
                    }
                } else {
                    if let '0'..='9' = c {
                        track_number_prompt_state.answer.push(c)
                    }
                }
            }
            KeyCode::Backspace => {
                track_number_prompt_state.answer.pop();
            }
            _ => (),
        }
    }

    fn from_add_to_playlist_prompt(&mut self, key: &KeyEvent) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let add_to_playlist_prompt_state =
            if let State::AddToPlaylistPrompt(add_to_playlist_prompt_state) = &self.state {
                add_to_playlist_prompt_state
            } else {
                panic!()
            };
        let stateful_list = add_to_playlist_prompt_state.stateful_list();
        let state = self.undo.pop()?;
        let track = if let State::Tracks(tracks_state) = &state {
            tracks_state.stateful_list().selected_item().unwrap()
        } else {
            panic!()
        };
        if let Some(playlist) = stateful_list.selected_item() {
            create_playlist_track(&self.context, playlist, track);
        } else {
            let name = stateful_list.pattern.trim();
            if !name.is_empty() {
                let playlist = create_playlist(&self.context, name);
                create_playlist_track(&self.context, &playlist, track);
            }
        }
        Some(state)
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

    fn from_prompt(&mut self, key: &KeyEvent) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let prompt_state = if let State::Prompt(prompt_state) = &self.state {
            prompt_state
        } else {
            panic!()
        };
        let answer = prompt_state.answer.trim();
        let previous_state = self.undo.last_mut()?;
        match previous_state {
            State::Albums(albums_state) => {
                if !answer.is_empty() {
                    let album = albums_state.stateful_list_mut().selected_item_mut()?;
                    if album.id > 0 {
                        *album = update_album(&self.context, album, answer);
                    }
                }
                self.undo.pop()
            }
            State::Artists(artists_state) => {
                if !answer.is_empty() {
                    let artist = artists_state.stateful_list_mut().selected_item_mut()?;
                    *artist = update_artist(&self.context, artist, answer);
                }
                self.undo.pop()
            }
            State::Genres(genres_state) => {
                if !answer.is_empty() {
                    let genre = genres_state.stateful_list_mut().selected_item_mut()?;
                    *genre = update_genre(&self.context, genre, answer);
                }
                self.undo.pop()
            }
            State::Playlists(playlists_state) => {
                if !answer.is_empty() {
                    let playlist = playlists_state.stateful_list_mut().selected_item_mut()?;
                    *playlist = update_playlist(&self.context, playlist, answer);
                }
                self.undo.pop()
            }
            State::Tracks(tracks_state) => {
                let track = tracks_state.stateful_list().selected_item()?;
                let track_input_builder = {
                    let mut track_input_builder =
                        TrackInputBuilder::new(track.id, track.name.clone());
                    track_input_builder
                        .album_id(track.album.as_ref().map(|album| album.id))
                        .artist_id(track.artist.as_ref().map(|artist| artist.id))
                        .genre_id(track.genre.as_ref().map(|genre| genre.id))
                        .track_number(track.track_number);
                    if !answer.is_empty() {
                        track_input_builder.name(String::from(answer));
                    }
                    track_input_builder
                };
                let prompt = format!(
                    "Artist name: ({}) ",
                    track.artist.as_ref().map_or("", |artist| artist.name())
                );
                self.to_track_artist_prompt(prompt, track_input_builder)
            }
            _ => None,
        }
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

    fn from_track_album_prompt(
        &self,
        track_album_prompt_state: &TrackAlbumPromptState,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = if let Some(State::Tracks(tracks_state)) = self.undo.last() {
            tracks_state.stateful_list.selected_item().unwrap()
        } else {
            panic!()
        };
        let stateful_list = track_album_prompt_state.stateful_list();
        let track_input_builder = {
            let mut track_input_builder = track_album_prompt_state.track_input_builder.clone();
            if let Some(album) = stateful_list.selected_item() {
                track_input_builder.album_id(Some(album.id));
            } else {
                let name = stateful_list.pattern.trim();
                if name.is_empty() {
                    let album = track.album.as_ref();
                    track_input_builder.album_id(album.map(|album| album.id));
                } else {
                    let album = create_album(&self.context, name);
                    track_input_builder.album_id(Some(album.id));
                };
            }
            track_input_builder
        };
        self.to_track_number_prompt(
            format!(
                "Track number: ({}) ",
                track
                    .track_number
                    .as_ref()
                    .map_or(String::new(), |track_number| track_number.to_string())
            ),
            track_input_builder,
        )
    }

    fn from_track_artist_prompt(
        &self,
        track_artist_prompt_state: &TrackArtistPromptState,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = if let Some(State::Tracks(tracks_state)) = self.undo.last() {
            tracks_state.stateful_list.selected_item().unwrap()
        } else {
            panic!()
        };
        let track_input_builder = {
            let mut track_input_builder = track_artist_prompt_state.track_input_builder.clone();
            let stateful_list = track_artist_prompt_state.stateful_list();
            if let Some(artist) = stateful_list.selected_item() {
                track_input_builder.artist_id(Some(artist.id));
            } else {
                let name = stateful_list.pattern.trim();
                if name.is_empty() {
                    let artist = track.artist.as_ref();
                    track_input_builder.artist_id(artist.map(|artist| artist.id));
                } else {
                    let artist = create_artist(&self.context, name);
                    track_input_builder.artist_id(Some(artist.id));
                };
            }
            track_input_builder
        };
        self.to_track_album_prompt(
            format!(
                "Album name: ({}) ",
                track.album.as_ref().map_or("", |album| album.name())
            ),
            track_input_builder,
        )
    }

    fn from_track_genre_prompt(&mut self, key: &KeyEvent) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track_genre_prompt_state =
            if let State::TrackGenrePrompt(track_genre_prompt_state) = &self.state {
                track_genre_prompt_state
            } else {
                panic!()
            };
        let track = if let Some(State::Tracks(tracks_state)) = self.undo.last() {
            tracks_state.stateful_list.selected_item().unwrap()
        } else {
            panic!()
        };
        let track_input_builder = {
            let mut track_input_builder = track_genre_prompt_state.track_input_builder.clone();
            let stateful_list = track_genre_prompt_state.stateful_list();
            if let Some(genre) = stateful_list.selected_item() {
                track_input_builder.genre_id(Some(genre.id));
            } else {
                let name = stateful_list.pattern.trim();
                if name.is_empty() {
                    let genre = track.genre.as_ref();
                    track_input_builder.genre_id(genre.map(|genre| genre.id));
                } else {
                    let genre = create_genre(&self.context, name);
                    track_input_builder.genre_id(Some(genre.id));
                };
            }
            track_input_builder
        };
        let state = {
            let mut state = self.undo.pop()?;
            let track = if let State::Tracks(tracks_state) = &mut state {
                tracks_state
                    .stateful_list_mut()
                    .selected_item_mut()
                    .unwrap()
            } else {
                panic!()
            };
            *track = update_track(&self.context, track_input_builder.build());
            state
        };
        Some(state)
    }

    fn from_track_number_prompt(
        &self,
        track_number_prompt_state: &TrackNumberPromptState,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = if let Some(State::Tracks(tracks_state)) = self.undo.last() {
            tracks_state.stateful_list.selected_item().unwrap()
        } else {
            panic!()
        };
        let track_input_builder = {
            let mut track_input_builder = track_number_prompt_state.track_input_builder.clone();
            let track_number = track_number_prompt_state.answer.trim();
            if !track_number.is_empty() {
                track_input_builder.track_number(Some(track_number.parse().unwrap()));
            }
            track_input_builder
        };
        self.to_track_genre_prompt(
            format!(
                "Genre name: ({}) ",
                track.genre.as_ref().map_or("", |genre| genre.name())
            ),
            track_input_builder,
        )
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
            KeyCode::F(4) => self.to_add_to_playlist_prompt(String::from("Playlist name: ")),
            _ => None,
        }
    }

    fn to_add_to_playlist_prompt(&self, prompt: String) -> Option<State> {
        let playlists = get_playlists(&self.context);
        Some(State::AddToPlaylistPrompt(AddToPlaylistPromptState {
            prompt,
            stateful_list: StatefulList::builder()
                .items(playlists)
                .autocomplete(true)
                .build(),
        }))
    }

    fn to_prompt(&self, prompt: String) -> Option<State> {
        Some(State::Prompt(PromptState {
            prompt,
            answer: String::new(),
        }))
    }

    fn to_track_album_prompt(
        &self,
        prompt: String,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        let albums = get_albums(&self.context);
        Some(State::TrackAlbumPrompt(TrackAlbumPromptState {
            prompt,
            stateful_list: StatefulList::builder()
                .items(albums)
                .autocomplete(true)
                .build(),
            track_input_builder,
        }))
    }

    fn to_track_artist_prompt(
        &self,
        prompt: String,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        let artists = get_artists(&self.context);
        Some(State::TrackArtistPrompt(TrackArtistPromptState {
            prompt,
            stateful_list: StatefulList::builder()
                .items(artists)
                .autocomplete(true)
                .build(),
            track_input_builder,
        }))
    }

    fn to_track_genre_prompt(
        &self,
        prompt: String,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        let genres = get_genres(&self.context);
        Some(State::TrackGenrePrompt(TrackGenrePromptState {
            prompt,
            stateful_list: StatefulList::builder()
                .items(genres)
                .autocomplete(true)
                .build(),
            track_input_builder,
        }))
    }

    fn to_track_number_prompt(
        &self,
        prompt: String,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        Some(State::TrackNumberPrompt(TrackNumberPromptState {
            prompt,
            answer: String::new(),
            track_input_builder,
        }))
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
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }

    fn to_albums(&self) -> Option<State> {
        let albums = get_albums(&self.context);
        Some(State::Albums(AlbumsState {
            stateful_list: StatefulList::builder().items(albums).build(),
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
            stateful_list: StatefulList::builder().items(albums).build(),
        }))
    }

    fn to_artists(&self) -> Option<State> {
        let artists = get_artists(&self.context);
        Some(State::Artists(ArtistsState {
            stateful_list: StatefulList::builder().items(artists).build(),
        }))
    }

    fn to_genres(&self) -> Option<State> {
        let genres = get_genres(&self.context);
        Some(State::Genres(GenresState {
            stateful_list: StatefulList::builder().items(genres).build(),
        }))
    }

    fn to_playlists(&self) -> Option<State> {
        let playlists = get_playlists(&self.context);
        Some(State::Playlists(PlaylistsState {
            stateful_list: StatefulList::builder().items(playlists).build(),
        }))
    }

    fn to_tracks(&self) -> Option<State> {
        let tracks = get_tracks(&self.context);
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }

    fn to_tracks_of_genre(&self, genre: &Genre) -> Option<State> {
        let tracks = get_tracks_of_genre(&self.context, genre);
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }

    fn to_tracks_of_playlist(&self, playlist: &Playlist) -> Option<State> {
        let tracks = get_tracks_of_playlist(&self.context, playlist);
        Some(State::Tracks(TracksState {
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }
}
