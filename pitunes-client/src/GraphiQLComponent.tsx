import { makeStyles, Theme } from '@material-ui/core/styles';
import GraphiQL from 'graphiql';
import 'graphiql/graphiql.min.css';
import React from 'react';
import { AppContext } from './App';
import { drawerWidth } from './ResponsiveDrawer';

const useStyles = makeStyles((theme: Theme) => ({
  graphiql: {
    boxSizing: 'unset',
    position: 'absolute',
    top: theme.mixins.toolbar.minHeight,
    right: 0,
    bottom: 0,
    left: 0,
    [theme.breakpoints.up('sm')]: {
      top: (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any).minHeight,
      left: drawerWidth,
    },
  },
}));

export const GraphiQLComponent = () => {
  const classes = useStyles();

  return (
    <AppContext.Consumer>
      {({ fetcher }) => (
        <div className={classes.graphiql}>
          <GraphiQL fetcher={fetcher} />
        </div>
      )}
    </AppContext.Consumer>
  );
};
