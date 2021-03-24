import { createStyles, makeStyles, Theme, Typography } from '@material-ui/core';
import React from 'react';
import { ReactComponent as Doge } from './doge.svg';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    wrapper: {
      display: 'flex',
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      margin: 'auto',
    },
    dogeWrapper: {
      display: 'flex',
      width: 96,
      height: 96,
      borderRadius: 48,
      background:
        theme.palette.type === 'dark'
          ? theme.palette.grey[800]
          : theme.palette.grey[300],
      marginBottom: theme.spacing(3),
    },
    doge: {
      width: 48,
      height: 48,
      margin: 'auto',
      fill: theme.palette.common.white,
    },
  })
);

export const EmptyListComponent = () => {
  const classes = useStyles();
  return (
    <div className={classes.wrapper}>
      <div className={classes.container}>
        <div className={classes.dogeWrapper}>
          <Doge className={classes.doge}></Doge>
        </div>
        <Typography variant="subtitle1" color="textSecondary" component="div">
          Wow, such empty
        </Typography>
      </div>
    </div>
  );
};
