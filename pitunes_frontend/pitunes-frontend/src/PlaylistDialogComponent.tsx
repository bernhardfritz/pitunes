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
import { createPlaylist, fetcher } from './graphql/api';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    fabSm: (props: PlaylistDialogComponentProps) => ({
      position: 'fixed',
      right: theme.spacing(2),
      bottom:
        (props.playerVisible ? Number(theme.mixins.toolbar.minHeight) : 0) +
        theme.spacing(2),
      [theme.breakpoints.up('sm')]: {},
    }),
    fab: (props: PlaylistDialogComponentProps) => ({
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

type PlaylistDialogComponentProps = {
  playerVisible: boolean;
};

type PlaylistDialogComponentState = { open: boolean };

export const PlaylistDialogComponent = (
  props: PlaylistDialogComponentProps
) => {
  const classes = useStyles(props);
  const [state, setState] = useState<PlaylistDialogComponentState>({
    open: false,
  });
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
    const { data } = await fetcher(
      createPlaylist(event.target.elements['name'].value)
    );
    setState({ open: false });
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
      <PlaylistDialog
        open={state.open}
        handleClose={handleClose}
        handleSubmit={handleSubmit}
      />
    </>
  );
};

type PlaylistDialogProps = {
  open: boolean;
  handleClose: () => void;
  handleSubmit: (event: any) => void;
};

export const PlaylistDialog = (props: PlaylistDialogProps) => (
  <Dialog
    open={props.open}
    onClose={props.handleClose}
    aria-labelledby="form-dialog-title"
  >
    <form onSubmit={props.handleSubmit}>
      <DialogTitle id="form-dialog-title">Create playlist</DialogTitle>
      <DialogContent>
        <TextField type="text" id="name" label="Name" autoFocus />
      </DialogContent>
      <DialogActions>
        <Button onClick={props.handleClose}>Cancel</Button>
        <Button type="submit">Create</Button>
      </DialogActions>
    </form>
  </Dialog>
);
