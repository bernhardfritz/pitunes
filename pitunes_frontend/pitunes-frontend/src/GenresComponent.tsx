// eslint-disable-next-line import/no-webpack-loader-syntax
import GenresQuery from '!!raw-loader!./graphql/GenresQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { useGraphQLData } from './useGraphQLData';

export const GenresComponent = () => {
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: GenresQuery,
    operationName: 'GenresQuery',
  });

  return data ? (
    data.genres ? (
      <List>
        <IdNameListItemLinks items={data.genres} to={(id) => `/genres/${id}`} />
      </List>
    ) : (
      <EmptyListComponent />
    )
  ) : (
    <LoadingComponent />
  );
};
