import {
  createStyles,
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
import { FormDialogComponent } from './FormDialogComponent';
import { createPlaylist } from './graphql/api';
import { fetcher } from './graphql/fetcher';

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
  refresh?: () => void;
};

export const PlaylistDialogComponent = (
  props: PlaylistDialogComponentProps
) => {
  const classes = useStyles(props);
  const [open, setOpen] = useState<boolean>(false);
  const history = useHistory();
  const sm = !useMediaQuery((theme: Theme) => theme.breakpoints.up('sm')); // workaround

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    const { data } = await fetcher(
      createPlaylist(event.target.elements['name'].value)
    );
    setOpen(false);
    history.push(`/playlists/${data.createPlaylist.id}`);
  };

  return (
    <>
      <Zoom in={history.location.pathname === '/playlists'} unmountOnExit>
        <Fab
          className={sm ? classes.fabSm : classes.fab}
          onClick={() => setOpen(true)}
        >
          <AddIcon />
        </Fab>
      </Zoom>
      <FormDialogComponent
        open={open}
        onClose={() => setOpen(false)}
        onSubmit={handleSubmit}
        title="Create playlist"
        submit="Create"
      >
        <TextField type="text" id="name" label="Name" autoFocus />
      </FormDialogComponent>
    </>
  );
};
