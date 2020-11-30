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

impl From<album_query::AlbumQueryAlbum> for Album {
    fn from(album_query::AlbumQueryAlbum { id, name }: album_query::AlbumQueryAlbum) -> Album {
        Album { id, name }
    }
}

impl From<album_tracks_query::AlbumTracksQueryAlbumTracks> for Track {
    fn from(
        album_tracks_query::AlbumTracksQueryAlbumTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<album_tracks_query::AlbumTracksQueryAlbumTracksAlbum> for Album {
    fn from(
        album_tracks_query::AlbumTracksQueryAlbumTracksAlbum { id, name }: album_tracks_query::AlbumTracksQueryAlbumTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<album_tracks_query::AlbumTracksQueryAlbumTracksArtist> for Artist {
    fn from(
        album_tracks_query::AlbumTracksQueryAlbumTracksArtist { id, name }: album_tracks_query::AlbumTracksQueryAlbumTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<album_tracks_query::AlbumTracksQueryAlbumTracksGenre> for Genre {
    fn from(
        album_tracks_query::AlbumTracksQueryAlbumTracksGenre { id, name }: album_tracks_query::AlbumTracksQueryAlbumTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<albums_query::AlbumsQueryAlbums> for Album {
    fn from(
        albums_query::AlbumsQueryAlbums { id, name }: albums_query::AlbumsQueryAlbums,
    ) -> Album {
        Album { id, name }
    }
}

impl From<update_album_mutation::UpdateAlbumMutationUpdateAlbum> for Album {
    fn from(
        update_album_mutation::UpdateAlbumMutationUpdateAlbum { id, name }: update_album_mutation::UpdateAlbumMutationUpdateAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<artist_albums_query::ArtistAlbumsQueryArtistAlbums> for Album {
    fn from(
        artist_albums_query::ArtistAlbumsQueryArtistAlbums { id, name }: artist_albums_query::ArtistAlbumsQueryArtistAlbums,
    ) -> Album {
        Album { id, name }
    }
}

impl From<artist_query::ArtistQueryArtist> for Artist {
    fn from(
        artist_query::ArtistQueryArtist { id, name }: artist_query::ArtistQueryArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracks> for Track {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracksAlbum> for Album {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracksAlbum { id, name }: artist_tracks_query::ArtistTracksQueryArtistTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracksArtist> for Artist {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracksArtist { id, name }: artist_tracks_query::ArtistTracksQueryArtistTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracksGenre> for Genre {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracksGenre { id, name }: artist_tracks_query::ArtistTracksQueryArtistTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<artists_query::ArtistsQueryArtists> for Artist {
    fn from(
        artists_query::ArtistsQueryArtists { id, name }: artists_query::ArtistsQueryArtists,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<update_artist_mutation::UpdateArtistMutationUpdateArtist> for Artist {
    fn from(
        update_artist_mutation::UpdateArtistMutationUpdateArtist { id, name }: update_artist_mutation::UpdateArtistMutationUpdateArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<genre_query::GenreQueryGenre> for Genre {
    fn from(genre_query::GenreQueryGenre { id, name }: genre_query::GenreQueryGenre) -> Genre {
        Genre { id, name }
    }
}

impl From<genre_tracks_query::GenreTracksQueryGenreTracks> for Track {
    fn from(
        genre_tracks_query::GenreTracksQueryGenreTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<genre_tracks_query::GenreTracksQueryGenreTracksAlbum> for Album {
    fn from(
        genre_tracks_query::GenreTracksQueryGenreTracksAlbum { id, name }: genre_tracks_query::GenreTracksQueryGenreTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<genre_tracks_query::GenreTracksQueryGenreTracksArtist> for Artist {
    fn from(
        genre_tracks_query::GenreTracksQueryGenreTracksArtist { id, name }: genre_tracks_query::GenreTracksQueryGenreTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<genre_tracks_query::GenreTracksQueryGenreTracksGenre> for Genre {
    fn from(
        genre_tracks_query::GenreTracksQueryGenreTracksGenre { id, name }: genre_tracks_query::GenreTracksQueryGenreTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<genres_query::GenresQueryGenres> for Genre {
    fn from(
        genres_query::GenresQueryGenres { id, name }: genres_query::GenresQueryGenres,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<update_genre_mutation::UpdateGenreMutationUpdateGenre> for Genre {
    fn from(
        update_genre_mutation::UpdateGenreMutationUpdateGenre { id, name }: update_genre_mutation::UpdateGenreMutationUpdateGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<create_album_mutation::CreateAlbumMutationCreateAlbum> for Album {
    fn from(
        create_album_mutation::CreateAlbumMutationCreateAlbum { id, name }: create_album_mutation::CreateAlbumMutationCreateAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<create_artist_mutation::CreateArtistMutationCreateArtist> for Artist {
    fn from(
        create_artist_mutation::CreateArtistMutationCreateArtist { id, name }: create_artist_mutation::CreateArtistMutationCreateArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<create_genre_mutation::CreateGenreMutationCreateGenre> for Genre {
    fn from(
        create_genre_mutation::CreateGenreMutationCreateGenre { id, name }: create_genre_mutation::CreateGenreMutationCreateGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<create_playlist_mutation::CreatePlaylistMutationCreatePlaylist> for Playlist {
    fn from(
        create_playlist_mutation::CreatePlaylistMutationCreatePlaylist { id, name }: create_playlist_mutation::CreatePlaylistMutationCreatePlaylist,
    ) -> Playlist {
        Playlist { id, name }
    }
}

impl From<playlist_tracks_query::PlaylistTracksQueryPlaylistTracks> for Track {
    fn from(
        playlist_tracks_query::PlaylistTracksQueryPlaylistTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<playlist_tracks_query::PlaylistTracksQueryPlaylistTracksAlbum> for Album {
    fn from(
        playlist_tracks_query::PlaylistTracksQueryPlaylistTracksAlbum { id, name }: playlist_tracks_query::PlaylistTracksQueryPlaylistTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<playlist_tracks_query::PlaylistTracksQueryPlaylistTracksArtist> for Artist {
    fn from(
        playlist_tracks_query::PlaylistTracksQueryPlaylistTracksArtist { id, name }: playlist_tracks_query::PlaylistTracksQueryPlaylistTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<playlist_tracks_query::PlaylistTracksQueryPlaylistTracksGenre> for Genre {
    fn from(
        playlist_tracks_query::PlaylistTracksQueryPlaylistTracksGenre { id, name }: playlist_tracks_query::PlaylistTracksQueryPlaylistTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<playlists_query::PlaylistsQueryPlaylists> for Playlist {
    fn from(
        playlists_query::PlaylistsQueryPlaylists { id, name }: playlists_query::PlaylistsQueryPlaylists,
    ) -> Playlist {
        Playlist { id, name }
    }
}

impl From<update_playlist_mutation::UpdatePlaylistMutationUpdatePlaylist> for Playlist {
    fn from(
        update_playlist_mutation::UpdatePlaylistMutationUpdatePlaylist { id, name }: update_playlist_mutation::UpdatePlaylistMutationUpdatePlaylist,
    ) -> Playlist {
        Playlist { id, name }
    }
}

impl From<create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracks>
    for Track
{
    fn from(
        create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksAlbum>
    for Album
{
    fn from(
        create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksAlbum { id, name }: create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl
    From<create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksArtist>
    for Artist
{
    fn from(
        create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksArtist { id, name }: create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksGenre>
    for Genre
{
    fn from(
        create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksGenre { id, name }: create_playlist_track_mutation::CreatePlaylistTrackMutationCreatePlaylistTrackTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<track_query::TrackQueryTrack> for Track {
    fn from(
        track_query::TrackQueryTrack {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<track_query::TrackQueryTrackAlbum> for Album {
    fn from(
        track_query::TrackQueryTrackAlbum { id, name }: track_query::TrackQueryTrackAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<track_query::TrackQueryTrackArtist> for Artist {
    fn from(
        track_query::TrackQueryTrackArtist { id, name }: track_query::TrackQueryTrackArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<track_query::TrackQueryTrackGenre> for Genre {
    fn from(
        track_query::TrackQueryTrackGenre { id, name }: track_query::TrackQueryTrackGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<tracks_query::TracksQueryTracks> for Track {
    fn from(
        tracks_query::TracksQueryTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<tracks_query::TracksQueryTracksAlbum> for Album {
    fn from(
        tracks_query::TracksQueryTracksAlbum { id, name }: tracks_query::TracksQueryTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<tracks_query::TracksQueryTracksArtist> for Artist {
    fn from(
        tracks_query::TracksQueryTracksArtist { id, name }: tracks_query::TracksQueryTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<tracks_query::TracksQueryTracksGenre> for Genre {
    fn from(
        tracks_query::TracksQueryTracksGenre { id, name }: tracks_query::TracksQueryTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks>
    for Track
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksAlbum>
    for Album
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksAlbum { id, name }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl
    From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksArtist>
    for Artist
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksArtist { id, name }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksGenre>
    for Genre
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksGenre { id, name }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracks>
    for Track
{
    fn from(
        delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracks {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksAlbum>
    for Album
{
    fn from(
        delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksAlbum { id, name }: delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl
    From<delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksArtist>
    for Artist
{
    fn from(
        delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksArtist { id, name }: delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksGenre>
    for Genre
{
    fn from(
        delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksGenre { id, name }: delete_playlist_track_mutation::DeletePlaylistTrackMutationDeletePlaylistTrackTracksGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<update_track_mutation::UpdateTrackMutationUpdateTrack> for Track {
    fn from(
        update_track_mutation::UpdateTrackMutationUpdateTrack {
            id,
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
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
            track_number,
        }
    }
}

impl From<update_track_mutation::UpdateTrackMutationUpdateTrackAlbum> for Album {
    fn from(
        update_track_mutation::UpdateTrackMutationUpdateTrackAlbum { id, name }: update_track_mutation::UpdateTrackMutationUpdateTrackAlbum,
    ) -> Album {
        Album { id, name }
    }
}

impl From<update_track_mutation::UpdateTrackMutationUpdateTrackArtist> for Artist {
    fn from(
        update_track_mutation::UpdateTrackMutationUpdateTrackArtist { id, name }: update_track_mutation::UpdateTrackMutationUpdateTrackArtist,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<update_track_mutation::UpdateTrackMutationUpdateTrackGenre> for Genre {
    fn from(
        update_track_mutation::UpdateTrackMutationUpdateTrackGenre { id, name }: update_track_mutation::UpdateTrackMutationUpdateTrackGenre,
    ) -> Genre {
        Genre { id, name }
    }
}

pub mod exports {
    pub use super::{
        album_query, album_tracks_query, albums_query, artist_albums_query, artist_query,
        artist_tracks_query, artists_query, create_album_mutation, create_artist_mutation,
        create_genre_mutation, create_playlist_mutation, create_playlist_track_mutation,
        delete_album_mutation, delete_artist_mutation, delete_genre_mutation,
        delete_playlist_mutation, delete_playlist_track_mutation, genre_query, genre_tracks_query,
        genres_query, playlist_tracks_query, playlists_query, track_query, tracks_query,
        update_album_mutation, update_artist_mutation, update_genre_mutation,
        update_playlist_mutation, update_playlist_track_mutation, update_track_mutation,
    };
}
