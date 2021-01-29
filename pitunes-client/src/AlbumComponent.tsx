// eslint-disable-next-line import/no-webpack-loader-syntax
import AlbumTracksQuery from '!!raw-loader!./graphql/AlbumTracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { TrackListItems } from './TrackListItems';

type AlbumComponentProps = RouteComponentProps<{ id: string }>;

export const AlbumComponent = (props: AlbumComponentProps) => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: AlbumTracksQuery,
          operationName: 'AlbumTracksQuery',
          variables: {
            id: props.match.params.id,
          },
        }}
      >
        {(data: any) => (
          <List>
            <TrackListItems tracks={data.album.tracks} />
          </List>
        )}
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
