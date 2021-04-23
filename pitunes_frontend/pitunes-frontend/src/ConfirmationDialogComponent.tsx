import React, { FC } from 'react';
import { FormDialogComponent } from './FormDialogComponent';

type ConfirmationDialogComponentProps = {
  open: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  confirm?: string;
};

export const ConfirmationDialogComponent: FC<ConfirmationDialogComponentProps> = (
  props
) => {
  const handleSubmit = (event: any) => {
    event.preventDefault();
    props.onConfirm();
  };

  return (
    <FormDialogComponent
      open={props.open}
      onClose={props.onClose}
      onSubmit={handleSubmit}
      title={props.title}
      submit={props.confirm}
      autoFocus
    >
      {props.children}
    </FormDialogComponent>
  );
};
