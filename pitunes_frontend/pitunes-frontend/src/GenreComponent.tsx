// eslint-disable-next-line import/no-webpack-loader-syntax
import GenreTracksQuery from '!!raw-loader!./graphql/GenreTracksQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { GraphQLResource } from './GraphQLResource';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';

export const GenreComponent = () => {
  const { id } = useParams<{ id: string }>();

  return (
    <AppContext.Consumer>
      {({ fetcher }) => (
        <GraphQLResource
          fetcher={fetcher}
          fetcherParams={{
            query: GenreTracksQuery,
            operationName: 'GenreTracksQuery',
            variables: {
              id,
            },
          }}
        >
          {(data: any) => (
            <>
              <TitleComponent
                title={data.genre.name}
                subtitle="Genre"
              ></TitleComponent>
              {data.genre.tracks.length > 0 ? (
                <List>
                  <TrackListItems tracks={data.genre.tracks} />
                </List>
              ) : (
                <EmptyListComponent />
              )}
            </>
          )}
        </GraphQLResource>
      )}
    </AppContext.Consumer>
  );
};
