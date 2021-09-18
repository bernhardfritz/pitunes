import { List } from '@material-ui/core';
import Fuse from 'fuse.js';
import React, { useEffect, useState } from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { Track } from './models';
import { range } from './range';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const TracksComponent = () => {
  const { data, refresh } = useGraphQLData(API.tracks());
  const [pattern, setPattern] = useState('');
  const [trackFuse, setTrackFuse] = useState<Fuse<Track>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.tracks) {
        setTrackFuse(new Fuse(data.tracks, { keys: ['name'] }));
      }
    }
  }, [data]);
  const tracks = data?.tracks ?? [];
  const filteredTrackIndices =
    trackFuse !== undefined && pattern.length > 0
      ? trackFuse.search(pattern).map((result) => result.refIndex)
      : range(tracks.length);

  return data ? (
    <>
      <TitleComponent title="Tracks"></TitleComponent>
      {tracks.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            <TrackListItems
              tracks={tracks}
              filteredTrackIndices={filteredTrackIndices}
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
