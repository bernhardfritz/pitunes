import { List } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { EmptyListComponent } from './EmptyListComponent';
import { genreTracks } from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const GenreComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { data } = useGraphQLData(genreTracks(id));

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
