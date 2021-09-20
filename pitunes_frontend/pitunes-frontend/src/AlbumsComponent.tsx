import { List } from '@material-ui/core';
import React, { useState } from 'react';
import { AlbumListItems } from './AlbumListItems';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const AlbumsComponent = () => {
  const { data, refresh } = useGraphQLData(API.albums());
  const [pattern, setPattern] = useState('');
  const handleSearch = (pattern: string) => setPattern(pattern);

  return data ? (
    <>
      <TitleComponent title="Albums"></TitleComponent>
      {data.albums ?? data.albums.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            <AlbumListItems
              albums={data.albums}
              pattern={pattern}
              refresh={refresh}
            />
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
