import { List } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { EmptyListComponent } from './EmptyListComponent';
import { albumTracks } from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const AlbumComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { data } = useGraphQLData(albumTracks(id));

  return data ? (
    <>
      <TitleComponent title={data.album.name} subtitle="Album"></TitleComponent>
      {data.album.tracks.length > 0 ? (
        <List>
          <TrackListItems
            tracks={data.album.tracks}
            playlists={data.playlists ?? []}
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
