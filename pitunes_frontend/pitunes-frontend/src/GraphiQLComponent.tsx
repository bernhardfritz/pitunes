import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import GraphiQL from 'graphiql';
import 'graphiql/graphiql.min.css';
import React from 'react';
import { fetcher } from './graphql/api';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    graphiql: (props: GraphiQLComponentProps) => ({
      boxSizing: 'unset',
      height: `calc(100vh - ${
        (props.playerVisible ? 2 : 1) * Number(theme.mixins.toolbar.minHeight)
      }px)`,
      [theme.breakpoints.up('sm')]: {
        height: `calc(100vh - ${
          (props.playerVisible ? 2 : 1) *
          Number(
            (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any).minHeight
          )
        }px)`,
      },
    }),
  })
);

type GraphiQLComponentProps = { playerVisible: boolean };

export const GraphiQLComponent = (props: GraphiQLComponentProps) => {
  const classes = useStyles(props);

  return (
    <div className={classes.graphiql}>
      <GraphiQL fetcher={fetcher} />
    </div>
  );
};
