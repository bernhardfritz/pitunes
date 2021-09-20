import { List } from '@material-ui/core';
import React, { useState } from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const TracksComponent = () => {
  const { data, refresh } = useGraphQLData(API.tracks());
  const [pattern, setPattern] = useState('');
  const handleSearch = (pattern: string) => setPattern(pattern);

  return data ? (
    <>
      <TitleComponent title="Tracks"></TitleComponent>
      {data.tracks.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            <TrackListItems
              tracks={data.tracks}
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
