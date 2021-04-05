// eslint-disable-next-line import/no-webpack-loader-syntax
import CreatePlaylistTrackMutation from '!!raw-loader!./graphql/CreatePlaylistTrackMutation.graphql';
// eslint-disable-next-line import/no-webpack-loader-syntax
import DeletePlaylistTrackMutation from '!!raw-loader!./graphql/DeletePlaylistTrackMutation.graphql';
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
import { Playlist, Track } from './models';
import { NestedMenuItem } from './NestedMenuItem';
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

export const TrackListItems = ({
  tracks,
  playlists,
  playlist,
  refresh,
}: TrackListItemProps) => {
  const classes = useStyles();
  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const openMenu = Boolean(anchorEl);
  const { dispatch, fetcher } = useContext(AppContext);

  const handleMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
  };

  const createPlaylistTrack = async (
    playlist: Playlist,
    track: Track,
    position?: number
  ) => {
    setAnchorEl(null);
    const { data } = await fetcher({
      query: CreatePlaylistTrackMutation,
      operationName: 'CreatePlaylistTrackMutation',
      variables: {
        id: playlist.id,
        input: {
          id: track.id,
          position,
        },
      },
    });

    return data;
  };

  const deletePlaylistTrack = async (
    playlist: Playlist,
    track: Track,
    position?: number
  ) => {
    setAnchorEl(null);
    const { data } = await fetcher({
      query: DeletePlaylistTrackMutation,
      operationName: 'DeletePlaylistTrackMutation',
      variables: {
        id: playlist.id,
        input: {
          id: track.id,
          position,
        },
      },
    });

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
                {playlists.map((playlist) => (
                  <MenuItem
                    onClick={() => createPlaylistTrack(playlist, track)}
                  >
                    {playlist.name}
                  </MenuItem>
                ))}
              </NestedMenuItem>
              {playlist && (
                <MenuItem
                  onClick={() => deletePlaylistTrack(playlist, track, index)}
                >
                  Remove from this playlist
                </MenuItem>
              )}
            </Menu>
          </ListItemSecondaryAction>
        </ListItem>
      ))}
    </>
  );
};
