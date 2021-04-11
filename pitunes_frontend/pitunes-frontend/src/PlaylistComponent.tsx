import { List } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { EmptyListComponent } from './EmptyListComponent';
import { playlistTracks } from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const PlaylistComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { data, refresh } = useGraphQLData(playlistTracks(id));

  return data ? (
    <>
      <TitleComponent
        title={data.playlist.name}
        subtitle="Playlist"
      ></TitleComponent>
      {data.playlist.tracks.length > 0 ? (
        <List>
          <TrackListItems
            tracks={data.playlist.tracks}
            playlists={data.playlists ?? []}
            playlist={data.playlist}
            refresh={refresh}
          />
        </List>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
