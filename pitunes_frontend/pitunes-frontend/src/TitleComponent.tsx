import {
  Box,
  createStyles,
  makeStyles,
  Theme,
  Typography,
} from '@material-ui/core';
import React from 'react';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    ellipsis: {
      overflow: 'hidden',
      whiteSpace: 'nowrap',
      textOverflow: 'ellipsis',
    },
  })
);

type TitleComponentProps = { title: string; subtitle?: string };

export const TitleComponent = (props: TitleComponentProps) => {
  const classes = useStyles();
  return (
    <Box p={2}>
      {props.subtitle && (
        <Typography
          variant="subtitle1"
          color="textSecondary"
          component="div"
          className={classes.ellipsis}
        >
          {props.subtitle}
        </Typography>
      )}
      <Typography variant="h4" component="div" className={classes.ellipsis}>
        {props.title}
      </Typography>
    </Box>
  );
};
