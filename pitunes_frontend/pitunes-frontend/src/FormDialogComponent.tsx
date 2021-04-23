import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
} from '@material-ui/core';
import React, { FC } from 'react';

type FormDialogComponentProps = {
  open: boolean;
  onClose: () => void;
  onSubmit: (event: any) => void;
  title: string;
  submit?: string;
  autoFocus?: boolean;
};

export const FormDialogComponent: FC<FormDialogComponentProps> = (props) => {
  const { open, onClose, onSubmit, title, submit, autoFocus, children } = props;

  return (
    <Dialog open={open} onClose={onClose} aria-labelledby="form-dialog-title">
      <form onSubmit={onSubmit}>
        <DialogTitle id="form-dialog-title">{title}</DialogTitle>
        <DialogContent>{children}</DialogContent>
        <DialogActions>
          <Button onClick={onClose}>Cancel</Button>
          <Button type="submit" autoFocus={autoFocus}>
            {submit ?? 'Submit'}
          </Button>
        </DialogActions>
      </form>
    </Dialog>
  );
};
