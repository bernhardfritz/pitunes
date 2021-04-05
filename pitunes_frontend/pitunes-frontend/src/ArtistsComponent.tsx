// eslint-disable-next-line import/no-webpack-loader-syntax
import ArtistsQuery from '!!raw-loader!./graphql/ArtistsQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { useGraphQLData } from './useGraphQLData';

export const ArtistsComponent = () => {
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: ArtistsQuery,
    operationName: 'ArtistsQuery',
  });

  return data ? (
    data.artists ? (
      <List>
        <IdNameListItemLinks
          items={data.artists}
          to={(id) => `/artists/${id}`}
        />
      </List>
    ) : (
      <EmptyListComponent />
    )
  ) : (
    <LoadingComponent />
  );
};
