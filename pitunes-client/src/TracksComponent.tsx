// eslint-disable-next-line import/no-webpack-loader-syntax
import TracksQuery from '!!raw-loader!./graphql/TracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { TrackListItems } from './TrackListItems';

export const TracksComponent = () => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: TracksQuery,
          operationName: 'TracksQuery',
        }}
      >
        {(data: any) => (
          <List>
            <TrackListItems tracks={data.tracks} />
          </List>
        )}
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
