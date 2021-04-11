/* eslint-disable import/no-webpack-loader-syntax */
import AlbumsQuery from '!!raw-loader!./AlbumsQuery.graphql';
import AlbumTracksQuery from '!!raw-loader!./AlbumTracksQuery.graphql';
import ArtistQuery from '!!raw-loader!./ArtistQuery.graphql';
import ArtistsQuery from '!!raw-loader!./ArtistsQuery.graphql';
import CreatePlaylistMutation from '!!raw-loader!./CreatePlaylistMutation.graphql';
import CreatePlaylistTrackMutation from '!!raw-loader!./CreatePlaylistTrackMutation.graphql';
import DeletePlaylistTrackMutation from '!!raw-loader!./DeletePlaylistTrackMutation.graphql';
import GenresQuery from '!!raw-loader!./GenresQuery.graphql';
import GenreTracksQuery from '!!raw-loader!./GenreTracksQuery.graphql';
import PlaylistsQuery from '!!raw-loader!./PlaylistsQuery.graphql';
import PlaylistTracksQuery from '!!raw-loader!./PlaylistTracksQuery.graphql';
import TrackQuery from '!!raw-loader!./TrackQuery.graphql';
import TracksQuery from '!!raw-loader!./TracksQuery.graphql';
/* eslint-enable import/no-webpack-loader-syntax */
import { FetcherParams } from 'graphiql/dist/components/GraphiQL';

type FetcherResult = { data: any };

type Fetcher = (graphQLParams: FetcherParams) => Promise<FetcherResult>;

export const fetcher: Fetcher = (graphQLParams: FetcherParams) =>
  fetch('/api/graphql', {
    method: 'post',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(graphQLParams),
  }).then((response) => response.json());

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
