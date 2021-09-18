import { List } from '@material-ui/core';
import Fuse from 'fuse.js';
import React, { useEffect, useState } from 'react';
import { AlbumListItems } from './AlbumListItems';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { Album } from './models';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const AlbumsComponent = () => {
  const { data, refresh } = useGraphQLData(API.albums());

  const [pattern, setPattern] = useState('');
  const [albumFuse, setAlbumFuse] = useState<Fuse<Album>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.albums) {
        setAlbumFuse(new Fuse(data.albums, { keys: ['name'] }));
      }
    }
  }, [data]);
  const albums = data?.albums ?? [];
  const filteredAlbums =
    albumFuse !== undefined && pattern.length > 0
      ? albumFuse.search(pattern).map((result) => result.item)
      : albums;

  return data ? (
    <>
      <TitleComponent title="Albums"></TitleComponent>
      {albums.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            <AlbumListItems albums={filteredAlbums} refresh={refresh} />
          </List>
        </>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
