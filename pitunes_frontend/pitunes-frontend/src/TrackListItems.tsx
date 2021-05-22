import {
  createStyles,
  FormGroup,
  ListItem,
  ListItemSecondaryAction,
  ListItemText,
  makeStyles,
  MenuItem,
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
import { Album, Artist, Genre, Playlist, Track } from './models';
import { orNbsp } from './orNbsp';
import { rotateRight } from './rotateRight';
import { useGraphQLData } from './useGraphQLData';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    ellipsis: {
      overflow: 'hidden',
      whiteSpace: 'nowrap',
      textOverflow: 'ellipsis',
    },
    formGroup: {
      '&>*:not(last-child)': {
        marginBottom: theme.spacing(2),
      },
    },
  })
);

type TrackListItemsProps = {
  tracks: Track[];
  playlist?: Playlist;
  refresh?: () => void;
};

type TrackAndPosition = { track: Track; position: number };

export const TrackListItems = ({
  tracks,
  playlist,
  refresh,
}: TrackListItemsProps) => {
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
  const { data } = useGraphQLData(API.albumsArtistsGenresPlaylists());

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
        event.target.elements['albumId'].value || undefined,
        event.target.elements['artistId'].value || undefined,
        event.target.elements['genreId'].value || undefined,
        +event.target.elements['trackNumber'].value || undefined
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
                    {orNbsp(track.name)}
                  </Typography>
                }
              />
              <ListItemSecondaryAction>
                <MenuComponent
                  items={[
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
                        ...(data?.playlists ?? []).map(
                          (playlist: Playlist) => ({
                            name: playlist.name,
                            onClick: () =>
                              handleClickAddToPlaylist(playlist, track),
                          })
                        ),
                      ],
                    },
                    {},
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
            <FormGroup className={classes.formGroup}>
              <TextField
                type="text"
                id="name"
                label="Name"
                defaultValue={editTrack?.name}
                autoFocus
              />
              <TextField
                label="Artist"
                defaultValue={editTrack?.artist?.id}
                inputProps={{ id: 'artistId' }}
                select
              >
                <MenuItem value="">{orNbsp('')}</MenuItem>
                {(data?.artists ?? []).map((artist: Artist) => (
                  <MenuItem value={artist.id}>{orNbsp(artist.name)}</MenuItem>
                ))}
              </TextField>
              <TextField
                label="Album"
                defaultValue={editTrack?.album?.id}
                inputProps={{ id: 'albumId' }}
                select
              >
                <MenuItem value="">{orNbsp('')}</MenuItem>
                {(data?.albums ?? []).map((album: Album) => (
                  <MenuItem value={album.id}>{orNbsp(album.name)}</MenuItem>
                ))}
              </TextField>
              <TextField
                type="number"
                id="trackNumber"
                label="Track number"
                defaultValue={editTrack?.trackNumber}
              />
              <TextField
                label="Genre"
                defaultValue={editTrack?.genre?.id}
                inputProps={{ id: 'genreId' }}
                select
              >
                <MenuItem value="">{orNbsp('')}</MenuItem>
                {(data?.genres ?? []).map((genre: Genre) => (
                  <MenuItem value={genre.id}>{orNbsp(genre.name)}</MenuItem>
                ))}
              </TextField>
            </FormGroup>
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
      )}
    </>
  );
};
