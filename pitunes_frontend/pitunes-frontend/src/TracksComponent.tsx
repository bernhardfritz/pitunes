import { List } from '@material-ui/core';
import React from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const TracksComponent = () => {
  const { data, refresh } = useGraphQLData(API.tracks());

  return data ? (
    <>
      <TitleComponent title="Tracks"></TitleComponent>
      {data.tracks && data.tracks.length > 0 ? (
        <List>
          <TrackListItems
            tracks={data.tracks}
            refresh={refresh}
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
