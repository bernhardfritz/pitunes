// eslint-disable-next-line import/no-webpack-loader-syntax
import ArtistQuery from '!!raw-loader!./graphql/ArtistQuery.graphql';
import { List, ListSubheader, makeStyles, Theme } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { AppContext } from './App';
import { GraphQLResource } from './GraphQLResource';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { TrackListItems } from './TrackListItems';

type ArtistComponentProps = RouteComponentProps<{ id: string }>;

const useStyles = makeStyles((theme: Theme) => ({
  ul: {
    backgroundColor: theme.palette.background.default,
    padding: 0,
  },
  listSubheader: {
    top: 48,
  },
}));

export const ArtistComponent = (props: ArtistComponentProps) => {
  const classes = useStyles();

  return (
    <AppContext.Consumer>
      {({ fetcher }) => (
        <GraphQLResource
          fetcher={fetcher}
          fetcherParams={{
            query: ArtistQuery,
            operationName: 'ArtistQuery',
            variables: {
              id: props.match.params.id,
            },
          }}
        >
          {(data: any) => (
            <List subheader={<li />}>
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
              <li>
                <ul className={classes.ul}>
                  <ListSubheader className={classes.listSubheader}>
                    Tracks
                  </ListSubheader>
                  <TrackListItems tracks={data.artist.tracks} />
                </ul>
              </li>
            </List>
          )}
        </GraphQLResource>
      )}
    </AppContext.Consumer>
  );
};
