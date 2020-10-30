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
    // todo change Track to be FullTrack and remove FullTrack
    pub id: i64,
    pub name: String,
    pub duration: i64,
    pub album: Option<Album>,
    pub artist: Option<Artist>,
    pub genre: Option<Genre>,
}

impl IdName for Track {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name[..]
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
        }: album_tracks_query::AlbumTracksQueryAlbumTracks,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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
        }: artist_tracks_query::ArtistTracksQueryArtistTracks,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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
        }: genre_tracks_query::GenreTracksQueryGenreTracks,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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
        }: playlist_tracks_query::PlaylistTracksQueryPlaylistTracks,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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

impl From<track_query::TrackQueryTrack> for Track {
    fn from(
        track_query::TrackQueryTrack {
            id,
            name,
            duration,
            album,
            artist,
            genre,
        }: track_query::TrackQueryTrack,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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
        }: tracks_query::TracksQueryTracks,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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
        }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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

impl From<update_track_mutation::UpdateTrackMutationUpdateTrack> for Track {
    fn from(
        update_track_mutation::UpdateTrackMutationUpdateTrack {
            id,
            name,
            duration,
            album,
            artist,
            genre,
        }: update_track_mutation::UpdateTrackMutationUpdateTrack,
    ) -> Track {
        Track {
            id,
            name,
            duration,
            album: album.map(|album| album.into()),
            artist: artist.map(|artist| artist.into()),
            genre: genre.map(|genre| genre.into()),
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
        create_genre_mutation, create_playlist_mutation, delete_playlist_mutation,
        delete_playlist_track_mutation, genre_query, genre_tracks_query, genres_query,
        playlist_tracks_query, playlists_query, track_query, tracks_query, update_album_mutation,
        update_artist_mutation, update_genre_mutation, update_playlist_mutation,
        update_playlist_track_mutation, update_track_mutation,
    };
}
