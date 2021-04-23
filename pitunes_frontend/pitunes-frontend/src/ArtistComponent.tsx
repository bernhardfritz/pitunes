import { List, ListSubheader, makeStyles, Theme } from '@material-ui/core';
import React from 'react';
import { useParams } from 'react-router-dom';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { TrackListItems } from './TrackListItems';
import { useGraphQLData } from './useGraphQLData';

const useStyles = makeStyles((theme: Theme) => ({
  ul: {
    backgroundColor: theme.palette.background.default,
    padding: 0,
  },
  listSubheader: {
    top: 48,
  },
}));

export const ArtistComponent = () => {
  const classes = useStyles();
  const { id } = useParams<{ id: string }>();
  const { data, refresh } = useGraphQLData(API.artist(id));

  return data ? (
    <>
      <TitleComponent
        title={data.artist.name}
        subtitle="Artist"
      ></TitleComponent>
      {data.artist.albums &&
      data.artist.albums.length > 0 &&
      data.artist.tracks &&
      data.artist.tracks.length > 0 ? (
        <List subheader={<li />}>
          {data.artist.albums && data.artist.albums.length > 0 && (
            <li>
              <ul className={classes.ul}>
                <ListSubheader className={classes.listSubheader}>
                  Albums
                </ListSubheader>
                <IdNameListItemLinks
                  items={data.artist.albums}
                  to={(id) => `/albums/${id}`}
                />
              </ul>
            </li>
          )}
          {data.artist.tracks && data.artist.tracks.length > 0 && (
            <li>
              <ul className={classes.ul}>
                <ListSubheader className={classes.listSubheader}>
                  Tracks
                </ListSubheader>
                <TrackListItems
                  tracks={data.artist.tracks}
                  playlists={data.playlists ?? []}
                  refresh={refresh}
                />
              </ul>
            </li>
          )}
        </List>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
