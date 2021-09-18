import {
  createStyles,
  List,
  ListSubheader,
  makeStyles,
  Theme,
} from '@material-ui/core';
import Fuse from 'fuse.js';
import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { AlbumListItems } from './AlbumListItems';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
import { Album, Track } from './models';
import { range } from './range';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    ul: {
      backgroundColor: theme.palette.background.default,
      padding: 0,
    },
    listSubheader: {
      top: 48,
    },
  })
);

export const ArtistComponent = () => {
  const classes = useStyles();
  const { id } = useParams<{ id: string }>();
  const { data, refresh } = useGraphQLData(API.artist(id));
  const [pattern, setPattern] = useState('');
  const [albumFuse, setAlbumFuse] = useState<Fuse<Album>>();
  const [trackFuse, setTrackFuse] = useState<Fuse<Track>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.artist.albums) {
        setAlbumFuse(new Fuse(data.artist.albums, { keys: ['name'] }));
      }
      if (data.artist.tracks) {
        setTrackFuse(new Fuse(data.artist.tracks, { keys: ['name'] }));
      }
    }
  }, [data]);
  const albums = data?.artist.albums ?? [];
  const filteredAlbums =
    albumFuse !== undefined && pattern.length > 0
      ? albumFuse.search(pattern).map((result) => result.item)
      : albums;
  const tracks = data?.artist.tracks ?? [];
  const filteredTrackIndices =
    trackFuse !== undefined && pattern.length > 0
      ? trackFuse.search(pattern).map((result) => result.refIndex)
      : range(tracks.length);

  return data ? (
    <>
      <TitleComponent
        title={data.artist.name}
        subtitle="Artist"
      ></TitleComponent>
      {albums.length > 0 || tracks.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List subheader={<li />}>
            {filteredAlbums.length > 0 && (
              <li>
                <ul className={classes.ul}>
                  <ListSubheader className={classes.listSubheader}>
                    Albums
                  </ListSubheader>
                  <List component="div">
                    <AlbumListItems albums={filteredAlbums} refresh={refresh} />
                  </List>
                </ul>
              </li>
            )}
            {filteredTrackIndices.length > 0 && (
              <li>
                <ul className={classes.ul}>
                  <ListSubheader className={classes.listSubheader}>
                    Tracks
                  </ListSubheader>
                  <List component="div">
                    <TrackListItems
                      tracks={tracks}
                      filteredTrackIndices={filteredTrackIndices}
                      refresh={refresh}
                    />
                  </List>
                </ul>
              </li>
            )}
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
