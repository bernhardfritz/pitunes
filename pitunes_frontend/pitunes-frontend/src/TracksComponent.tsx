// eslint-disable-next-line import/no-webpack-loader-syntax
import TracksQuery from '!!raw-loader!./graphql/TracksQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { LoadingComponent } from './LoadingComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const TracksComponent = () => {
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: TracksQuery,
    operationName: 'TracksQuery',
  });

  return data ? (
    data.tracks ? (
      <List>
        <TrackListItems tracks={data.tracks} playlists={data.playlists ?? []} />
      </List>
    ) : (
      <EmptyListComponent />
    )
  ) : (
    <LoadingComponent />
  );
};
