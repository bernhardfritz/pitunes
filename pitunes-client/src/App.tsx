import React, { useReducer } from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import ResponsiveDrawer from './ResponsiveDrawer';
import AlbumsComponent from './AlbumsComponent';
import AlbumComponent from './AlbumComponent';
import ArtistsComponent from './ArtistsComponent';
import ArtistComponent from './ArtistComponent';
import GenresComponent from './GenresComponent';
import PlaylistsComponent from './PlaylistsComponent';
import GenreComponent from './GenreComponent';
import PlaylistComponent from './PlaylistComponent';
import TracksComponent from './TracksComponent';
import { unstable_createMuiStrictModeTheme as createMuiTheme, ThemeProvider } from '@material-ui/core/styles';
import TrackComponent from './TrackComponent';
import RootComponent from './RootComponent';
import UploadComponent from './UploadComponent';
import GraphiQLComponent from './GraphiQLComponent';
import { CssBaseline, useMediaQuery } from '@material-ui/core';
import { Track } from './models';
import PlayerComponent from './PlayerComponent';
import { rotateRight } from './rotateRight';

type AppState = {
  title: string;
  queue: Track[];
  queueUpdatedAt: number;
};

export enum AppActionType {
  UPDATE_TITLE,
  UPDATE_QUEUE,
  PREV,
  NEXT,
}

export type AppAction =
  | { type: AppActionType.UPDATE_TITLE; title: string }
  | { type: AppActionType.UPDATE_QUEUE; queue: Track[] }
  | { type: AppActionType.PREV }
  | { type: AppActionType.NEXT };

const reducer: React.Reducer<AppState, AppAction> = (prevState, action) => {
  switch (action.type) {
    case AppActionType.UPDATE_TITLE:
      return {
        ...prevState,
        title: action.title,
      };
    case AppActionType.UPDATE_QUEUE:
      return {
        ...prevState,
        queue: action.queue,
        queueUpdatedAt: Date.now(),
      };
    case AppActionType.PREV:
      return {
        ...prevState,
        queue: rotateRight([...prevState.queue], -1),
        queueUpdatedAt: Date.now(),
      };
    case AppActionType.NEXT:
      return {
        ...prevState,
        queue: rotateRight([...prevState.queue], 1),
        queueUpdatedAt: Date.now(),
      };
  }
};

export default function App() {
  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');

  const theme = React.useMemo(
    () =>
      createMuiTheme({
        palette: {
          type: prefersDarkMode ? 'dark' : 'light',
        },
      }),
    [prefersDarkMode]
  );

  const graphQLFetcher = (graphQLParams: any) => {
    return fetch('/api/graphql', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(graphQLParams),
    }).then((response) => response.json());
  };

  const [state, dispatch] = useReducer(reducer, { title: '', queue: [], queueUpdatedAt: 0 });

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <ResponsiveDrawer title={state.title}>
          {/*
              A <Switch> looks through all its children <Route>
              elements and renders the first one whose path
              matches the current URL. Use a <Switch> any time
              you have multiple routes, but you want only one
              of them to render at a time
            */}
          <Switch>
            <Route exact path="/">
              <RootComponent dispatch={dispatch} />
            </Route>
            <Route exact path="/albums">
              <AlbumsComponent
                dispatch={dispatch}
                fetcher={graphQLFetcher}
              ></AlbumsComponent>
            </Route>
            <Route
              path="/albums/:id"
              render={(props) => (
                <AlbumComponent
                  {...props}
                  dispatch={dispatch}
                  fetcher={graphQLFetcher}
                ></AlbumComponent>
              )}
            />
            <Route exact path="/artists">
              <ArtistsComponent
                dispatch={dispatch}
                fetcher={graphQLFetcher}
              ></ArtistsComponent>
            </Route>
            <Route
              path="/artists/:id"
              render={(props) => (
                <ArtistComponent
                  {...props}
                  dispatch={dispatch}
                  fetcher={graphQLFetcher}
                ></ArtistComponent>
              )}
            />
            <Route exact path="/genres">
              <GenresComponent
                dispatch={dispatch}
                fetcher={graphQLFetcher}
              ></GenresComponent>
            </Route>
            <Route
              path="/genres/:id"
              render={(props) => (
                <GenreComponent
                  {...props}
                  dispatch={dispatch}
                  fetcher={graphQLFetcher}
                ></GenreComponent>
              )}
            />
            <Route exact path="/playlists">
              <PlaylistsComponent
                dispatch={dispatch}
                fetcher={graphQLFetcher}
              ></PlaylistsComponent>
            </Route>
            <Route
              path="/genres/:id"
              render={(props) => (
                <PlaylistComponent
                  {...props}
                  dispatch={dispatch}
                  fetcher={graphQLFetcher}
                ></PlaylistComponent>
              )}
            />
            <Route exact path="/tracks">
              <TracksComponent
                dispatch={dispatch}
                fetcher={graphQLFetcher}
              ></TracksComponent>
            </Route>
            {/* <Route
              path="/tracks/:id"
              render={(props) => (
                <TrackComponent
                  {...props}
                  dispatch={dispatch}
                  fetcher={graphQLFetcher}
                ></TrackComponent>
              )}
            /> */}
            <Route path="/upload">
              <UploadComponent dispatch={dispatch} />
            </Route>
            <Route path="/graphiql">
              <GraphiQLComponent dispatch={dispatch} fetcher={graphQLFetcher} />
            </Route>
          </Switch>
        </ResponsiveDrawer>
        <PlayerComponent
          dispatch={dispatch}
          track={state.queue.length > 0 ? state.queue[0] : undefined}
          queueUpdatedAt={state.queueUpdatedAt}
        />
      </Router>
    </ThemeProvider>
  );
}
