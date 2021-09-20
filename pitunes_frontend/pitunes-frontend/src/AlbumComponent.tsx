import { List } from '@material-ui/core';
import React, { useState } from 'react';
import { useParams } from 'react-router-dom';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const AlbumComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { data, refresh } = useGraphQLData(API.albumTracks(id));
  const [pattern, setPattern] = useState('');
  const handleSearch = (pattern: string) => setPattern(pattern);

  return data ? (
    <>
      <TitleComponent title={data.album.name} subtitle="Album"></TitleComponent>
      {data.album.tracks && data.album.tracks.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            <TrackListItems
              tracks={data.album.tracks}
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
