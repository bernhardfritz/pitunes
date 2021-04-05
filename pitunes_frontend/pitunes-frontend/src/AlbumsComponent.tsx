// eslint-disable-next-line import/no-webpack-loader-syntax
import AlbumsQuery from '!!raw-loader!./graphql/AlbumsQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { useGraphQLData } from './useGraphQLData';

export const AlbumsComponent = () => {
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: AlbumsQuery,
    operationName: 'AlbumsQuery',
  });

  return data ? (
    data.albums ? (
      <List>
        <IdNameListItemLinks items={data.albums} to={(id) => `/albums/${id}`} />
      </List>
    ) : (
      <EmptyListComponent />
    )
  ) : (
    <LoadingComponent />
  );
};
