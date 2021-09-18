import { List } from '@material-ui/core';
import Fuse from 'fuse.js';
import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { Track } from './models';
import { range } from './range';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const AlbumComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { data, refresh } = useGraphQLData(API.albumTracks(id));
  const [pattern, setPattern] = useState('');
  const [trackFuse, setTrackFuse] = useState<Fuse<Track>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.album.tracks) {
        setTrackFuse(new Fuse(data.album.tracks, { keys: ['name'] }));
      }
    }
  }, [data]);
  const tracks = data?.album.tracks ?? [];
  const filteredTrackIndices =
    trackFuse !== undefined && pattern.length > 0
      ? trackFuse.search(pattern).map((result) => result.refIndex)
      : range(tracks.length);

  return data ? (
    <>
      <TitleComponent title={data.album.name} subtitle="Album"></TitleComponent>
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
