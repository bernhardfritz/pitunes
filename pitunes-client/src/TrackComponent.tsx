// eslint-disable-next-line import/no-webpack-loader-syntax
import TrackQuery from '!!raw-loader!./graphql/TrackQuery.graphql';
import {
  AppBar,
  createStyles,
  IconButton,
  makeStyles,
  Slider,
  Theme,
  Toolbar,
} from '@material-ui/core';
import { grey } from '@material-ui/core/colors';
import AlbumIcon from '@material-ui/icons/Album';
import CloseIcon from '@material-ui/icons/Close';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import RepeatIcon from '@material-ui/icons/Repeat';
import ShuffleIcon from '@material-ui/icons/Shuffle';
import SkipNextIcon from '@material-ui/icons/SkipNext';
import SkipPreviousIcon from '@material-ui/icons/SkipPrevious';
import React, { useContext, useEffect } from 'react';
import { RouteComponentProps, useParams, withRouter } from 'react-router-dom';
import { AppActionType, AppContext } from './App';
import { useGraphQLData } from './useGraphQLData';
import { useLoaded } from './useLoaded';
import { WithAudio, withAudio } from './withAudio';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    container: {
      position: 'fixed',
      inset: 0,
      backgroundColor: theme.palette.background.default,
      zIndex: theme.zIndex.modal,
    },
    toolbar: theme.mixins.toolbar,
    column: {
      display: 'flex',
      flexFlow: 'column',
      height: '100%',
    },
    coverArt: {
      flexGrow: 1,
      width: 'auto',
      margin: 'auto',
      maxWidth: '100%',
      backgroundColor: '#fff',
      fill: grey[400],
    },
    controls: {
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
    },
    playIcon: {
      height: 48,
      width: 48,
    },
  })
);

type TrackComponentProps = WithAudio & RouteComponentProps;

const TrackComponent = (props: TrackComponentProps) => {
  const classes = useStyles();
  const { id } = useParams<{ id: string }>();
  const { dispatch, fetcher } = useContext(AppContext);
  const data = useGraphQLData(fetcher, {
    query: TrackQuery,
    operationName: 'TrackQuery',
    variables: {
      id,
    },
  });
  const loaded = useLoaded();

  useEffect(() => {
    loaded &&
      data &&
      dispatch({
        type: AppActionType.UPDATE_QUEUE,
        queue: [data.track],
      });
  }, [loaded, data]);

  return (
    <div className={classes.container}>
      <AppBar>
        <Toolbar>
          <IconButton
            edge="start"
            color="inherit"
            onClick={() => props.history.goBack()}
            aria-label="close"
          >
            <CloseIcon />
          </IconButton>
        </Toolbar>
      </AppBar>
      <div className={classes.column}>
        <div className={classes.toolbar} />
        <AlbumIcon className={classes.coverArt} />
        <Slider
          color="secondary"
          value={props.currentTime}
          max={data?.track.duration / 1000}
          onChange={(event, value) => props.seek(value as number)}
        ></Slider>
        <div className={classes.controls}>
          <IconButton>
            <ShuffleIcon />
          </IconButton>
          <IconButton onClick={() => dispatch({ type: AppActionType.PREV })}>
            <SkipPreviousIcon />
          </IconButton>
          <IconButton onClick={() => props.togglePaused()}>
            {props.paused ? (
              <PlayCircleFilledIcon className={classes.playIcon} />
            ) : (
              <PauseCircleFilledIcon className={classes.playIcon} />
            )}
          </IconButton>
          <IconButton onClick={() => dispatch({ type: AppActionType.NEXT })}>
            <SkipNextIcon />
          </IconButton>
          <IconButton>
            <RepeatIcon />
          </IconButton>
        </div>
      </div>
    </div>
  );
};

export const TrackComponentWithRouter = withRouter(withAudio(TrackComponent));
