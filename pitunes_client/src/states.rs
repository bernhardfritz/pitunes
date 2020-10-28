use crate::{
    models::{Album, Artist, FullTrack, Genre, IdName, Playlist, RootItem, Track},
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
    pub message: String,
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
