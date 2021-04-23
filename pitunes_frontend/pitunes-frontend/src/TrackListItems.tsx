import {
  createStyles,
  ListItem,
  ListItemSecondaryAction,
  ListItemText,
  makeStyles,
  TextField,
  Theme,
  Typography,
} from '@material-ui/core';
import React, { useContext, useState } from 'react';
import { AppActionType, AppContext } from './App';
import { ConfirmationDialogComponent } from './ConfirmationDialogComponent';
import { FormDialogComponent } from './FormDialogComponent';
import * as API from './graphql/api';
import { fetcher } from './graphql/fetcher';
import { MenuComponent } from './MenuComponent';
import { Playlist, Track } from './models';
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
  const [
    newPlaylistTrack,
    setClickNewPlaylistTrack,
  ] = useState<TrackAndPosition | null>(null);
  const openCreatePlaylistDialog = Boolean(newPlaylistTrack);
  const [editTrack, setEditTrack] = useState<Track | null>(null);
  const openEditTrackDialog = Boolean(editTrack);
  const [deleteTrack, setDeleteTrack] = useState<Track | null>(null);
  const openDeleteTrackDialog = Boolean(deleteTrack);
  const { dispatch } = useContext(AppContext);

  const handleSubmitCreatePlaylistDialog = async (event: any) => {
    event.preventDefault();

    const { data } = await fetcher(
      API.createPlaylist(event.target.elements['name'].value)
    );

    await fetcher(
      API.createPlaylistTrack(
        data.createPlaylist.id,
        newPlaylistTrack!.track.id,
        newPlaylistTrack!.position
      )
    );

    setClickNewPlaylistTrack(null);

    if (refresh !== undefined) {
      refresh();
    }
  };

  const handleSubmitEditTrackDialog = async (event: any) => {
    event.preventDefault();

    if (editTrack === null) {
      return;
    }

    await fetcher(
      API.updateTrack(
        editTrack.id,
        event.target.elements['name'].value,
        editTrack.album?.id,
        editTrack.artist?.id,
        editTrack.genre?.id,
        editTrack.trackNumber
      )
    );

    setEditTrack(null);

    if (refresh !== undefined) {
      refresh();
    }
  };

  const handleClickAddToPlaylist = async (
    playlist: Playlist,
    track: Track,
    position?: number
  ) => {
    await fetcher(API.createPlaylistTrack(playlist.id, track.id, position));

    if (refresh !== undefined) {
      refresh();
    }
  };

  const handleClickRemoveFromPlaylist = async (
    playlist: Playlist,
    track: Track,
    position?: number
  ) => {
    await fetcher(API.deletePlaylistTrack(playlist.id, track.id, position));

    if (refresh !== undefined) {
      refresh();
    }
  };

  const handleConfirmDeleteTrackDialog = async () => {
    if (deleteTrack === null) {
      return;
    }

    await fetcher(API.deleteTrack(deleteTrack.id));
    setDeleteTrack(null);

    if (refresh !== undefined) {
      refresh();
    }
  };

  return (
    <>
      {tracks && tracks.length > 0 && (
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
                  <Typography className={classes.ellipsis}>
                    {track.name}
                  </Typography>
                }
              />
              <ListItemSecondaryAction>
                <MenuComponent
                  items={[
                    {
                      name: 'Add to playlist',
                      items: [
                        {
                          name: 'New playlist',
                          onClick: () =>
                            setClickNewPlaylistTrack({
                              track,
                              position: index,
                            }),
                        },
                        ...playlists.map((playlist) => ({
                          name: playlist.name,
                          onClick: () =>
                            handleClickAddToPlaylist(playlist, track),
                        })),
                      ],
                    },
                    ...(playlist
                      ? [
                          {
                            name: 'Remove from this playlist',
                            onClick: () =>
                              handleClickRemoveFromPlaylist(
                                playlist,
                                track,
                                index
                              ),
                          },
                        ]
                      : []),
                    {
                      name: 'Edit',
                      onClick: () => setEditTrack(track),
                    },
                    {
                      name: 'Delete',
                      onClick: () => setDeleteTrack(track),
                    },
                  ]}
                ></MenuComponent>
              </ListItemSecondaryAction>
            </ListItem>
          ))}
        </>
      )}
      <FormDialogComponent
        open={openCreatePlaylistDialog}
        onClose={() => setClickNewPlaylistTrack(null)}
        onSubmit={handleSubmitCreatePlaylistDialog}
        title="Create playlist"
        submit="Create"
      >
        <TextField type="text" id="name" label="Name" autoFocus />
      </FormDialogComponent>
      <FormDialogComponent
        open={openEditTrackDialog}
        onClose={() => setEditTrack(null)}
        onSubmit={handleSubmitEditTrackDialog}
        title="Edit track"
        submit="Edit"
      >
        <TextField
          type="text"
          id="name"
          label="Name"
          defaultValue={editTrack?.name}
          autoFocus
        />
      </FormDialogComponent>
      <ConfirmationDialogComponent
        open={openDeleteTrackDialog}
        onClose={() => setDeleteTrack(null)}
        onConfirm={handleConfirmDeleteTrackDialog}
        title="Delete track"
        confirm="Delete"
      >
        {deleteTrack?.name}
      </ConfirmationDialogComponent>
    </>
  );
};
