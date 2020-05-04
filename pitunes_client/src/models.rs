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
    query_path = "src/graphql/albums_query.graphql",
    response_derives = "Debug"
)]
pub struct AlbumsQuery;

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
    query_path = "src/graphql/genre_query.graphql",
    response_derives = "Debug"
)]
pub struct GenreQuery;

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
    query_path = "src/graphql/playlist_query.graphql",
    response_derives = "Debug"
)]
pub struct PlaylistQuery;

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
    query_path = "src/graphql/update_playlist_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct UpdatePlaylistTrackMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/delete_playlist_track_mutation.graphql",
    response_derives = "Debug"
)]
pub struct DeletePlaylistTrackMutation;

#[derive(Clone)]
pub struct Album {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct Track {
    pub id: i64,
    pub name: String,
}

impl From<album_query::AlbumQueryAlbumTracks> for Track {
    fn from(
        album_query::AlbumQueryAlbumTracks { id, name }: album_query::AlbumQueryAlbumTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<albums_query::AlbumsQueryAlbums> for Album {
    fn from(
        albums_query::AlbumsQueryAlbums { id, name }: albums_query::AlbumsQueryAlbums,
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

impl From<artists_query::ArtistsQueryArtists> for Artist {
    fn from(
        artists_query::ArtistsQueryArtists { id, name }: artists_query::ArtistsQueryArtists,
    ) -> Artist {
        Artist { id, name }
    }
}

impl From<artist_tracks_query::ArtistTracksQueryArtistTracks> for Track {
    fn from(
        artist_tracks_query::ArtistTracksQueryArtistTracks { id, name }: artist_tracks_query::ArtistTracksQueryArtistTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<genre_query::GenreQueryGenreTracks> for Track {
    fn from(
        genre_query::GenreQueryGenreTracks { id, name }: genre_query::GenreQueryGenreTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<genres_query::GenresQueryGenres> for Genre {
    fn from(
        genres_query::GenresQueryGenres { id, name }: genres_query::GenresQueryGenres,
    ) -> Genre {
        Genre { id, name }
    }
}

impl From<playlist_query::PlaylistQueryPlaylistTracks> for Track {
    fn from(
        playlist_query::PlaylistQueryPlaylistTracks { id, name }: playlist_query::PlaylistQueryPlaylistTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<playlists_query::PlaylistsQueryPlaylists> for Playlist {
    fn from(
        playlists_query::PlaylistsQueryPlaylists { id, name }: playlists_query::PlaylistsQueryPlaylists,
    ) -> Playlist {
        Playlist { id, name }
    }
}

impl From<tracks_query::TracksQueryTracks> for Track {
    fn from(
        tracks_query::TracksQueryTracks { id, name }: tracks_query::TracksQueryTracks,
    ) -> Track {
        Track { id, name }
    }
}

impl From<update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks>
    for Track
{
    fn from(
        update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks { id, name }: update_playlist_track_mutation::UpdatePlaylistTrackMutationUpdatePlaylistTrackTracks,
    ) -> Track {
        Track { id, name }
    }
}

pub mod exports {
    pub use super::album_query;
    pub use super::albums_query;
    pub use super::artist_albums_query;
    pub use super::artist_tracks_query;
    pub use super::artists_query;
    pub use super::delete_playlist_track_mutation;
    pub use super::genre_query;
    pub use super::genres_query;
    pub use super::playlist_query;
    pub use super::playlists_query;
    pub use super::track_query;
    pub use super::tracks_query;
    pub use super::update_playlist_track_mutation;
}
