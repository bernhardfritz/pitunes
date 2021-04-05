// eslint-disable-next-line import/no-webpack-loader-syntax
import AlbumTracksQuery from '!!raw-loader!./graphql/AlbumTracksQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const AlbumComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: AlbumTracksQuery,
    operationName: 'AlbumTracksQuery',
    variables: {
      id,
    },
  });

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
