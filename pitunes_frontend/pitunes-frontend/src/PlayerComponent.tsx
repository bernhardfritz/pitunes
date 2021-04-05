import {
  AppBar,
  createStyles,
  IconButton,
  makeStyles,
  Theme,
  Toolbar,
} from '@material-ui/core';
import { grey } from '@material-ui/core/colors';
import AlbumIcon from '@material-ui/icons/Album';
import PauseIcon from '@material-ui/icons/Pause';
import PlayArrowIcon from '@material-ui/icons/PlayArrow';
import React, { useEffect } from 'react';
import { RouteComponentProps, withRouter } from 'react-router-dom';
import { Track } from './models';
import { drawerWidth } from './ResponsiveDrawer';
import { WithAudio, withAudio } from './withAudio';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    appBar: {
      top: 'auto',
      bottom: 0,
      [theme.breakpoints.up('sm')]: {
        width: `calc(100% - ${drawerWidth}px)`,
        marginLeft: drawerWidth,
      },
      color: theme.palette.text.primary,
      backgroundColor: theme.palette.background.paper,
    },
    toolbar: {
      paddingLeft: 0,
      paddingRight: 4,
      [theme.breakpoints.up('sm')]: {
        paddingRight: 8,
      },
    },
    indicator: {
      position: 'absolute',
      top: 0,
      left: 0,
      height: 2,
      backgroundColor: theme.palette.primary.main,
    },
    coverArtPreview: {
      backgroundColor: '#fff',
      fill: grey[400],
      width: 'auto',
      height: theme.mixins.toolbar.minHeight,
      [theme.breakpoints.up('sm')]: {
        height: (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any)
          .minHeight,
      },
    },
    flex: {
      display: 'flex',
      width: '100%',
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
    iconButtonWrapper: {
      display: 'flex',
      alignItems: 'center',
    },
  })
);

type PlayerComponentProps = { track: Track } & WithAudio & RouteComponentProps;

const PlayerComponent = (props: PlayerComponentProps) => {
  const classes = useStyles();

  useEffect(() => {
    props.play(`/api/tracks/${props.track.id}.mp3`);
  }, [props.track]);

  const handleAppBarClick = () =>
    props.history.push(`/tracks/${props.track?.id}`);
  const handlePauseClick = (event: any) => {
    props.togglePaused();
    event.stopPropagation();
  };

  return (
    <AppBar
      position="fixed"
      className={classes.appBar}
      onClick={handleAppBarClick}
    >
      <Toolbar className={classes.toolbar}>
        <span
          className={classes.indicator}
          style={{
            width: `${
              (props.currentTime * 1000 * 100) / props.track?.duration
            }%`,
          }}
        ></span>
        <div className={classes.flex}>
          <AlbumIcon className={classes.coverArtPreview} />
          <div className={classes.trackMetadataContainer}>
            <div className={`${classes.ellipsis} ${classes.bold}`}>
              {props.track?.name}
            </div>
            <div className={classes.ellipsis}>{props.track?.artist?.name}</div>
          </div>
          <div className={classes.iconButtonWrapper}>
            <IconButton color="inherit" onClick={handlePauseClick}>
              {props.paused ? <PlayArrowIcon /> : <PauseIcon />}
            </IconButton>
          </div>
        </div>
      </Toolbar>
    </AppBar>
  );
};

export const PlayerComponentWithRouter = withRouter(withAudio(PlayerComponent));
