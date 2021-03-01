import { makeStyles, Theme } from '@material-ui/core/styles';
import GraphiQL from 'graphiql';
import 'graphiql/graphiql.min.css';
import React from 'react';
import { AppContext } from './App';

const useStyles = makeStyles((theme: Theme) => ({
  graphiql: {
    boxSizing: 'unset',
    height: '100vh',
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
