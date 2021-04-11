import { List } from '@material-ui/core';
import React from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import { playlists } from './graphql/api';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const PlaylistsComponent = () => {
  const { data } = useGraphQLData(playlists());

  return data ? (
    <>
      <TitleComponent title="Playlists"></TitleComponent>
      {data.playlists && data.playlists.length > 0 ? (
        <List>
          <IdNameListItemLinks
            items={data.playlists}
            to={(id) => `/playlists/${id}`}
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
