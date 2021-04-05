// eslint-disable-next-line import/no-webpack-loader-syntax
import PlaylistTracksQuery from '!!raw-loader!./graphql/PlaylistTracksQuery.graphql';
import { List } from '@material-ui/core';
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

export const PlaylistComponent = () => {
  const { id } = useParams<{ id: string }>();
  const { fetcher } = useContext(AppContext);
  const { data, refresh } = useGraphQLData(fetcher, {
    query: PlaylistTracksQuery,
    operationName: 'PlaylistTracksQuery',
    variables: {
      id,
    },
  });

  return data ? (
    <>
      <TitleComponent
        title={data.playlist.name}
        subtitle="Playlist"
      ></TitleComponent>
      {data.playlist.tracks.length > 0 ? (
        <List>
          <TrackListItems
            tracks={data.playlist.tracks}
            playlists={data.playlists ?? []}
            playlist={data.playlist}
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
