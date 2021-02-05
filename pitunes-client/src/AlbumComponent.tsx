// eslint-disable-next-line import/no-webpack-loader-syntax
import AlbumTracksQuery from '!!raw-loader!./graphql/AlbumTracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { TrackListItems } from './TrackListItems';

export const AlbumComponent = () => {
  const { id } = useParams<{ id: string }>();

  return (
    <AppContext.Consumer>
      {({ fetcher }) => (
        <GraphQLResource
          fetcher={fetcher}
          fetcherParams={{
            query: AlbumTracksQuery,
            operationName: 'AlbumTracksQuery',
            variables: {
              id,
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
};
