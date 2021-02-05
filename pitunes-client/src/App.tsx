import { CssBaseline, Tab, Tabs, useMediaQuery } from '@material-ui/core';
import {
  ThemeProvider,
  unstable_createMuiStrictModeTheme as createMuiTheme,
} from '@material-ui/core/styles';
import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import React, { useReducer } from 'react';
import {
  Redirect,
  Route,
  RouteComponentProps,
  withRouter,
} from 'react-router-dom';
import { AlbumComponent } from './AlbumComponent';
import { AlbumsComponent } from './AlbumsComponent';
import './App.css';
import { ArtistComponent } from './ArtistComponent';
import { ArtistsComponent } from './ArtistsComponent';
import { GenreComponent } from './GenreComponent';
import { GenresComponent } from './GenresComponent';
import { GraphiQLComponent } from './GraphiQLComponent';
import { Track } from './models';
import PlayerComponent from './PlayerComponent';
import { PlaylistComponent } from './PlaylistComponent';
import { PlaylistsComponent } from './PlaylistsComponent';
import { ResponsiveDrawer } from './ResponsiveDrawer';
import { rotateRight } from './rotateRight';
import { TracksComponent } from './TracksComponent';
import { TransitionRoute } from './TransitionRoute';
import { UploadComponent } from './UploadComponent';
import { usePrevious } from './usePrevious';

export enum TransitionType {
  LEFT = 'left',
  RIGHT = 'right',
  FORWARD = 'forward',
  BACKWARD = 'backward',
}

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

type AppProps = RouteComponentProps;

const App = (props: AppProps) => {
  const { history, location } = props;
  const prevLocation = usePrevious(location);

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

  const tabs = [
    {
      label: 'Playlists',
      to: '/playlists',
    },
    {
      label: 'Artists',
      to: '/artists',
    },
    {
      label: 'Albums',
      to: '/albums',
    },
  ];

  const tabIndex = tabs.findIndex((tab) =>
    location.pathname.startsWith(tab.to)
  );
  const prevTabIndex =
    prevLocation !== undefined
      ? tabs.findIndex((tab) => prevLocation.pathname.startsWith(tab.to))
      : -1;
  const transitionType =
    tabIndex < prevTabIndex
      ? TransitionType.LEFT
      : tabIndex === prevTabIndex
      ? prevLocation !== undefined &&
        location.pathname.startsWith(prevLocation.pathname)
        ? TransitionType.FORWARD
        : TransitionType.BACKWARD
      : TransitionType.RIGHT;

  const [state, dispatch] = useReducer(reducer, {
    title: '',
    queue: [],
    queueUpdatedAt: 0,
  });

  const handleTabChange = (event: React.ChangeEvent<{}>, tabIndex: number) => {
    history.push(tabs[tabIndex].to);
  };

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
        <ResponsiveDrawer
          tabs={
            <Tabs
              value={tabIndex}
              onChange={handleTabChange}
              variant="fullWidth"
            >
              {tabs.map((tab) => (
                <Tab key={tab.label} label={tab.label} />
              ))}
            </Tabs>
          }
        >
          <div className={transitionType}>
            <Route exact path="/">
              <Redirect to="/playlists" />
            </Route>
            <TransitionRoute exact path="/albums">
              <AlbumsComponent />
            </TransitionRoute>
            <TransitionRoute exact path="/albums/:id">
              <AlbumComponent />
            </TransitionRoute>
            <TransitionRoute exact path="/artists">
              <ArtistsComponent />
            </TransitionRoute>
            <TransitionRoute exact path="/artists/:id">
              <ArtistComponent />
            </TransitionRoute>
            <Route exact path="/genres">
              <GenresComponent />
            </Route>
            <Route exact path="/genres/:id">
              <GenreComponent />
            </Route>
            <TransitionRoute exact path="/playlists">
              <PlaylistsComponent />
            </TransitionRoute>
            <TransitionRoute exact path="/playlists/:id">
              <PlaylistComponent />
            </TransitionRoute>
            <Route exact path="/tracks">
              <TracksComponent />
            </Route>
            <Route exact path="/upload">
              <UploadComponent />
            </Route>
            <Route exact path="/graphiql">
              <GraphiQLComponent />
            </Route>
          </div>
        </ResponsiveDrawer>
        <PlayerComponent
          dispatch={dispatch}
          track={state.queue.length > 0 ? state.queue[0] : undefined}
          queueUpdatedAt={state.queueUpdatedAt}
        />
      </ThemeProvider>
    </AppContext.Provider>
  );
};

export const AppWithRouter = withRouter(App);
