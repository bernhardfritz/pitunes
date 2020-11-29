use crate::{
    models::{
        Album, AlbumInputBuilder, Artist, ArtistInputBuilder, Genre, GenreInputBuilder, IdName,
        Playlist, PlaylistInputBuilder, RootItem, Track, TrackInputBuilder,
    },
    util::stateful_list::StatefulList,
};

pub struct AddToPlaylistPrompt {
    pub prompt: String,
    pub stateful_list: StatefulList<Playlist>,
}

impl HasPrompt for AddToPlaylistPrompt {
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

    fn answer_mut(&mut self) -> &mut String {
        unimplemented!()
    }
}

impl HasStatefulList for AddToPlaylistPrompt {
    type Item = Playlist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct AlbumNamePrompt {
    pub prompt: String,
    pub answer: String,
    pub album_input_builder: AlbumInputBuilder,
}

impl HasPrompt for AlbumNamePrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct AlbumTracks {
    pub album: Album,
    pub stateful_list: StatefulList<Track>,
}

impl HasStatefulList for AlbumTracks {
    type Item = Track;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct Albums {
    pub stateful_list: StatefulList<Album>,
}

impl HasStatefulList for Albums {
    type Item = Album;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct ArtistAlbums {
    pub artist: Artist,
    pub stateful_list: StatefulList<Album>,
}

impl HasStatefulList for ArtistAlbums {
    type Item = Album;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct ArtistNamePrompt {
    pub prompt: String,
    pub answer: String,
    pub artist_input_builder: ArtistInputBuilder,
}

impl HasPrompt for ArtistNamePrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct Artists {
    pub stateful_list: StatefulList<Artist>,
}

impl HasStatefulList for Artists {
    type Item = Artist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct ArtistTracks {
    pub artist: Artist,
    pub stateful_list: StatefulList<Track>,
}

impl HasStatefulList for ArtistTracks {
    type Item = Track;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct ConfirmPrompt {
    pub prompt: String,
    pub answer: String,
}

impl HasPrompt for ConfirmPrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct GenreNamePrompt {
    pub prompt: String,
    pub answer: String,
    pub genre_input_builder: GenreInputBuilder,
}

impl HasPrompt for GenreNamePrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct Genres {
    pub stateful_list: StatefulList<Genre>,
}

impl HasStatefulList for Genres {
    type Item = Genre;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct GenreTracks {
    pub genre: Genre,
    pub stateful_list: StatefulList<Track>,
}

impl HasStatefulList for GenreTracks {
    type Item = Track;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct PlaylistNamePrompt {
    pub prompt: String,
    pub answer: String,
    pub playlist_input_builder: PlaylistInputBuilder,
}

impl HasPrompt for PlaylistNamePrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct Playlists {
    pub stateful_list: StatefulList<Playlist>,
}

impl HasStatefulList for Playlists {
    type Item = Playlist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct PlaylistTracks {
    pub playlist: Playlist,
    pub stateful_list: StatefulList<Track>,
}

impl HasStatefulList for PlaylistTracks {
    type Item = Track;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct Root {
    pub stateful_list: StatefulList<RootItem>,
}

impl From<&str> for RootItem {
    fn from(name: &str) -> Self {
        RootItem {
            name: String::from(name),
        }
    }
}

impl HasStatefulList for Root {
    type Item = RootItem;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackAlbumPrompt {
    pub prompt: String,
    pub stateful_list: StatefulList<Album>,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackAlbumPrompt {
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

    fn answer_mut(&mut self) -> &mut String {
        unimplemented!()
    }
}

impl HasStatefulList for TrackAlbumPrompt {
    type Item = Album;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackArtistPrompt {
    pub prompt: String,
    pub stateful_list: StatefulList<Artist>,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackArtistPrompt {
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

    fn answer_mut(&mut self) -> &mut String {
        unimplemented!()
    }
}

impl HasStatefulList for TrackArtistPrompt {
    type Item = Artist;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackGenrePrompt {
    pub prompt: String,
    pub stateful_list: StatefulList<Genre>,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackGenrePrompt {
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

    fn answer_mut(&mut self) -> &mut String {
        unimplemented!()
    }
}

impl HasStatefulList for TrackGenrePrompt {
    type Item = Genre;

    fn stateful_list(&self) -> &StatefulList<Self::Item> {
        &self.stateful_list
    }

    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item> {
        &mut self.stateful_list
    }
}

pub struct TrackNamePrompt {
    pub prompt: String,
    pub answer: String,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackNamePrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct TrackNumberPrompt {
    pub prompt: String,
    pub answer: String,
    pub track_input_builder: TrackInputBuilder,
}

impl HasPrompt for TrackNumberPrompt {
    fn prompt(&self) -> &str {
        &self.prompt[..]
    }

    fn answer(&self) -> &str {
        &self.answer[..]
    }

    fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}

pub struct Tracks {
    pub stateful_list: StatefulList<Track>,
}

impl HasStatefulList for Tracks {
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
    fn answer_mut(&mut self) -> &mut String;
}

pub trait HasStatefulList {
    type Item: IdName;

    fn stateful_list(&self) -> &StatefulList<Self::Item>;
    fn stateful_list_mut(&mut self) -> &mut StatefulList<Self::Item>;
}

pub enum State {
    AddToPlaylistPrompt(AddToPlaylistPrompt),
    AlbumNamePrompt(AlbumNamePrompt),
    Albums(Albums),
    AlbumTracks(AlbumTracks),
    ArtistAlbums(ArtistAlbums),
    ArtistNamePrompt(ArtistNamePrompt),
    Artists(Artists),
    ArtistTracks(ArtistTracks),
    ConfirmPrompt(ConfirmPrompt),
    GenreNamePrompt(GenreNamePrompt),
    Genres(Genres),
    GenreTracks(GenreTracks),
    PlaylistNamePrompt(PlaylistNamePrompt),
    Playlists(Playlists),
    PlaylistTracks(PlaylistTracks),
    Root(Root),
    TrackAlbumPrompt(TrackAlbumPrompt),
    TrackArtistPrompt(TrackArtistPrompt),
    TrackGenrePrompt(TrackGenrePrompt),
    TrackNamePrompt(TrackNamePrompt),
    TrackNumberPrompt(TrackNumberPrompt),
    Tracks(Tracks),
}

impl State {
    pub fn selected_track(&self) -> Option<&Track> {
        fn extracted(has_tracks: &impl HasStatefulList<Item = Track>) -> Option<&Track> {
            has_tracks.stateful_list().selected_item()
        }
        match self {
            State::AlbumTracks(album_tracks) => extracted(album_tracks),
            State::ArtistTracks(artist_tracks) => extracted(artist_tracks),
            State::GenreTracks(genre_tracks) => extracted(genre_tracks),
            State::PlaylistTracks(playlist_tracks) => extracted(playlist_tracks),
            State::Tracks(tracks) => extracted(tracks),
            _ => None,
        }
    }
}
