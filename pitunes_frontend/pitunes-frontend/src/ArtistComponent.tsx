import {
  createStyles,
  List,
  ListSubheader,
  makeStyles,
  Theme,
} from '@material-ui/core';
import React, { useState } from 'react';
import { useParams } from 'react-router-dom';
import { AlbumListItems } from './AlbumListItems';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { LoadingComponent } from './LoadingComponent';
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
  const handleSearch = (pattern: string) => setPattern(pattern);

  return data ? (
    <>
      <TitleComponent
        title={data.artist.name}
        subtitle="Artist"
      ></TitleComponent>
      {(data.artist.albums && data.artist.albums.length > 0) ||
      (data.artist.tracks && data.artist.tracks.length > 0) ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List subheader={<li />}>
            {data.artist.albums && data.artist.albums.length > 0 && (
              <li>
                <ul className={classes.ul}>
                  <ListSubheader className={classes.listSubheader}>
                    Albums
                  </ListSubheader>
                  <List component="div">
                    <AlbumListItems
                      albums={data.artist.albums}
                      pattern={pattern}
                      refresh={refresh}
                    />
                  </List>
                </ul>
              </li>
            )}
            {data.artist.tracks && data.artist.tracks.length > 0 && (
              <li>
                <ul className={classes.ul}>
                  <ListSubheader className={classes.listSubheader}>
                    Tracks
                  </ListSubheader>
                  <List component="div">
                    <TrackListItems
                      tracks={data.artist.tracks}
                      pattern={pattern}
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
