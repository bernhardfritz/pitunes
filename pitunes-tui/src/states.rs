use crate::{
    models::{Album, Artist, Genre, IdName, Playlist, RootItem, Track, TrackInputBuilder},
    util::stateful_list::StatefulList,
};

pub struct AlbumsState {
    pub stateful_list: StatefulList<Album>,
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
    pub stateful_list: StatefulList<Artist>,
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
    pub stateful_list: StatefulList<Genre>,
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
    pub stateful_list: StatefulList<Playlist>,
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
    pub prompt: String,
    pub answer: String,
}

impl HasPrompt for PromptState {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }
}

pub struct RootState {
    pub stateful_list: StatefulList<RootItem>,
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

pub struct TrackAlbumPromptState {
    pub prompt: String,
    pub stateful_list: StatefulList<Album>,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackAlbumPromptState {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        let stateful_list = self.stateful_list();
        if let Some(selected_item) = stateful_list.selected_item() {
            selected_item.name()
        } else {
            &self.stateful_list().pattern[..]
        }
    }
}

impl HasStatefulList for TrackAlbumPromptState {
    type Item = Album;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackArtistPromptState {
    pub prompt: String,
    pub stateful_list: StatefulList<Artist>,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackArtistPromptState {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        let stateful_list = self.stateful_list();
        if let Some(selected_item) = stateful_list.selected_item() {
            selected_item.name()
        } else {
            &self.stateful_list().pattern[..]
        }
    }
}

impl HasStatefulList for TrackArtistPromptState {
    type Item = Artist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackGenrePromptState {
    pub prompt: String,
    pub stateful_list: StatefulList<Genre>,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackGenrePromptState {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        let stateful_list = self.stateful_list();
        if let Some(selected_item) = stateful_list.selected_item() {
            selected_item.name()
        } else {
            &self.stateful_list().pattern[..]
        }
    }
}

impl HasStatefulList for TrackGenrePromptState {
    type Item = Genre;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackNumberPromptState {
    pub prompt: String,
    pub answer: String,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackNumberPromptState {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }
}

pub struct TracksState {
    pub stateful_list: StatefulList<Track>,
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

pub trait HasPrompt {
    fn prompt(&self) -> &str;
    fn answer(&self) -> &str;
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
    TrackAlbumPrompt(TrackAlbumPromptState),
    TrackArtistPrompt(TrackArtistPromptState),
    TrackGenrePrompt(TrackGenrePromptState),
    TrackNumberPrompt(TrackNumberPromptState),
    Tracks(TracksState),
}
