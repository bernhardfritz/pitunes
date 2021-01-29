import { CssBaseline, useMediaQuery } from '@material-ui/core';
import {
  ThemeProvider,
  unstable_createMuiStrictModeTheme as createMuiTheme,
} from '@material-ui/core/styles';
import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import React, { useReducer } from 'react';
import {
  BrowserRouter as Router,
  Redirect,
  Route,
  Switch,
} from 'react-router-dom';
import { AlbumComponent } from './AlbumComponent';
import { AlbumsComponent } from './AlbumsComponent';
import { ArtistComponent } from './ArtistComponent';
import { ArtistsComponent } from './ArtistsComponent';
import { GenreComponent } from './GenreComponent';
import { GenresComponent } from './GenresComponent';
import { GraphiQLComponent } from './GraphiQLComponent';
import { Track } from './models';
import PlayerComponent from './PlayerComponent';
import { PlaylistComponent } from './PlaylistComponent';
import { PlaylistsComponent } from './PlaylistsComponent';
import ResponsiveDrawer from './ResponsiveDrawer';
import { rotateRight } from './rotateRight';
import { TracksComponent } from './TracksComponent';
import { UploadComponent } from './UploadComponent';

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

type AppContextProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: (graphQLParams: FetcherParams) => Promise<any>;
};

export const AppContext = React.createContext<AppContextProps>({
  dispatch: (action: AppAction) => {},
  fetcher: (graphQLParams: FetcherParams) => Promise.resolve(),
});

export const App = () => {
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

  const [state, dispatch] = useReducer(reducer, {
    title: '',
    queue: [],
    queueUpdatedAt: 0,
  });

  return (
    <AppContext.Provider
      value={{
        dispatch,
        fetcher: (graphQLParams: FetcherParams) =>
          fetch('/api/graphql', {
            method: 'post',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(graphQLParams),
          }).then((response) => response.json()),
      }}
    >
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
                <Redirect to="/playlists" />
              </Route>
              <Route exact path="/albums">
                <AlbumsComponent />
              </Route>
              <Route path="/albums/:id" render={AlbumComponent} />
              <Route exact path="/artists">
                <ArtistsComponent />
              </Route>
              <Route path="/artists/:id" component={ArtistComponent} />
              <Route exact path="/genres">
                <GenresComponent />
              </Route>
              <Route path="/genres/:id" render={GenreComponent} />
              <Route exact path="/playlists">
                <PlaylistsComponent />
              </Route>
              <Route path="/playlists/:id" render={PlaylistComponent} />
              <Route exact path="/tracks">
                <TracksComponent />
              </Route>
              <Route path="/upload">
                <UploadComponent />
              </Route>
              <Route path="/graphiql">
                <GraphiQLComponent />
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
    </AppContext.Provider>
  );
}
