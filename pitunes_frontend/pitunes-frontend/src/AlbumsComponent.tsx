import { List } from '@material-ui/core';
import React from 'react';
import { AlbumListItems } from './AlbumListItems';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const AlbumsComponent = () => {
  const { data, refresh } = useGraphQLData(API.albums());

  return data ? (
    <>
      <TitleComponent title="Albums"></TitleComponent>
      {data.albums && data.albums.length > 0 ? (
        <List>
          <AlbumListItems albums={data.albums} refresh={refresh} />
        </List>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
