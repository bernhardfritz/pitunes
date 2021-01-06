import React from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import GraphiQL from 'graphiql';

import 'graphiql/graphiql.min.css';
import ResponsiveDrawer, { drawerWidth } from './ResponsiveDrawer';
import AlbumsComponent from './AlbumsComponent';
import AlbumComponent from './AlbumComponent';
import ArtistsComponent from './ArtistsComponent';
import ArtistComponent from './ArtistComponent';
import GenresComponent from './GenresComponent';
import PlaylistsComponent from './PlaylistsComponent';
import GenreComponent from './GenreComponent';
import PlaylistComponent from './PlaylistComponent';
import TracksComponent from './TracksComponent';
import {
  createMuiTheme,
  createStyles,
  Theme,
  ThemeProvider,
  WithStyles,
  withStyles,
} from '@material-ui/core/styles';
import TrackComponent from './TrackComponent';
import { RootComponent } from './RootComponent';
import { UploadComponent } from './UploadComponent';

const theme = createMuiTheme({
  palette: {
    type:
      window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)')
        ? 'dark'
        : 'light',
  },
});

const styles = (theme: Theme) =>
  createStyles({
    graphiql: {
      boxSizing: 'unset',
      position: 'absolute',
      top: theme.mixins.toolbar.minHeight,
      right: 0,
      bottom: 0,
      left: 0,
      [theme.breakpoints.up('sm')]: {
        top: (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any)
          .minHeight,
        left: drawerWidth,
      },
    },
  });

type AppProps = {} & WithStyles<typeof styles, true>;

type AppState = {};

class App extends React.Component<AppProps, AppState> {
  graphQLFetcher = (graphQLParams: any) => {
    return fetch('/api/graphql', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(graphQLParams),
    }).then((response) => response.json());
  };

  render() {
    return (
      <ThemeProvider theme={theme}>
        <Router>
          <ResponsiveDrawer>
            {/*
              A <Switch> looks through all its children <Route>
              elements and renders the first one whose path
              matches the current URL. Use a <Switch> any time
              you have multiple routes, but you want only one
              of them to render at a time
            */}
            <Switch>
              <Route exact path="/">
                <RootComponent />
              </Route>
              <Route exact path="/albums">
                <AlbumsComponent
                  fetcher={this.graphQLFetcher}
                ></AlbumsComponent>
              </Route>
              <Route
                path="/albums/:id"
                render={(props) => (
                  <AlbumComponent
                    {...props}
                    fetcher={this.graphQLFetcher}
                  ></AlbumComponent>
                )}
              />
              <Route exact path="/artists">
                <ArtistsComponent
                  fetcher={this.graphQLFetcher}
                ></ArtistsComponent>
              </Route>
              <Route
                path="/artists/:id"
                render={(props) => (
                  <ArtistComponent
                    {...props}
                    fetcher={this.graphQLFetcher}
                  ></ArtistComponent>
                )}
              />
              <Route exact path="/genres">
                <GenresComponent
                  fetcher={this.graphQLFetcher}
                ></GenresComponent>
              </Route>
              <Route
                path="/genres/:id"
                render={(props) => (
                  <GenreComponent
                    {...props}
                    fetcher={this.graphQLFetcher}
                  ></GenreComponent>
                )}
              />
              <Route exact path="/playlists">
                <PlaylistsComponent
                  fetcher={this.graphQLFetcher}
                ></PlaylistsComponent>
              </Route>
              <Route
                path="/genres/:id"
                render={(props) => (
                  <PlaylistComponent
                    {...props}
                    fetcher={this.graphQLFetcher}
                  ></PlaylistComponent>
                )}
              />
              <Route exact path="/tracks">
                <TracksComponent
                  fetcher={this.graphQLFetcher}
                ></TracksComponent>
              </Route>
              <Route
                path="/tracks/:id"
                render={(props) => (
                  <TrackComponent
                    {...props}
                    fetcher={this.graphQLFetcher}
                  ></TrackComponent>
                )}
              />
              <Route path="/upload">
                <UploadComponent />
              </Route>
              <Route path="/graphiql">
                <div className={this.props.classes.graphiql}>
                  <GraphiQL fetcher={this.graphQLFetcher} />
                </div>
              </Route>
            </Switch>
          </ResponsiveDrawer>
        </Router>
      </ThemeProvider>
    );
  }
}

export default withStyles(styles, { withTheme: true })(App);
