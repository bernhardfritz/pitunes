use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/album_query.graphql",
    response_derives = "Debug"
)]
pub struct AlbumQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/album_tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct AlbumTracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/albums_query.graphql",
    response_derives = "Debug"
)]
pub struct AlbumsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artist_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artist_albums_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistAlbumsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artist_tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistTracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/artists_query.graphql",
    response_derives = "Debug"
)]
pub struct ArtistsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/create_album_mutation.graphql",
    response_derives = "Debug"
)]
pub struct CreateAlbumMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/create_artist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct CreateArtistMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/create_genre_mutation.graphql",
    response_derives = "Debug"
)]
pub struct CreateGenreMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/create_playlist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct CreatePlaylistMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/create_playlist_track_mutation.graphql",
    respoonse_derives = "Debug"
)]
pub struct CreatePlaylistTrackMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_album_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeleteAlbumMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_artist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeleteArtistMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_genre_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeleteGenreMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_playlist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeletePlaylistMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_playlist_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeletePlaylistTrackMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeleteTrackMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/genre_query.graphql",
    response_derives = "Debug"
)]
pub struct GenreQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/genre_tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct GenreTracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/genres_query.graphql",
    response_derives = "Debug"
)]
pub struct GenresQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/playlist_tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct PlaylistTracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/playlists_query.graphql",
    response_derives = "Debug"
)]
pub struct PlaylistsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/track_query.graphql",
    response_derives = "Debug"
)]
pub struct TrackQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/tracks_query.graphql",
    response_derives = "Debug"
)]
pub struct TracksQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_album_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdateAlbumMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_artist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdateArtistMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_genre_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdateGenreMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_playlist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdatePlaylistMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_playlist_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdatePlaylistTrackMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/update_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdateTrackMutation;

pub trait IdName {
    fn id(&self) -> i64;
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub struct Album {
    pub id: i64,
    pub name: String,
}

impl IdName for Album {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

#[derive(Clone)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

impl IdName for Artist {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

#[derive(Clone)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

impl IdName for Genre {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

#[derive(Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
}

impl IdName for Playlist {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

pub struct RootItem {
    pub name: String,
}

impl IdName for RootItem {
    fn id(&self) -> i64 {
        0
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

#[derive(Clone)]
pub struct Track {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub duration: i64,
    pub album: Option<Album>,
    pub artist: Option<Artist>,
    pub genre: Option<Genre>,
    pub track_number: Option<i64>,
}

impl IdName for Track {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
    }
}

#[derive(Clone)]
pub struct AlbumInputBuilder {
    id: i64,
    name: String,
}

impl AlbumInputBuilder {
    pub fn new(album: &Album) -> AlbumInputBuilder {
        AlbumInputBuilder {
            id: album.id,
            name: album.name.clone(),
        }
    }

    pub fn name(&mut self, name: String) -> &mut AlbumInputBuilder {
        self.name = name;
        self
    }

    pub fn build(&self) -> update_album_mutation::Variables {
        update_album_mutation::Variables {
            id: self.id,
            album_input: update_album_mutation::AlbumInput {
                name: self.name.clone(),
            },
        }
    }
}

#[derive(Clone)]
pub struct ArtistInputBuilder {
    id: i64,
    name: String,
}

impl ArtistInputBuilder {
    pub fn new(artist: &Artist) -> ArtistInputBuilder {
        ArtistInputBuilder {
            id: artist.id,
            name: artist.name.clone(),
        }
    }

    pub fn name(&mut self, name: String) -> &mut ArtistInputBuilder {
        self.name = name;
        self
    }

    pub fn build(&self) -> update_artist_mutation::Variables {
        update_artist_mutation::Variables {
            id: self.id,
            artist_input: update_artist_mutation::ArtistInput {
                name: self.name.clone(),
            },
        }
    }
}

#[derive(Clone)]
pub struct GenreInputBuilder {
    id: i64,
    name: String,
}

impl GenreInputBuilder {
    pub fn new(genre: &Genre) -> GenreInputBuilder {
        GenreInputBuilder {
            id: genre.id,
            name: genre.name.clone(),
        }
    }

    pub fn name(&mut self, name: String) -> &mut GenreInputBuilder {
        self.name = name;
        self
    }

    pub fn build(&self) -> update_genre_mutation::Variables {
        update_genre_mutation::Variables {
            id: self.id,
            genre_input: update_genre_mutation::GenreInput {
                name: self.name.clone(),
            },
        }
    }
}

#[derive(Clone)]
pub struct PlaylistInputBuilder {
    id: i64,
    name: String,
}

impl PlaylistInputBuilder {
    pub fn new(playlist: &Playlist) -> PlaylistInputBuilder {
        PlaylistInputBuilder {
            id: playlist.id,
            name: playlist.name.clone(),
        }
    }

    pub fn name(&mut self, name: String) -> &mut PlaylistInputBuilder {
        self.name = name;
        self
    }

    pub fn build(&self) -> update_playlist_mutation::Variables {
        update_playlist_mutation::Variables {
            id: self.id,
            playlist_input: update_playlist_mutation::PlaylistInput {
                name: self.name.clone(),
            },
        }
    }
}

#[derive(Clone)]
pub struct TrackInputBuilder {
    id: i64,
    name: String,
    album_id: Option<i64>,
    artist_id: Option<i64>,
    genre_id: Option<i64>,
    track_number: Option<i64>,
}

impl TrackInputBuilder {
    pub fn new(track: &Track) -> TrackInputBuilder {
        TrackInputBuilder {
            id: track.id,
            name: track.name.clone(),
            album_id: track.album.as_ref().map(|album| album.id),
            artist_id: track.artist.as_ref().map(|artist| artist.id),
            genre_id: track.genre.as_ref().map(|genre| genre.id),
            track_number: track.track_number,
        }
    }

    pub fn name(&mut self, name: String) -> &mut TrackInputBuilder {
        self.name = name;
        self
    }

    pub fn album_id(&mut self, album_id: Option<i64>) -> &mut TrackInputBuilder {
        self.album_id = album_id;
        self
    }

    pub fn artist_id(&mut self, artist_id: Option<i64>) -> &mut TrackInputBuilder {
        self.artist_id = artist_id;
        self
    }

    pub fn genre_id(&mut self, genre_id: Option<i64>) -> &mut TrackInputBuilder {
        self.genre_id = genre_id;
        self
    }

    pub fn track_number(&mut self, track_number: Option<i64>) -> &mut TrackInputBuilder {
        self.track_number = track_number;
        self
    }

    pub fn build(&self) -> update_track_mutation::Variables {
        update_track_mutation::Variables {
            id: self.id,
            track_input: update_track_mutation::TrackInput {
                name: self.name.clone(),
                album_id: self.album_id,
                artist_id: self.artist_id,
                genre_id: self.genre_id,
                track_number: self.track_number,
            },
        }
    }
}

macro_rules! impl_from {
    ($from: ty, $for: ty, $($field: ident),*) => {
        impl From<$from> for $for {
            fn from(from: $from) -> Self {
                Self {
                    $($field: from.$field),*
                }
            }
        }
    };
}

impl_from!(album_query::AlbumQueryAlbum, Album, id, name);
impl_from!(
    album_tracks_query::AlbumTracksQueryAlbumTracksAlbum,
    Album,
    id,
    name
);
impl_from!(albums_query::AlbumsQueryAlbums, Album, id, name);
impl_from!(
    artist_albums_query::ArtistAlbumsQueryArtistAlbums,
    Album,
    id,
    name
);
impl_from!(
    artist_tracks_query::ArtistTracksQueryArtistTracksAlbum,
    Album,
    id,
    name
);
impl_from!(
    create_album_mutation::CreateAlbumMutationCreateAlbum,
    Album,
    id,
    name
);
impl_from!(
    create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksAlbum,
    Album,
    id,
    name
);
impl_from!(
    genre_tracks_query::GenreTracksQueryGenreTracksAlbum,
    Album,
    id,
    name
);
impl_from!(
    playlist_tracks_query::PlaylistTracksQueryPlaylistTracksAlbum,
    Album,
    id,
    name
);
impl_from!(track_query::TrackQueryTrackAlbum, Album, id, name);
impl_from!(tracks_query::TracksQueryTracksAlbum, Album, id, name);
impl_from!(
    update_album_mutation::UpdateAlbumMutationUpdateAlbum,
    Album,
    id,
    name
);
impl_from!(
    delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksAlbum,
    Album,
    id,
    name
);
impl_from!(
    update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksAlbum,
    Album,
    id,
    name
);
impl_from!(
    update_track_mutation::UpdateTrackMutationUpdateTrackAlbum,
    Album,
    id,
    name
);

impl_from!(
    album_tracks_query::AlbumTracksQueryAlbumTracksArtist,
    Artist,
    id,
    name
);
impl_from!(artist_query::ArtistQueryArtist, Artist, id, name);
impl_from!(
    artist_tracks_query::ArtistTracksQueryArtistTracksArtist,
    Artist,
    id,
    name
);
impl_from!(artists_query::ArtistsQueryArtists, Artist, id, name);
impl_from!(
    create_artist_mutation::CreateArtistMutationCreateArtist,
    Artist,
    id,
    name
);
impl_from!(
    create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksArtist,
    Artist,
    id,
    name
);
impl_from!(
    delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksArtist,
    Artist,
    id,
    name
);
impl_from!(
    genre_tracks_query::GenreTracksQueryGenreTracksArtist,
    Artist,
    id,
    name
);
impl_from!(
    playlist_tracks_query::PlaylistTracksQueryPlaylistTracksArtist,
    Artist,
    id,
    name
);
impl_from!(track_query::TrackQueryTrackArtist, Artist, id, name);
impl_from!(tracks_query::TracksQueryTracksArtist, Artist, id, name);
impl_from!(
    update_artist_mutation::UpdateArtistMutationUpdateArtist,
    Artist,
    id,
    name
);
impl_from!(
    update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksArtist,
    Artist,
    id,
    name
);
impl_from!(
    update_track_mutation::UpdateTrackMutationUpdateTrackArtist,
    Artist,
    id,
    name
);

impl_from!(
    album_tracks_query::AlbumTracksQueryAlbumTracksGenre,
    Genre,
    id,
    name
);
impl_from!(
    artist_tracks_query::ArtistTracksQueryArtistTracksGenre,
    Genre,
    id,
    name
);
impl_from!(
    create_genre_mutation::CreateGenreMutationCreateGenre,
    Genre,
    id,
    name
);
impl_from!(
    create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksGenre,
    Genre,
    id,
    name
);
impl_from!(genre_query::GenreQueryGenre, Genre, id, name);
impl_from!(
    genre_tracks_query::GenreTracksQueryGenreTracksGenre,
    Genre,
    id,
    name
);
impl_from!(genres_query::GenresQueryGenres, Genre, id, name);
impl_from!(
    playlist_tracks_query::PlaylistTracksQueryPlaylistTracksGenre,
    Genre,
    id,
    name
);
impl_from!(track_query::TrackQueryTrackGenre, Genre, id, name);
impl_from!(tracks_query::TracksQueryTracksGenre, Genre, id, name);
impl_from!(
    update_genre_mutation::UpdateGenreMutationUpdateGenre,
    Genre,
    id,
    name
);
impl_from!(
    delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksGenre,
    Genre,
    id,
    name
);
impl_from!(
    update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksGenre,
    Genre,
    id,
    name
);
impl_from!(
    update_track_mutation::UpdateTrackMutationUpdateTrackGenre,
    Genre,
    id,
    name
);

impl_from!(
    create_playlist_mutation::CreatePlaylistMutationCreatePlaylist,
    Playlist,
    id,
    name
);
impl_from!(playlists_query::PlaylistsQueryPlaylists, Playlist, id, name);
impl_from!(
    update_playlist_mutation::UpdatePlaylistMutationUpdatePlaylist,
    Playlist,
    id,
    name
);

impl From<album_tracks_query::AlbumTracksQueryAlbumTracks> for Track {
    fn from(
        album_tracks_query::AlbumTracksQueryAlbumTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: album_tracks_query::AlbumTracksQueryAlbumTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracks> for Track {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: artist_tracks_query::ArtistTracksQueryArtistTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracks>
    for Track
{
    fn from(
        create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<genre_tracks_query::GenreTracksQueryGenreTracks> for Track {
    fn from(
        genre_tracks_query::GenreTracksQueryGenreTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: genre_tracks_query::GenreTracksQueryGenreTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<playlist_tracks_query::PlaylistTracksQueryPlaylistTracks> for Track {
    fn from(
        playlist_tracks_query::PlaylistTracksQueryPlaylistTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: playlist_tracks_query::PlaylistTracksQueryPlaylistTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<track_query::TrackQueryTrack> for Track {
    fn from(
        track_query::TrackQueryTrack {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: track_query::TrackQueryTrack,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<tracks_query::TracksQueryTracks> for Track {
    fn from(
        tracks_query::TracksQueryTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: tracks_query::TracksQueryTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracks>
    for Track
{
    fn from(
        delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks>
    for Track
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<update_track_mutation::UpdateTrackMutationUpdateTrack> for Track {
    fn from(
        update_track_mutation::UpdateTrackMutationUpdateTrack {
            id,
            uuid,
            name,
            duration,
            album,
            artist,
            genre,
            track_number,
        }: update_track_mutation::UpdateTrackMutationUpdateTrack,
    ) -> Track {
        Track {
            id,
            uuid,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

pub mod exports {
    pub use super::{
        album_query, album_tracks_query, albums_query, artist_albums_query, artist_query,
        artist_tracks_query, artists_query, create_album_mutation, create_artist_mutation,
        create_genre_mutation, create_playlist_mutation, create_playlist_track_mutation,
        delete_album_mutation, delete_artist_mutation, delete_genre_mutation,
        delete_playlist_mutation, delete_playlist_track_mutation, delete_track_mutation,
        genre_query, genre_tracks_query, genres_query, playlist_tracks_query, playlists_query,
        track_query, tracks_query, update_album_mutation, update_artist_mutation,
        update_genre_mutation, update_playlist_mutation, update_playlist_track_mutation,
        update_track_mutation,
    };
}
