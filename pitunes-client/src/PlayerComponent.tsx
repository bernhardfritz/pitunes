import {
  createStyles,
  Theme,
  withStyles,
  WithStyles,
} from '@material-ui/core/styles';
import React from 'react';
import grey from '@material-ui/core/colors/grey';
import { AppBar, IconButton, Slide, Slider, Toolbar } from '@material-ui/core';
import AlbumIcon from '@material-ui/icons/Album';
import SkipPreviousIcon from '@material-ui/icons/SkipPrevious';
import PlayArrowIcon from '@material-ui/icons/PlayArrow';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
import SkipNextIcon from '@material-ui/icons/SkipNext';
import PauseIcon from '@material-ui/icons/Pause';
import CloseIcon from '@material-ui/icons/Close';
import ShuffleIcon from '@material-ui/icons/Shuffle';
import RepeatIcon from '@material-ui/icons/Repeat';
import RepeatOneIcon from '@material-ui/icons/RepeatOne';
import { Track } from './models';
import Dialog from '@material-ui/core/Dialog';
import { TransitionProps } from '@material-ui/core/transitions';
import { AppAction, AppActionType } from './App';

const styles = (theme: Theme) =>
  createStyles({
    dialog: {
      backgroundColor: theme.palette.background.default,
    },
    appBar: {
      top: 'auto',
      bottom: 0,
    },
    toolbar: theme.mixins.toolbar,
    coverArtPreview: {
      marginLeft: '-16px',
      width: theme.mixins.toolbar.minHeight,
      height: theme.mixins.toolbar.minHeight,
      backgroundColor: '#fff',
      fill: grey[400],
    },
    coverArt: {
      margin: theme.spacing(1),
      width: `calc(min(100vw, calc(100vh - ${
        theme.mixins.toolbar.minHeight
      }px)) - ${theme.spacing(2)}px)`,
      height: `calc(min(100vw, calc(100vh - ${
        theme.mixins.toolbar.minHeight
      }px)) - ${theme.spacing(2)}px)`,
      backgroundColor: '#fff',
      fill: grey[400],
    },
    grow: {
      flexGrow: 1,
      marginLeft: theme.spacing(1),
    },
    trackName: {
      fontWeight: theme.typography.fontWeightBold,
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
  });

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children?: React.ReactElement },
  ref: React.Ref<unknown>
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

type PlayerComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  track?: Track;
  queueUpdatedAt: number;
} & WithStyles<typeof styles, true>;

type PlayerComponentState = {
  open: boolean;
  paused: boolean;
  currentTime: number;
};

class PlayerComponent extends React.Component<
  PlayerComponentProps,
  PlayerComponentState
> {
  readonly audio: React.RefObject<HTMLAudioElement>;
  constructor(props: PlayerComponentProps) {
    super(props);
    this.state = {
      open: false,
      paused: true,
      currentTime: 0.0,
    };
    this.audio = React.createRef();
  }

  componentDidUpdate(prevProps: PlayerComponentProps) {
    if (prevProps.queueUpdatedAt !== this.props.queueUpdatedAt) {
      const audio = this.audio.current;
      if (audio === null) {
        return;
      }
      audio.currentTime = 0;
    }
  }

  readonly handleClick = (event: any) => {
    const audio = this.audio.current;
    if (audio === null) {
      return;
    }
    if (audio.paused) {
      audio.play();
    } else {
      audio.pause();
    }
    event.stopPropagation();
  };

  readonly handleClose = () => {
    this.setState({ open: false });
  };

  render() {
    return (
      <>
        <audio
          src={
            this.props.track !== undefined
              ? `/api/tracks/${this.props.track.id}.mp3`
              : undefined
          }
          onPlay={() =>
            this.setState({ paused: this.audio.current?.paused ?? true })
          }
          onPause={() =>
            this.setState({ paused: this.audio.current?.paused ?? true })
          }
          onTimeUpdate={() =>
            this.setState({
              currentTime: this.audio.current?.currentTime ?? 0.0,
            })
          }
          onEnded={() => this.props.dispatch({ type: AppActionType.NEXT })}
          autoPlay
          ref={this.audio}
        />
        <AppBar
          position="fixed"
          className={this.props.classes.appBar}
          onClick={() =>
            this.setState((prevState) => ({ open: !prevState.open }))
          }
        >
          <Toolbar>
            <AlbumIcon className={this.props.classes.coverArtPreview} />
            <div className={this.props.classes.grow}>
              <div className={this.props.classes.trackName}>
                {this.props.track?.name}
              </div>
              <div>{this.props.track?.artist?.name}</div>
            </div>
            <IconButton edge="end" color="inherit" onClick={this.handleClick}>
              {this.state.paused ? <PlayArrowIcon /> : <PauseIcon />}
            </IconButton>
          </Toolbar>
        </AppBar>
        <Dialog
          fullScreen
          open={this.state.open}
          onClose={this.handleClose}
          TransitionComponent={Transition}
          className={this.props.classes.dialog}
        >
          <AppBar>
            <Toolbar>
              <IconButton
                edge="start"
                color="inherit"
                onClick={this.handleClose}
                aria-label="close"
              >
                <CloseIcon />
              </IconButton>
            </Toolbar>
          </AppBar>
          <div className={this.props.classes.toolbar} />
          <AlbumIcon className={this.props.classes.coverArt} />
          <Slider
            value={this.state.currentTime}
            max={this.audio.current?.duration}
            onChange={(event, value) => {
              const audio = this.audio.current;
              if (audio === null) {
                return;
              }
              audio.currentTime = value as number;
            }}
          ></Slider>
          <div className={this.props.classes.controls}>
            <IconButton>
              <ShuffleIcon />
            </IconButton>
            <IconButton
              onClick={() => this.props.dispatch({ type: AppActionType.PREV })}
            >
              <SkipPreviousIcon />
            </IconButton>
            <IconButton onClick={this.handleClick}>
              {this.state.paused ? (
                <PlayCircleFilledIcon className={this.props.classes.playIcon} />
              ) : (
                <PauseCircleFilledIcon
                  className={this.props.classes.playIcon}
                />
              )}
            </IconButton>
            <IconButton
              onClick={() => this.props.dispatch({ type: AppActionType.NEXT })}
            >
              <SkipNextIcon />
            </IconButton>
            <IconButton>
              <RepeatIcon />
            </IconButton>
          </div>
        </Dialog>
      </>
    );
  }
}

export default withStyles(styles, { withTheme: true })(PlayerComponent);
