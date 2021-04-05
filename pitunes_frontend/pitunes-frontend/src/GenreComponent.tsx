// eslint-disable-next-line import/no-webpack-loader-syntax
import GenreTracksQuery from '!!raw-loader!./graphql/GenreTracksQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const GenreComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: GenreTracksQuery,
    operationName: 'GenreTracksQuery',
    variables: {
      id,
    },
  });

  return data ? (
    <>
      <TitleComponent title={data.genre.name} subtitle="Genre"></TitleComponent>
      {data.genre.tracks.length > 0 ? (
        <List>
          <TrackListItems
            tracks={data.genre.tracks}
            playlists={data.playlists ?? []}
          />
        </List>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
