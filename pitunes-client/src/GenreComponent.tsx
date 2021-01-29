// eslint-disable-next-line import/no-webpack-loader-syntax
import GenreTracksQuery from '!!raw-loader!./graphql/GenreTracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { TrackListItems } from './TrackListItems';

type GenreComponentProps = RouteComponentProps<{ id: string }>;

export const GenreComponent = (props: GenreComponentProps) => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: GenreTracksQuery,
          operationName: 'GenreTracksQuery',
          variables: {
            id: props.match.params.id,
          },
        }}
      >
        {(data: any) => (
          <List>
            <TrackListItems tracks={data.genre.tracks} />
          </List>
        )}
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
