// eslint-disable-next-line import/no-webpack-loader-syntax
import PlaylistsQuery from '!!raw-loader!./graphql/PlaylistsQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { IdNameListItemLinks } from './IdNameListItemLinks';

export const PlaylistsComponent = () => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: PlaylistsQuery,
          operationName: 'PlaylistsQuery',
        }}
      >
        {(data: any) => (
          <List>
            <IdNameListItemLinks
              items={data.playlists}
              to={(id) => `/playlists/${id}`}
            />
          </List>
        )}
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
