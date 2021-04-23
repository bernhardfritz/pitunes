/* eslint-disable import/no-webpack-loader-syntax */
import AlbumsQuery from '!!raw-loader!./AlbumsQuery.graphql';
import AlbumTracksQuery from '!!raw-loader!./AlbumTracksQuery.graphql';
import ArtistQuery from '!!raw-loader!./ArtistQuery.graphql';
import ArtistsQuery from '!!raw-loader!./ArtistsQuery.graphql';
import CreatePlaylistMutation from '!!raw-loader!./CreatePlaylistMutation.graphql';
import CreatePlaylistTrackMutation from '!!raw-loader!./CreatePlaylistTrackMutation.graphql';
import DeletePlaylistMutation from '!!raw-loader!./DeletePlaylistMutation.graphql';
import DeletePlaylistTrackMutation from '!!raw-loader!./DeletePlaylistTrackMutation.graphql';
import DeleteTrackMutation from '!!raw-loader!./DeleteTrackMutation.graphql';
import GenresQuery from '!!raw-loader!./GenresQuery.graphql';
import GenreTracksQuery from '!!raw-loader!./GenreTracksQuery.graphql';
import PlaylistsQuery from '!!raw-loader!./PlaylistsQuery.graphql';
import PlaylistTracksQuery from '!!raw-loader!./PlaylistTracksQuery.graphql';
import TrackQuery from '!!raw-loader!./TrackQuery.graphql';
import TracksQuery from '!!raw-loader!./TracksQuery.graphql';
import UpdatePlaylistMutation from '!!raw-loader!./UpdatePlaylistMutation.graphql';
import UpdateTrackMutation from '!!raw-loader!./UpdateTrackMutation.graphql';
/* eslint-enable import/no-webpack-loader-syntax */

export const albums = () => ({
  query: AlbumsQuery,
  operationName: 'AlbumsQuery',
});

export const albumTracks = (id: string) => ({
  query: AlbumTracksQuery,
  operationName: 'AlbumTracksQuery',
  variables: {
    id,
  },
});

export const artist = (id: string) => ({
  query: ArtistQuery,
  operationName: 'ArtistQuery',
  variables: {
    id,
  },
});

export const artists = () => ({
  query: ArtistsQuery,
  operationName: 'ArtistsQuery',
});

export const createPlaylist = (name: string) => ({
  query: CreatePlaylistMutation,
  operationName: 'CreatePlaylistMutation',
  variables: {
    input: {
      name,
    },
  },
});

export const createPlaylistTrack = (
  playlistId: string,
  trackId: string,
  position?: number
) => ({
  query: CreatePlaylistTrackMutation,
  operationName: 'CreatePlaylistTrackMutation',
  variables: {
    id: playlistId,
    input: {
      id: trackId,
      position,
    },
  },
});

export const deletePlaylist = (id: string) => ({
  query: DeletePlaylistMutation,
  operationName: 'DeletePlaylistMutation',
  variables: {
    id,
  },
});

export const deletePlaylistTrack = (
  playlistId: string,
  trackId: string,
  position?: number
) => ({
  query: DeletePlaylistTrackMutation,
  operationName: 'DeletePlaylistTrackMutation',
  variables: {
    id: playlistId,
    input: {
      id: trackId,
      position,
    },
  },
});

export const deleteTrack = (id: string) => ({
  query: DeleteTrackMutation,
  operationName: 'DeleteTrackMutation',
  variables: {
    id,
  },
});

export const genres = () => ({
  query: GenresQuery,
  operationName: 'GenresQuery',
});

export const genreTracks = (id: string) => ({
  query: GenreTracksQuery,
  operationName: 'GenreTracksQuery',
  variables: {
    id,
  },
});

export const playlists = () => ({
  query: PlaylistsQuery,
  operationName: 'PlaylistsQuery',
});

export const playlistTracks = (id: string) => ({
  query: PlaylistTracksQuery,
  operationName: 'PlaylistTracksQuery',
  variables: {
    id,
  },
});

export const track = (id: string) => ({
  query: TrackQuery,
  operationName: 'TrackQuery',
  variables: {
    id,
  },
});

export const tracks = () => ({
  query: TracksQuery,
  operationName: 'TracksQuery',
});

export const updatePlaylist = (id: string, name: string) => ({
  query: UpdatePlaylistMutation,
  operationName: 'UpdatePlaylistMutation',
  variables: {
    id,
    input: {
      name,
    },
  },
});

export const updateTrack = (
  id: string,
  name: string,
  albumId?: string,
  artistId?: string,
  genreId?: string,
  trackNumber?: number
) => ({
  query: UpdateTrackMutation,
  operationName: 'UpdateTrackMutation',
  variables: {
    id,
    input: {
      name,
      albumId,
      artistId,
      genreId,
      trackNumber,
    },
  },
});
