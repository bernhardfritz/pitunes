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
import React, { useContext, useEffect, useRef } from 'react';
import { RouteComponentProps, useParams, withRouter } from 'react-router-dom';
import { AppActionType, AppContext } from './App';
import { formatDuration } from './formatDuration';
import * as API from './graphql/api';
import { Track } from './models';
import { useAudio } from './useAudio';
import { useGraphQLData } from './useGraphQLData';
import { useLoaded } from './useLoaded';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    container: {
      position: 'fixed',
      inset: 0,
      backgroundColor: theme.palette.background.default,
      zIndex: theme.zIndex.modal,
    },
    appBar: {
      color: theme.palette.text.primary,
      backgroundColor: theme.palette.background.paper,
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
      marginBottom: 12,
    },
    playIcon: {
      height: 48,
      width: 48,
    },
    trackMetadataContainer: {
      flexGrow: 1,
      minWidth: 0,
      margin: 'auto 8px',
    },
    bold: {
      fontWeight: theme.typography.fontWeightBold,
    },
    ellipsis: {
      overflow: 'hidden',
      whiteSpace: 'nowrap',
      textOverflow: 'ellipsis',
    },
    sliderContainerWrapper: {
      margin: '8px 8px 0 8px',
    },
    sliderContainer: {
      display: 'flex',
      alignItems: 'center',
      margin: '0 -4px',
      '&>*': {
        margin: '0 4px',
      }
    }
  })
);

type TrackComponentProps = {} & RouteComponentProps;

const TrackComponent = (props: TrackComponentProps) => {
  const classes = useStyles();
  const { id } = useParams<{ id: string }>();
  const { state, dispatch } = useContext(AppContext);
  const { data } = useGraphQLData(API.track(id));
  const loaded = useLoaded();
  const [paused, currentTime, play, togglePaused, seek] = useAudio(state.audio);
  const trackRef = useRef<Track>();

  useEffect(() => {
    loaded &&
      data &&
      dispatch({
        type: AppActionType.UPDATE_QUEUE,
        queue: [data.track],
      });
  }, [loaded, data]);

  useEffect(() => {
    trackRef.current = state.queue[0];
    if (trackRef.current !== undefined) {
      props.history.replace(`/tracks/${trackRef.current.id}`);
    }
  }, [state.queue[0]]);

  return (
    <div className={classes.container}>
      <AppBar className={classes.appBar}>
        <Toolbar>
          <IconButton
            edge="start"
            color="inherit"
            onClick={() => props.history.goBack()}
            aria-label="close"
          >
            <CloseIcon />
          </IconButton>
          {trackRef.current && (
            <div className={classes.trackMetadataContainer}>
              <div className={`${classes.ellipsis} ${classes.bold}`}>
                {trackRef.current.name}
              </div>
              <div className={classes.ellipsis}>{trackRef.current.artist?.name}</div>
            </div>
          )}
        </Toolbar>
      </AppBar>
      <div className={classes.column}>
        <div className={classes.toolbar} />
        <AlbumIcon className={classes.coverArt} />
        <div className={classes.sliderContainerWrapper}>
          <div className={classes.sliderContainer}>
            <div>{formatDuration(currentTime * 1000)}</div>
            <Slider
              value={currentTime}
              max={(trackRef.current?.duration ?? NaN) / 1000}
              onChange={(event, value) => seek(value as number)}
            ></Slider>
            <div>{formatDuration(trackRef.current?.duration ?? NaN)}</div>
          </div>
        </div>
        <div className={classes.controls}>
          <IconButton>
            <ShuffleIcon />
          </IconButton>
          <IconButton onClick={() => dispatch({ type: AppActionType.PREV })}>
            <SkipPreviousIcon />
          </IconButton>
          <IconButton onClick={() => togglePaused()}>
            {paused ? (
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

export const TrackComponentWithRouter = withRouter(TrackComponent);
