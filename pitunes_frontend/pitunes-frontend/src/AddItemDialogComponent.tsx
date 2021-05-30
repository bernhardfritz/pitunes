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
import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import React, { useState } from 'react';
import { useHistory } from 'react-router-dom';
import { FormDialogComponent } from './FormDialogComponent';
import { fetcher } from './graphql/fetcher';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    fabSm: (props: AddItemDialogComponentProps) => ({
      position: 'fixed',
      right: theme.spacing(2),
      bottom:
        (props.playerVisible ? Number(theme.mixins.toolbar.minHeight) : 0) +
        theme.spacing(2),
      [theme.breakpoints.up('sm')]: {},
    }),
    fab: (props: AddItemDialogComponentProps) => ({
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

type AddItemDialogComponentProps = {
  playerVisible: boolean;
  nameTofetcherParams: (name: string) => FetcherParams;
  dataToId: (data: any) => string;
  pathname: string;
  label: string;
};

export const AddItemDialogComponent = (props: AddItemDialogComponentProps) => {
  const classes = useStyles(props);
  const [open, setOpen] = useState<boolean>(false);
  const history = useHistory();
  const sm = !useMediaQuery((theme: Theme) => theme.breakpoints.up('sm')); // workaround

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    const { data } = await fetcher(
      props.nameTofetcherParams(event.target.elements['name'].value)
    );
    setOpen(false);
    history.push(`${props.pathname}/${props.dataToId(data)}`);
  };

  return (
    <>
      <Zoom in={history.location.pathname === props.pathname} unmountOnExit>
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
        title={`Create ${props.label}`}
        submit="Create"
      >
        <TextField type="text" id="name" label="Name" autoFocus />
      </FormDialogComponent>
    </>
  );
};
