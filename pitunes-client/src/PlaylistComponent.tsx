// eslint-disable-next-line import/no-webpack-loader-syntax
import PlaylistTracksQuery from '!!raw-loader!./graphql/PlaylistTracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { TrackListItems } from './TrackListItems';

type PlaylistComponentProps = RouteComponentProps<{ id: string }>;

export const PlaylistComponent = (props: PlaylistComponentProps) => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: PlaylistTracksQuery,
          operationName: 'PlaylistTracksQuery',
          variables: {
            id: props.match.params.id,
          },
        }}
      >
        {(data: any) => (
          <List>
            <TrackListItems tracks={data.playlist.tracks} />
          </List>
        )}
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
