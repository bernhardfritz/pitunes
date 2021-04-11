import {
  createStyles,
  IconButton,
  ListItem,
  ListItemSecondaryAction,
  ListItemText,
  makeStyles,
  Menu,
  MenuItem,
  Theme,
  Typography,
} from '@material-ui/core';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import React, { useContext, useState } from 'react';
import { AppActionType, AppContext } from './App';
import {
  createPlaylist,
  createPlaylistTrack,
  deletePlaylistTrack,
  fetcher,
} from './graphql/api';
import { Playlist, Track } from './models';
import { NestedMenuItem } from './NestedMenuItem';
import { PlaylistDialog } from './PlaylistDialogComponent';
import { rotateRight } from './rotateRight';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    ellipsis: {
      overflow: 'hidden',
      whiteSpace: 'nowrap',
      textOverflow: 'ellipsis',
    },
  })
);

type TrackListItemProps = {
  tracks: Track[];
  playlists: Playlist[];
  playlist?: Playlist;
  refresh?: () => void;
};

type TrackAndPosition = { track: Track; position: number };

export const TrackListItems = ({
  tracks,
  playlists,
  playlist,
  refresh,
}: TrackListItemProps) => {
  const classes = useStyles();
  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const openMenu = Boolean(anchorEl);
  const [
    trackAndPositionForNewPlaylist,
    setTrackAndPositionForNewPlaylist,
  ] = useState<TrackAndPosition | null>(null);
  const openPlaylistDialog = Boolean(trackAndPositionForNewPlaylist);
  const { dispatch } = useContext(AppContext);

  const handleMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
  };

  const handleNewPlaylistClick = (track: Track, position: number) => {
    setAnchorEl(null);
    setTrackAndPositionForNewPlaylist({ track, position });
  };

  const handlePlaylistDialogClose = () => {
    setTrackAndPositionForNewPlaylist(null);
  };

  const handleNewPlaylistSubmit = async (event: any) => {
    event.preventDefault();
    const { data: createPlaylistMutationData } = await fetcher(
      createPlaylist(event.target.elements['name'].value)
    );
    const { data: createPlaylistTrackMutationData } = await fetcher(
      createPlaylistTrack(
        createPlaylistMutationData.createPlaylist.id,
        trackAndPositionForNewPlaylist!.track.id,
        trackAndPositionForNewPlaylist!.position
      )
    );
    setTrackAndPositionForNewPlaylist(null);

    return createPlaylistTrackMutationData;
  };

  const handleAddToPlaylist = async (
    playlist: Playlist,
    track: Track,
    position?: number
  ) => {
    setAnchorEl(null);
    const { data } = await fetcher(
      createPlaylistTrack(playlist.id, track.id, position)
    );

    return data;
  };

  const handleRemoveFromPlaylist = async (
    playlist: Playlist,
    track: Track,
    position?: number
  ) => {
    setAnchorEl(null);
    const { data } = await fetcher(
      deletePlaylistTrack(playlist.id, track.id, position)
    );

    if (refresh) {
      refresh();
    }

    return data;
  };

  return (
    <>
      {tracks.map((track, index) => (
        <ListItem
          key={track.id}
          button
          onClick={() =>
            dispatch({
              type: AppActionType.UPDATE_QUEUE,
              queue: rotateRight([...tracks], index),
            })
          }
        >
          <ListItemText
            primary={
              <Typography className={classes.ellipsis}>{track.name}</Typography>
            }
          />
          <ListItemSecondaryAction>
            <IconButton edge="end" onClick={handleMenuClick}>
              <MoreVertIcon />
            </IconButton>
            <Menu anchorEl={anchorEl} open={openMenu} onClose={handleMenuClose}>
              <NestedMenuItem
                label="Add to playlist"
                parentMenuOpen={openMenu}
                left
              >
                <MenuItem onClick={() => handleNewPlaylistClick(track, index)}>
                  New playlist
                </MenuItem>
                {playlists.map((playlist) => (
                  <MenuItem
                    onClick={() => handleAddToPlaylist(playlist, track)}
                  >
                    {playlist.name}
                  </MenuItem>
                ))}
              </NestedMenuItem>
              {playlist && (
                <MenuItem
                  onClick={() =>
                    handleRemoveFromPlaylist(playlist, track, index)
                  }
                >
                  Remove from this playlist
                </MenuItem>
              )}
            </Menu>
          </ListItemSecondaryAction>
        </ListItem>
      ))}
      <PlaylistDialog
        open={openPlaylistDialog}
        handleClose={handlePlaylistDialogClose}
        handleSubmit={handleNewPlaylistSubmit}
      />
    </>
  );
};
