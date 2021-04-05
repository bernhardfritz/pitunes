// eslint-disable-next-line import/no-webpack-loader-syntax
import ArtistQuery from '!!raw-loader!./graphql/ArtistQuery.graphql';
import { List, ListSubheader, makeStyles, Theme } from '@material-ui/core';
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
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
  const { fetcher } = useContext(AppContext);
  const { data } = useGraphQLData(fetcher, {
    query: ArtistQuery,
    operationName: 'ArtistQuery',
    variables: {
      id,
    },
  });

  return data ? (
    <>
      <TitleComponent
        title={data.artist.name}
        subtitle="Artist"
      ></TitleComponent>
      {data.artist.albums.length > 0 && data.artist.tracks.length > 0 ? (
        <List subheader={<li />}>
          {data.artist.albums && (
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
          {data.artist.tracks && (
            <li>
              <ul className={classes.ul}>
                <ListSubheader className={classes.listSubheader}>
                  Tracks
                </ListSubheader>
                <TrackListItems
                  tracks={data.artist.tracks}
                  playlists={data.playlists ?? []}
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
