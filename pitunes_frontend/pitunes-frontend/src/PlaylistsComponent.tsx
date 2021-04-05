// eslint-disable-next-line import/no-webpack-loader-syntax
import PlaylistsQuery from '!!raw-loader!./graphql/PlaylistsQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { useGraphQLData } from './useGraphQLData';

export const PlaylistsComponent = () => {
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: PlaylistsQuery,
    operationName: 'PlaylistsQuery',
  });

  return data ? (
    data.playlists ? (
      <List>
        <IdNameListItemLinks
          items={data.playlists}
          to={(id) => `/playlists/${id}`}
        />
      </List>
    ) : (
      <EmptyListComponent />
    )
  ) : (
    <LoadingComponent />
  );
};
