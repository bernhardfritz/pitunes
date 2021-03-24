// eslint-disable-next-line import/no-webpack-loader-syntax
import CreatePlaylistMutation from '!!raw-loader!./graphql/CreatePlaylistMutation.graphql';
import {
  Button,
  createStyles,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Fab,
  makeStyles,
  TextField,
  Theme,
  useMediaQuery,
  Zoom,
} from '@material-ui/core';
import AddIcon from '@material-ui/icons/Add';
import React, { useState } from 'react';
import { useHistory } from 'react-router-dom';
import { Fetcher } from './fetcher';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    fabSm: (props: PlaylistDialogProps) => ({
      position: 'fixed',
      right: theme.spacing(2),
      bottom:
        (props.playerVisible ? Number(theme.mixins.toolbar.minHeight) : 0) +
        theme.spacing(2),
      [theme.breakpoints.up('sm')]: {},
    }),
    fab: (props: PlaylistDialogProps) => ({
      position: 'fixed',
      right: theme.spacing(2),
      bottom:
        (props.playerVisible
          ? Number(
              (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any)
                .minHeight
            )
          : 0) + theme.spacing(2),
    }),
  })
);

type PlaylistDialogProps = { fetcher: Fetcher; playerVisible: boolean };

type PlaylistDialogState = { open: boolean };

export const PlaylistDialog = (props: PlaylistDialogProps) => {
  const classes = useStyles(props);
  const [state, setState] = useState<PlaylistDialogState>({ open: false });
  const history = useHistory();
  const sm = !useMediaQuery((theme: Theme) => theme.breakpoints.up('sm')); // workaround

  const handleClickOpen = () => {
    setState({ open: true });
  };

  const handleClose = () => {
    setState({ open: false });
  };

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    const { data } = await props.fetcher({
      query: CreatePlaylistMutation,
      operationName: 'CreatePlaylistMutation',
      variables: {
        input: {
          name: event.target.elements['name'].value,
        },
      },
    });
    history.push(`/playlists/${data.createPlaylist.id}`);
  };

  return (
    <>
      <Zoom in={history.location.pathname === '/playlists'} unmountOnExit>
        <Fab
          className={sm ? classes.fabSm : classes.fab}
          onClick={handleClickOpen}
        >
          <AddIcon />
        </Fab>
      </Zoom>
      <Dialog
        open={state.open}
        onClose={handleClose}
        aria-labelledby="form-dialog-title"
      >
        <form onSubmit={handleSubmit}>
          <DialogTitle id="form-dialog-title">Create playlist</DialogTitle>
          <DialogContent>
            <TextField type="text" id="name" label="Name" autoFocus />
          </DialogContent>
          <DialogActions>
            <Button onClick={handleClose}>Cancel</Button>
            <Button type="submit">Create</Button>
          </DialogActions>
        </form>
      </Dialog>
    </>
  );
};
