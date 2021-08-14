import {
  createMuiTheme,
  createStyles,
  CssBaseline,
  makeStyles,
  Tab,
  Tabs,
  Theme,
  ThemeProvider,
  useMediaQuery,
} from '@material-ui/core';
import React, { useEffect, useReducer } from 'react';
import {
  Redirect,
  Route,
  RouteComponentProps,
  withRouter,
} from 'react-router-dom';
import { AddItemDialogComponent } from './AddItemDialogComponent';
import { AlbumComponent } from './AlbumComponent';
import { AlbumsComponent } from './AlbumsComponent';
import { ArtistComponent } from './ArtistComponent';
import { ArtistsComponent } from './ArtistsComponent';
import { GenreComponent } from './GenreComponent';
import { GenresComponent } from './GenresComponent';
import { GraphiQLComponent } from './GraphiQLComponent';
import * as API from './graphql/api';
import { Track } from './models';
import { PlayerComponentWithRouter } from './PlayerComponent';
import { PlaylistComponent } from './PlaylistComponent';
import { PlaylistsComponent } from './PlaylistsComponent';
import { ResponsiveDrawer } from './ResponsiveDrawer';
import { rotateRight } from './rotateRight';
import { TrackComponentWithRouter } from './TrackComponent';
import { TracksComponent } from './TracksComponent';
import { TransitionRoute } from './TransitionRoute';
import { UploadComponent } from './UploadComponent';
import { usePrevious } from './usePrevious';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    toolbar: theme.mixins.toolbar,
  })
);

export enum TransitionType {
  LEFT = 'left',
  RIGHT = 'right',
  FORWARD = 'forward',
  BACKWARD = 'backward',
  FADE = 'fade',
  UP = 'up',
  DOWN = 'down',
}

type AppState = {
  audio: HTMLAudioElement;
  queue: Track[];
  queueUpdatedAt: number;
};

export enum AppActionType {
  UPDATE_QUEUE,
  PREV,
  NEXT,
}

export type AppAction =
  | { type: AppActionType.UPDATE_QUEUE; queue: Track[] }
  | { type: AppActionType.PREV }
  | { type: AppActionType.NEXT };

const reducer: React.Reducer<AppState, AppAction> = (prevState, action) => {
  switch (action.type) {
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
  state: AppState;
  dispatch: React.Dispatch<AppAction>;
};

export const AppContext = React.createContext<AppContextProps>({
  state: {
    audio: new Audio(),
    queue: [],
    queueUpdatedAt: Date.now(),
  },
  dispatch: (action: AppAction) => {},
});

type AppProps = RouteComponentProps;

const App = (props: AppProps) => {
  const classes = useStyles();
  const { history, location } = props;
  const prevLocation = usePrevious(location);

  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');

  const theme = React.useMemo(
    () =>
      createMuiTheme({
        palette: {
          type: prefersDarkMode ? 'dark' : 'light',
          primary: {
            main: prefersDarkMode ? '#75bfff' : '#0074e8',
          },
          secondary: {
            main: prefersDarkMode ? '#fb7be6' : '#dd00a9',
          },
          background: {
            default: prefersDarkMode ? '#18181a' : '#ffffff',
            paper: prefersDarkMode ? '#232327' : '#f9f9fa',
          },
          text: {
            primary: prefersDarkMode ? '#ffffff' : '#38383d',
            secondary: prefersDarkMode ? '#b1b1b3' : '#939395',
          },
          error: {
            main: prefersDarkMode ? '#ff3b6b' : '#d70022',
          },
          warning: {
            main: prefersDarkMode ? '#ffbf00' : '#ffbf00',
          },
          info: {
            main: prefersDarkMode ? '#75bfff' : '#0074e8',
          },
          success: {
            main: prefersDarkMode ? '#86de74' : '#058b00',
          },
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
    {
      label: 'Genres',
      to: '/genres',
    },
    {
      label: 'Tracks',
      to: '/tracks',
    },
  ];

  const tabIndex = tabs.findIndex(
    (tab) =>
      !location.pathname.startsWith('/tracks/') &&
      location.pathname.startsWith(tab.to)
  );
  const prevTabIndex =
    prevLocation !== undefined
      ? tabs.findIndex(
          (tab) =>
            !prevLocation.pathname.startsWith('/tracks/') &&
            prevLocation.pathname.startsWith(tab.to)
        )
      : -1;
  const transitionType =
    tabIndex >= 0 && prevTabIndex >= 0
      ? tabIndex < prevTabIndex
        ? TransitionType.LEFT
        : tabIndex === prevTabIndex
        ? prevLocation !== undefined &&
          location.pathname.startsWith(prevLocation.pathname)
          ? TransitionType.FORWARD
          : TransitionType.BACKWARD
        : TransitionType.RIGHT
      : TransitionType.FADE;

  const [state, dispatch] = useReducer(reducer, {
    audio: new Audio(),
    queue: [],
    queueUpdatedAt: Date.now(),
  });

  const playerVisible = state.queue.length > 0;

  const handleTabChange = (event: React.ChangeEvent<{}>, tabIndex: number) => {
    history.push(tabs[tabIndex].to);
  };

  useEffect(() => {
    const onEnded = () => {
      dispatch({ type: AppActionType.NEXT });
    };
    state.audio.addEventListener('ended', onEnded);
    return () => {
      state.audio.removeEventListener('ended', onEnded);
    };
  }, [state.audio]);

  return (
    <AppContext.Provider
      value={{
        state,
        dispatch,
      }}
    >
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <ResponsiveDrawer
          title={
            location.pathname.startsWith('/upload')
              ? 'Upload'
              : location.pathname.startsWith('/graphiql')
              ? 'GraphiQL'
              : 'Library'
          }
          tabs={
            !location.pathname.startsWith('/upload') &&
            !location.pathname.startsWith('/graphiql') && (
              <Tabs
                value={
                  tabIndex >= 0
                    ? tabIndex
                    : prevTabIndex >= 0
                    ? prevTabIndex
                    : false
                }
                onChange={handleTabChange}
                indicatorColor="primary"
                textColor="primary"
                variant="scrollable"
                scrollButtons="auto"
              >
                {tabs.map((tab) => (
                  <Tab key={tab.label} label={tab.label} />
                ))}
              </Tabs>
            )
          }
        >
          <div className={transitionType}>
            <Route exact path="/">
              <Redirect to="/playlists" />
            </Route>
            <TransitionRoute exact path="/albums">
              <AlbumsComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/albums/:id">
              <AlbumComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/artists">
              <ArtistsComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/artists/:id">
              <ArtistComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/genres">
              <GenresComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/genres/:id">
              <GenreComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/playlists">
              <PlaylistsComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/playlists/:id">
              <PlaylistComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <TransitionRoute exact path="/tracks">
              <TracksComponent />
              {playerVisible && <div className={classes.toolbar} />}
            </TransitionRoute>
            <Route exact path="/upload">
              <UploadComponent playerVisible={playerVisible} />
              {playerVisible && <div className={classes.toolbar} />}
            </Route>
            <Route exact path="/graphiql">
              <GraphiQLComponent playerVisible={playerVisible} />
            </Route>
          </div>
        </ResponsiveDrawer>
        <div
          className={
            location.pathname.startsWith('/tracks/')
              ? TransitionType.UP
              : TransitionType.DOWN
          }
        >
          <TransitionRoute exact path="/tracks/:id">
            <TrackComponentWithRouter />
          </TransitionRoute>
        </div>
        <AddItemDialogComponent
          playerVisible={playerVisible}
          nameTofetcherParams={API.createAlbum}
          dataToId={(data) => data.createAlbum.id}
          pathname="/albums"
          label="album"
        />
        <AddItemDialogComponent
          playerVisible={playerVisible}
          nameTofetcherParams={API.createArtist}
          dataToId={(data) => data.createArtist.id}
          pathname="/artists"
          label="artist"
        />
        <AddItemDialogComponent
          playerVisible={playerVisible}
          nameTofetcherParams={API.createGenre}
          dataToId={(data) => data.createGenre.id}
          pathname="/genres"
          label="genre"
        />
        <AddItemDialogComponent
          playerVisible={playerVisible}
          nameTofetcherParams={API.createPlaylist}
          dataToId={(data) => data.createPlaylist.id}
          pathname="/playlists"
          label="playlist"
        />
        {playerVisible && <PlayerComponentWithRouter track={state.queue[0]} />}
      </ThemeProvider>
    </AppContext.Provider>
  );
};

export const AppWithRouter = withRouter(App);
