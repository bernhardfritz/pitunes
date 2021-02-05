// eslint-disable-next-line import/no-webpack-loader-syntax
import PlaylistTracksQuery from '!!raw-loader!./graphql/PlaylistTracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { TrackListItems } from './TrackListItems';

export const PlaylistComponent = () => {
  const { id } = useParams<{ id: string }>();

  return (
    <AppContext.Consumer>
      {({ fetcher }) => (
        <GraphQLResource
          fetcher={fetcher}
          fetcherParams={{
            query: PlaylistTracksQuery,
            operationName: 'PlaylistTracksQuery',
            variables: {
              id,
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
};
