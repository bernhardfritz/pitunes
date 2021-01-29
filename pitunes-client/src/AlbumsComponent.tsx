// eslint-disable-next-line import/no-webpack-loader-syntax
import AlbumsQuery from '!!raw-loader!./graphql/AlbumsQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { IdNameListItemLinks } from './IdNameListItemLinks';

export const AlbumsComponent = () => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: AlbumsQuery,
          operationName: 'AlbumsQuery',
        }}
      >
        {(data: any) => (
          <List>
            <IdNameListItemLinks
              items={data.albums}
              to={(id) => `/albums/${id}`}
            />
          </List>
        )}
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
