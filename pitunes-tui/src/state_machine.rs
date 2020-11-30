use std::{io::Stdout, mem, sync::Arc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use crate::{
    constants::{ALBUMS, ALL_TRACKS, ARTISTS, GENRES, PLAYLISTS, TRACKS},
    models::{
        Album, AlbumInputBuilder, AlbumQuery, AlbumTracksQuery, AlbumsQuery, Artist,
        ArtistAlbumsQuery, ArtistInputBuilder, ArtistQuery, ArtistTracksQuery, ArtistsQuery,
        CreateAlbumMutation, CreateArtistMutation, CreateGenreMutation, CreatePlaylistMutation,
        DeletePlaylistMutation, DeletePlaylistTrackMutation, Genre, GenreInputBuilder, GenreQuery,
        GenreTracksQuery, GenresQuery, IdName, Playlist, PlaylistInputBuilder, PlaylistTracksQuery,
        PlaylistsQuery, Track, TrackInputBuilder, TracksQuery, UpdateAlbumMutation,
        UpdateArtistMutation, UpdateGenreMutation, UpdatePlaylistMutation,
        UpdatePlaylistTrackMutation, UpdateTrackMutation,
    },
    play_queue, renderer,
    requests::{
        create_album, create_artist, create_genre, create_playlist, create_playlist_track,
        delete_album, delete_artist, delete_genre, delete_playlist, delete_playlist_track,
        read_album, read_albums, read_albums_of_artist, read_artist, read_artists, read_genre,
        read_genres, read_playlists, read_track, read_tracks, read_tracks_of_album,
        read_tracks_of_artist, read_tracks_of_genre, read_tracks_of_playlist, update_album,
        update_artist, update_genre, update_playlist, update_playlist_track, update_track,
    },
    states::HasPrompt,
    states::{
        AddToPlaylistPrompt, AlbumNamePrompt, AlbumTracks, Albums, ArtistAlbums, ArtistNamePrompt,
        ArtistTracks, Artists, ConfirmPrompt, GenreNamePrompt, GenreTracks, Genres,
        HasStatefulList, PlaylistNamePrompt, PlaylistTracks, Playlists, Root, State,
        TrackAlbumPrompt, TrackArtistPrompt, TrackGenrePrompt, TrackNamePrompt, TrackNumberPrompt,
        Tracks,
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
            | State::AlbumTracks(_)
            | State::ArtistAlbums(_)
            | State::Artists(_)
            | State::ArtistTracks(_)
            | State::Genres(_)
            | State::GenreTracks(_)
            | State::Playlists(_)
            | State::PlaylistTracks(_)
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
            State::AddToPlaylistPrompt(add_to_playlist_prompt) => {
                StateMachine::mutate_stateful_list(add_to_playlist_prompt, key)
            }
            State::AlbumNamePrompt(album_name_prompt) => {
                StateMachine::mutate_prompt(album_name_prompt, key)
            }
            State::AlbumTracks(album_tracks) => {
                StateMachine::mutate_stateful_list(album_tracks, key)
            }
            State::Albums(albums) => StateMachine::mutate_stateful_list(albums, key),
            State::ArtistAlbums(artist_albums) => {
                StateMachine::mutate_stateful_list(artist_albums, key)
            }
            State::ArtistNamePrompt(artist_name_prompt) => {
                StateMachine::mutate_prompt(artist_name_prompt, key)
            }
            State::Artists(artists) => StateMachine::mutate_stateful_list(artists, key),
            State::ArtistTracks(artist_tracks) => {
                StateMachine::mutate_stateful_list(artist_tracks, key)
            }
            State::ConfirmPrompt(confirm_prompt) => {
                StateMachine::mutate_confirm_prompt(confirm_prompt, key)
            }
            State::GenreNamePrompt(genre_name_prompt) => {
                StateMachine::mutate_prompt(genre_name_prompt, key)
            }
            State::Genres(genres) => StateMachine::mutate_stateful_list(genres, key),
            State::GenreTracks(genre_tracks) => {
                StateMachine::mutate_stateful_list(genre_tracks, key)
            }
            State::PlaylistNamePrompt(playlist_name_prompt) => {
                StateMachine::mutate_prompt(playlist_name_prompt, key)
            }
            State::Playlists(playlists) => StateMachine::mutate_stateful_list(playlists, key),
            State::PlaylistTracks(playlist_tracks) => {
                StateMachine::mutate_stateful_list(playlist_tracks, key)
            }
            State::Root(root) => StateMachine::mutate_stateful_list(root, key),
            State::TrackAlbumPrompt(track_album_prompt) => {
                StateMachine::mutate_stateful_list(track_album_prompt, key);
            }
            State::TrackArtistPrompt(track_artist_prompt) => {
                StateMachine::mutate_stateful_list(track_artist_prompt, key);
            }
            State::TrackGenrePrompt(track_genre_prompt) => {
                StateMachine::mutate_stateful_list(track_genre_prompt, key);
            }
            State::TrackNamePrompt(track_name_prompt) => {
                StateMachine::mutate_prompt(track_name_prompt, key);
            }
            State::TrackNumberPrompt(track_number_prompt) => {
                StateMachine::mutate_track_number_prompt(track_number_prompt, key);
            }
            State::Tracks(tracks) => {
                StateMachine::mutate_stateful_list(tracks, key);
                let to = self.from_tracks_mut(key);
                if let Some(to) = to {
                    self.state = to;
                    return;
                }
            }
        }
        let refresh = match &self.state {
            State::AddToPlaylistPrompt(add_to_playlist_prompt) => {
                self.from_add_to_playlist_prompt(add_to_playlist_prompt, key)
            }
            State::AlbumNamePrompt(album_name_prompt) => {
                self.from_album_name_prompt(album_name_prompt, key)
            }
            State::ArtistNamePrompt(artist_name_prompt) => {
                self.from_artist_name_prompt(artist_name_prompt, key)
            }
            State::ConfirmPrompt(confirm_prompt) => self.from_confirm_prompt(confirm_prompt, key),
            State::GenreNamePrompt(genre_name_prompt) => {
                self.from_genre_name_prompt(genre_name_prompt, key)
            }
            State::PlaylistNamePrompt(playlist_name_prompt) => {
                self.from_playlist_name_prompt(playlist_name_prompt, key)
            }
            State::TrackGenrePrompt(track_genre_prompt) => {
                self.from_track_genre_prompt(track_genre_prompt, key)
            }
            _ => None,
        };
        if refresh.is_some() {
            if let Some(to) = self.undo.pop() {
                self.state = to;
            }
            self.refresh();
            return;
        }
        let to = match &self.state {
            State::Albums(albums) => self.from_albums(albums, key),
            State::AlbumTracks(album_tracks) => self.from_tracks(album_tracks, key),
            State::ArtistAlbums(artist_albums) => self.from_albums(artist_albums, key),
            State::Artists(artists) => self.from_artists(artists, key),
            State::ArtistTracks(artist_tracks) => self.from_tracks(artist_tracks, key),
            State::Genres(genres) => self.from_genres(genres, key),
            State::GenreTracks(genre_tracks) => self.from_tracks(genre_tracks, key),
            State::Playlists(playlists) => self.from_playlists(playlists, key),
            State::PlaylistTracks(playlist_tracks) => self.from_tracks(playlist_tracks, key),
            State::Root(root) => self.from_root(root, key),
            State::TrackAlbumPrompt(track_album_prompt) => {
                self.from_track_album_prompt(track_album_prompt, key)
            }
            State::TrackArtistPrompt(track_artist_prompt) => {
                self.from_track_artist_prompt(track_artist_prompt, key)
            }
            State::TrackNamePrompt(track_name_prompt) => {
                self.from_track_name_prompt(track_name_prompt, key)
            }
            State::TrackNumberPrompt(track_number_prompt) => {
                self.from_track_number_prompt(track_number_prompt, key)
            }
            State::Tracks(tracks) => self.from_tracks(tracks, key),
            _ => None,
        };
        if let Some(to) = to {
            let old_state = mem::replace(&mut self.state, to);
            match old_state {
                State::Albums(_)
                | State::AlbumTracks(_)
                | State::ArtistAlbums(_)
                | State::Artists(_)
                | State::ArtistTracks(_)
                | State::Genres(_)
                | State::GenreTracks(_)
                | State::Playlists(_)
                | State::PlaylistTracks(_)
                | State::Root(_)
                | State::Tracks(_) => {
                    self.undo.push(old_state);
                    self.redo.clear();
                }
                _ => (),
            }
        }
    }

    pub fn is_prompt(&self) -> bool {
        match self.state {
            State::AddToPlaylistPrompt(_) => true,
            State::AlbumNamePrompt(_) => true,
            State::ArtistNamePrompt(_) => true,
            State::ConfirmPrompt(_) => true,
            State::GenreNamePrompt(_) => true,
            State::PlaylistNamePrompt(_) => true,
            State::TrackAlbumPrompt(_) => true,
            State::TrackArtistPrompt(_) => true,
            State::TrackGenrePrompt(_) => true,
            State::TrackNamePrompt(_) => true,
            State::TrackNumberPrompt(_) => true,
            _ => false,
        }
    }

    pub fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, chunk: Rect) {
        match &mut self.state {
            State::AddToPlaylistPrompt(add_to_playlist_prompt) => {
                renderer::render_autocomplete_prompt(f, chunk, add_to_playlist_prompt)
            }
            State::AlbumNamePrompt(album_name_prompt) => {
                renderer::render_prompt(f, chunk, album_name_prompt)
            }
            State::Albums(albums) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                albums,
                None,
            ),
            State::AlbumTracks(album_tracks) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                album_tracks,
                Some(&self.context.queue_lock),
            ),
            State::ArtistAlbums(artist_albums) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                artist_albums,
                None,
            ),
            State::ArtistNamePrompt(artist_name_prompt) => {
                renderer::render_prompt(f, chunk, artist_name_prompt)
            }
            State::Artists(artists) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                artists,
                None,
            ),
            State::ArtistTracks(artist_tracks) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                artist_tracks,
                Some(&self.context.queue_lock),
            ),
            State::ConfirmPrompt(confirm_prompt) => {
                renderer::render_prompt(f, chunk, confirm_prompt)
            }
            State::GenreNamePrompt(genre_name_prompt) => {
                renderer::render_prompt(f, chunk, genre_name_prompt)
            }
            State::Genres(genres) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                genres,
                None,
            ),
            State::GenreTracks(genre_tracks) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                genre_tracks,
                Some(&self.context.queue_lock),
            ),
            State::PlaylistNamePrompt(playlist_name_prompt) => {
                renderer::render_prompt(f, chunk, playlist_name_prompt)
            }
            State::Playlists(playlists) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                playlists,
                None,
            ),
            State::PlaylistTracks(playlist_tracks) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                playlist_tracks,
                Some(&self.context.queue_lock),
            ),
            State::Tracks(tracks) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                tracks,
                Some(&self.context.queue_lock),
            ),
            State::TrackAlbumPrompt(track_album_prompt) => {
                renderer::render_autocomplete_prompt(f, chunk, track_album_prompt)
            }
            State::TrackArtistPrompt(track_artist_prompt) => {
                renderer::render_autocomplete_prompt(f, chunk, track_artist_prompt)
            }
            State::TrackGenrePrompt(track_genre_prompt) => {
                renderer::render_autocomplete_prompt(f, chunk, track_genre_prompt)
            }
            State::TrackNamePrompt(track_name_prompt) => {
                renderer::render_prompt(f, chunk, track_name_prompt)
            }
            State::TrackNumberPrompt(track_number_prompt) => {
                renderer::render_prompt(f, chunk, track_number_prompt)
            }
            State::Root(root) => renderer::render_top_block_and_stateful_list(
                f,
                chunk,
                &self.context.server_url[..],
                &self.undo[..],
                root,
                None,
            ),
        }
    }

    fn mutate_stateful_list(has_stateful_list: &mut impl HasStatefulList, key: &KeyEvent) {
        let stateful_list = has_stateful_list.stateful_list_mut();
        match key {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            } => stateful_list.previous(),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            } => stateful_list.next(),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: _,
            } => {
                if stateful_list.autocomplete() {
                    if let Some(selected_item) = stateful_list.selected_item() {
                        stateful_list.pattern = String::from(selected_item.name());
                    }
                }
                stateful_list.pattern.push(*c);
                let old_indices = stateful_list.update_indices(&IdName::name);
                if !stateful_list.autocomplete() && stateful_list.indices.is_empty() {
                    stateful_list.pattern.pop();
                    stateful_list.indices = old_indices;
                }
            }
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: _,
            } => {
                if stateful_list.autocomplete() {
                    if let Some(selected_item) = stateful_list.selected_item() {
                        stateful_list.pattern = String::from(selected_item.name());
                    }
                }
                if stateful_list.pattern.pop().is_some() {
                    stateful_list.update_indices(&IdName::name);
                }
            }
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
            }
            | KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
            } => {
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

    fn mutate_confirm_prompt(confirm_prompt: &mut ConfirmPrompt, key: &KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                if confirm_prompt.answer.is_empty() {
                    if let 'y' | 'Y' | 'n' | 'N' = c {
                        confirm_prompt.answer.push(c);
                    }
                }
            }
            KeyCode::Backspace => {
                confirm_prompt.answer.pop();
            }
            _ => (),
        }
    }

    fn mutate_prompt(has_prompt: &mut impl HasPrompt, key: &KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                has_prompt.answer_mut().push(c);
            }
            KeyCode::Backspace => {
                has_prompt.answer_mut().pop();
            }
            _ => (),
        }
    }

    fn mutate_track_number_prompt(track_number_prompt: &mut TrackNumberPrompt, key: &KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                if track_number_prompt.answer.is_empty() {
                    if let '1'..='9' = c {
                        track_number_prompt.answer.push(c)
                    }
                } else {
                    if let '0'..='9' = c {
                        track_number_prompt.answer.push(c)
                    }
                }
            }
            KeyCode::Backspace => {
                track_number_prompt.answer.pop();
            }
            _ => (),
        }
    }

    fn from_add_to_playlist_prompt(
        &self,
        add_to_playlist_prompt: &AddToPlaylistPrompt,
        key: &KeyEvent,
    ) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let stateful_list = add_to_playlist_prompt.stateful_list();
        let last = self.undo.last()?;
        let track = last.selected_track()?;
        if let Some(playlist) = stateful_list.selected_item() {
            create_playlist_track(&self.context, playlist, track);
        } else {
            let name = stateful_list.pattern.trim();
            if !name.is_empty() {
                let playlist = create_playlist(&self.context, name);
                create_playlist_track(&self.context, &playlist, track);
            }
        }
        Some(())
    }

    fn from_album_name_prompt(
        &self,
        album_name_prompt: &AlbumNamePrompt,
        key: &KeyEvent,
    ) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        if !album_name_prompt.answer.is_empty() {
            let album_input_builder = {
                let mut album_input_builder = album_name_prompt.album_input_builder.clone();
                album_input_builder.name(album_name_prompt.answer.clone());
                album_input_builder
            };
            update_album(&self.context, album_input_builder.build());
        }
        Some(())
    }

    fn from_albums(
        &self,
        has_albums: &impl HasStatefulList<Item = Album>,
        key: &KeyEvent,
    ) -> Option<State> {
        let album = has_albums.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_album_tracks(album),
            KeyCode::F(2) => {
                if album.id > 0 {
                    self.to_album_name_prompt(album)
                } else {
                    None
                }
            }
            KeyCode::Delete => {
                if album.id > 0 {
                    self.to_confirm_prompt(album)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn from_artist_name_prompt(
        &self,
        artist_name_prompt: &ArtistNamePrompt,
        key: &KeyEvent,
    ) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        if !artist_name_prompt.answer.is_empty() {
            let artist_input_builder = {
                let mut artist_input_builder = artist_name_prompt.artist_input_builder.clone();
                artist_input_builder.name(artist_name_prompt.answer.clone());
                artist_input_builder
            };
            update_artist(&self.context, artist_input_builder.build());
        }
        Some(())
    }

    fn from_artists(&self, artists: &Artists, key: &KeyEvent) -> Option<State> {
        let artist = artists.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_artist_albums(artist),
            KeyCode::F(2) => self.to_artist_name_prompt(artist),
            KeyCode::Delete => self.to_confirm_prompt(artist),
            _ => None,
        }
    }

    fn from_confirm_prompt(&self, confirm_prompt: &ConfirmPrompt, key: &KeyEvent) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        if !confirm_prompt.answer.is_empty() {
            if &confirm_prompt.answer[..] == "y" || &confirm_prompt.answer[..] == "Y" {
                let last = self.undo.last()?;
                match last {
                    State::Albums(albums) => {
                        delete_album(&self.context, albums.stateful_list().selected_item()?);
                    }
                    State::ArtistAlbums(artist_albums) => {
                        delete_album(
                            &self.context,
                            artist_albums.stateful_list().selected_item()?,
                        );
                    }
                    State::Artists(artists) => {
                        delete_artist(&self.context, artists.stateful_list().selected_item()?);
                    }
                    State::Genres(genres) => {
                        delete_genre(&self.context, genres.stateful_list().selected_item()?);
                    }
                    State::Playlists(playlists) => {
                        delete_playlist(&self.context, playlists.stateful_list().selected_item()?);
                    }
                    State::PlaylistTracks(playlist_tracks) => {
                        delete_playlist_track(
                            &self.context,
                            &playlist_tracks.playlist,
                            last.selected_track()?,
                            playlist_tracks.stateful_list().selected_index(),
                        );
                    }
                    _ => (),
                }
            }
        }
        Some(())
    }

    fn from_genre_name_prompt(
        &self,
        genre_name_prompt: &GenreNamePrompt,
        key: &KeyEvent,
    ) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        if !genre_name_prompt.answer.is_empty() {
            let genre_input_builder = {
                let mut genre_input_builder = genre_name_prompt.genre_input_builder.clone();
                genre_input_builder.name(genre_name_prompt.answer.clone());
                genre_input_builder
            };
            update_genre(&self.context, genre_input_builder.build());
        }
        Some(())
    }

    fn from_genres(&self, genres: &Genres, key: &KeyEvent) -> Option<State> {
        let genre = genres.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_genre_tracks(genre),
            KeyCode::F(2) => self.to_genre_name_prompt(genre),
            KeyCode::Delete => self.to_confirm_prompt(genre),
            _ => None,
        }
    }

    fn from_playlist_name_prompt(
        &self,
        playlist_name_prompt: &PlaylistNamePrompt,
        key: &KeyEvent,
    ) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        if !playlist_name_prompt.answer.is_empty() {
            let playlist_input_builder = {
                let mut playlist_input_builder =
                    playlist_name_prompt.playlist_input_builder.clone();
                playlist_input_builder.name(playlist_name_prompt.answer.clone());
                playlist_input_builder
            };
            update_playlist(&self.context, playlist_input_builder.build());
        }
        Some(())
    }

    fn from_playlists(&self, playlists: &Playlists, key: &KeyEvent) -> Option<State> {
        let playlist = playlists.stateful_list().selected_item()?;
        match key.code {
            KeyCode::Enter => self.to_playlist_tracks(playlist),
            KeyCode::F(2) => self.to_playlist_name_prompt(playlist),
            KeyCode::Delete => self.to_confirm_prompt(playlist),
            _ => None,
        }
    }

    fn from_root(&self, root: &Root, key: &KeyEvent) -> Option<State> {
        let selected_item = root.stateful_list().selected_item()?;
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
        track_album_prompt: &TrackAlbumPrompt,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = self.undo.last()?.selected_track()?;
        let track_input_builder = {
            let mut track_input_builder = track_album_prompt.track_input_builder.clone();
            let stateful_list = track_album_prompt.stateful_list();
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
        self.to_track_number_prompt(track, track_input_builder)
    }

    fn from_track_artist_prompt(
        &self,
        track_artist_prompt: &TrackArtistPrompt,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = self.undo.last()?.selected_track()?;
        let track_input_builder = {
            let mut track_input_builder = track_artist_prompt.track_input_builder.clone();
            let stateful_list = track_artist_prompt.stateful_list();
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
        self.to_track_album_prompt(track, track_input_builder)
    }

    fn from_track_genre_prompt(
        &self,
        track_genre_prompt: &TrackGenrePrompt,
        key: &KeyEvent,
    ) -> Option<()> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = self.undo.last()?.selected_track()?;
        let track_input_builder = {
            let mut track_input_builder = track_genre_prompt.track_input_builder.clone();
            let stateful_list = track_genre_prompt.stateful_list();
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
        update_track(&self.context, track_input_builder.build());
        Some(())
    }

    fn from_track_name_prompt(
        &self,
        track_name_prompt: &TrackNamePrompt,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = self.undo.last()?.selected_track()?;
        let track_input_builder = {
            let mut track_input_builder = track_name_prompt.track_input_builder.clone();
            let track_name = track_name_prompt.answer.trim();
            if !track_name.is_empty() {
                track_input_builder.name(String::from(track_name));
            }
            track_input_builder
        };
        self.to_track_artist_prompt(track, track_input_builder)
    }

    fn from_track_number_prompt(
        &self,
        track_number_prompt: &TrackNumberPrompt,
        key: &KeyEvent,
    ) -> Option<State> {
        if key.code != KeyCode::Enter {
            return None;
        }
        let track = self.undo.last()?.selected_track()?;
        let track_input_builder = {
            let mut track_input_builder = track_number_prompt.track_input_builder.clone();
            let track_number = track_number_prompt.answer.trim();
            if !track_number.is_empty() {
                track_input_builder.track_number(Some(track_number.parse().unwrap()));
            }
            track_input_builder
        };
        self.to_track_genre_prompt(track, track_input_builder)
    }

    fn from_tracks(
        &self,
        has_tracks: &impl HasStatefulList<Item = Track>,
        key: &KeyEvent,
    ) -> Option<State> {
        let stateful_list = has_tracks.stateful_list();
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
            KeyCode::F(2) => self.to_track_name_prompt(stateful_list.selected_item()?),
            KeyCode::F(4) => self.to_add_to_playlist_prompt(),
            KeyCode::Delete => self.to_confirm_prompt(stateful_list.selected_item()?),
            _ => None,
        }
    }

    fn from_tracks_mut(&mut self, key: &KeyEvent) -> Option<State> {
        let tracks = if let State::Tracks(tracks) = &mut self.state {
            tracks
        } else {
            panic!()
        };
        let stateful_list = tracks.stateful_list_mut();
        match key {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::ALT,
            } => {
                if let Some(State::Playlists(playlists)) = self.undo.last() {
                    if let Some(playlist) = playlists.stateful_list().selected_item() {
                        if let Some(selected_index) = stateful_list.selected_index() {
                            if selected_index > 0 {
                                stateful_list.items = update_playlist_track(
                                    &self.context,
                                    playlist,
                                    selected_index,
                                    selected_index - 1,
                                );
                                stateful_list.previous();
                            }
                        }
                    }
                }
                None
            }
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::ALT,
            } => {
                if let Some(State::Playlists(playlists)) = self.undo.last() {
                    if let Some(playlist) = playlists.stateful_list().selected_item() {
                        if let Some(selected_index) = stateful_list.selected_index() {
                            if selected_index < stateful_list.items.len() - 1 {
                                stateful_list.items = update_playlist_track(
                                    &self.context,
                                    playlist,
                                    selected_index,
                                    selected_index + 2,
                                );
                                stateful_list.next();
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn refresh(&mut self) {
        let to = match &self.state {
            State::Albums(_) => self.to_albums(),
            State::AlbumTracks(album_tracks) => self.to_album_tracks(&album_tracks.album),
            State::ArtistAlbums(artist_albums) => self.to_artist_albums(&artist_albums.artist),
            State::Artists(_) => self.to_artists(),
            State::ArtistTracks(artist_tracks) => self.to_artist_tracks(&artist_tracks.artist),
            State::Genres(_) => self.to_genres(),
            State::GenreTracks(genre_tracks) => self.to_genre_tracks(&genre_tracks.genre),
            State::Playlists(_) => self.to_playlists(),
            State::PlaylistTracks(playlist_tracks) => {
                self.to_playlist_tracks(&playlist_tracks.playlist)
            }
            State::Tracks(_) => self.to_tracks(),
            _ => None,
        };
        if let Some(to) = to {
            self.state = to;
        }
    }

    fn to_add_to_playlist_prompt(&self) -> Option<State> {
        let playlists = read_playlists(&self.context);
        Some(State::AddToPlaylistPrompt(AddToPlaylistPrompt {
            prompt: String::from("Playlist name: "),
            stateful_list: StatefulList::builder()
                .items(playlists)
                .autocomplete(true)
                .build(),
        }))
    }

    fn to_album_name_prompt(&self, album: &Album) -> Option<State> {
        Some(State::AlbumNamePrompt(AlbumNamePrompt {
            prompt: format!("Album name: ({}) ", album.name()),
            answer: String::new(),
            album_input_builder: AlbumInputBuilder::new(album),
        }))
    }

    fn to_album_tracks(&self, album: &Album) -> Option<State> {
        if album.id > 0 {
            let tracks = read_tracks_of_album(&self.context, album);
            Some(State::AlbumTracks(AlbumTracks {
                album: album.clone(),
                stateful_list: StatefulList::builder().items(tracks).build(),
            }))
        } else {
            let artist = Artist {
                id: -album.id,
                name: album.name.clone(),
            };
            self.to_artist_tracks(&artist)
        }
    }

    fn to_albums(&self) -> Option<State> {
        let albums = read_albums(&self.context);
        Some(State::Albums(Albums {
            stateful_list: StatefulList::builder().items(albums).build(),
        }))
    }

    fn to_artist_albums(&self, artist: &Artist) -> Option<State> {
        let albums = {
            let mut albums = read_albums_of_artist(&self.context, artist);
            albums.insert(
                0,
                Album {
                    id: -artist.id,
                    name: String::from(ALL_TRACKS),
                },
            );
            albums
        };
        Some(State::ArtistAlbums(ArtistAlbums {
            artist: artist.clone(),
            stateful_list: StatefulList::builder().items(albums).build(),
        }))
    }

    fn to_artist_name_prompt(&self, artist: &Artist) -> Option<State> {
        Some(State::ArtistNamePrompt(ArtistNamePrompt {
            prompt: format!("Artist name: ({}) ", artist.name()),
            answer: String::new(),
            artist_input_builder: ArtistInputBuilder::new(artist),
        }))
    }

    fn to_artist_tracks(&self, artist: &Artist) -> Option<State> {
        let tracks = read_tracks_of_artist(&self.context, artist);
        Some(State::ArtistTracks(ArtistTracks {
            artist: artist.clone(),
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }

    fn to_artists(&self) -> Option<State> {
        let artists = read_artists(&self.context);
        Some(State::Artists(Artists {
            stateful_list: StatefulList::builder().items(artists).build(),
        }))
    }

    fn to_confirm_prompt(&self, id_name: &impl IdName) -> Option<State> {
        Some(State::ConfirmPrompt(ConfirmPrompt {
            prompt: format!("Delete {} ? (y/N) ", id_name.name()),
            answer: String::new(),
        }))
    }

    fn to_genre_name_prompt(&self, genre: &Genre) -> Option<State> {
        Some(State::GenreNamePrompt(GenreNamePrompt {
            prompt: format!("Genre name: ({}) ", genre.name()),
            answer: String::new(),
            genre_input_builder: GenreInputBuilder::new(genre),
        }))
    }

    fn to_genre_tracks(&self, genre: &Genre) -> Option<State> {
        let tracks = read_tracks_of_genre(&self.context, genre);
        Some(State::GenreTracks(GenreTracks {
            genre: genre.clone(),
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }

    fn to_genres(&self) -> Option<State> {
        let genres = read_genres(&self.context);
        Some(State::Genres(Genres {
            stateful_list: StatefulList::builder().items(genres).build(),
        }))
    }

    fn to_playlist_name_prompt(&self, playlist: &Playlist) -> Option<State> {
        Some(State::PlaylistNamePrompt(PlaylistNamePrompt {
            prompt: format!("Playlist name: ({}) ", playlist.name()),
            answer: String::new(),
            playlist_input_builder: PlaylistInputBuilder::new(playlist),
        }))
    }

    fn to_playlist_tracks(&self, playlist: &Playlist) -> Option<State> {
        let tracks = read_tracks_of_playlist(&self.context, playlist);
        Some(State::PlaylistTracks(PlaylistTracks {
            playlist: playlist.clone(),
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }

    fn to_playlists(&self) -> Option<State> {
        let playlists = read_playlists(&self.context);
        Some(State::Playlists(Playlists {
            stateful_list: StatefulList::builder().items(playlists).build(),
        }))
    }

    fn to_track_album_prompt(
        &self,
        track: &Track,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        let albums = read_albums(&self.context);
        Some(State::TrackAlbumPrompt(TrackAlbumPrompt {
            prompt: format!(
                "Album name: ({}) ",
                track.album.as_ref().map_or("", |album| album.name())
            ),
            stateful_list: StatefulList::builder()
                .items(albums)
                .autocomplete(true)
                .build(),
            track_input_builder,
        }))
    }

    fn to_track_artist_prompt(
        &self,
        track: &Track,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        let artists = read_artists(&self.context);
        Some(State::TrackArtistPrompt(TrackArtistPrompt {
            prompt: format!(
                "Artist name: ({}) ",
                track.artist.as_ref().map_or("", |artist| artist.name())
            ),
            stateful_list: StatefulList::builder()
                .items(artists)
                .autocomplete(true)
                .build(),
            track_input_builder,
        }))
    }

    fn to_track_genre_prompt(
        &self,
        track: &Track,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        let genres = read_genres(&self.context);
        Some(State::TrackGenrePrompt(TrackGenrePrompt {
            prompt: format!(
                "Genre name: ({}) ",
                track.genre.as_ref().map_or("", |genre| genre.name())
            ),
            stateful_list: StatefulList::builder()
                .items(genres)
                .autocomplete(true)
                .build(),
            track_input_builder,
        }))
    }

    fn to_track_name_prompt(&self, track: &Track) -> Option<State> {
        Some(State::TrackNamePrompt(TrackNamePrompt {
            prompt: format!("Track name: ({}) ", track.name()),
            answer: String::new(),
            track_input_builder: TrackInputBuilder::new(track),
        }))
    }

    fn to_track_number_prompt(
        &self,
        track: &Track,
        track_input_builder: TrackInputBuilder,
    ) -> Option<State> {
        Some(State::TrackNumberPrompt(TrackNumberPrompt {
            prompt: format!(
                "Track number: ({}) ",
                track
                    .track_number
                    .as_ref()
                    .map_or(String::new(), |track_number| track_number.to_string())
            ),
            answer: String::new(),
            track_input_builder,
        }))
    }

    fn to_tracks(&self) -> Option<State> {
        let tracks = read_tracks(&self.context);
        Some(State::Tracks(Tracks {
            stateful_list: StatefulList::builder().items(tracks).build(),
        }))
    }
}
