import {
  createStyles,
  Theme,
  withStyles,
  WithStyles,
} from '@material-ui/core/styles';
import GraphiQL from 'graphiql';
import 'graphiql/graphiql.min.css';
import React from 'react';
import { Fetcher } from './fetcher';
import { AppAction, AppActionType } from './App';
import { drawerWidth } from './ResponsiveDrawer';

const styles = (theme: Theme) =>
  createStyles({
    graphiql: {
      boxSizing: 'unset',
      position: 'absolute',
      top: theme.mixins.toolbar.minHeight,
      right: 0,
      bottom: 0,
      left: 0,
      [theme.breakpoints.up('sm')]: {
        top: (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any)
          .minHeight,
        left: drawerWidth,
      },
    },
  });

type GraphiQLComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
} & WithStyles<typeof styles, true>;

type GraphiQLComponentState = {};

class GraphiQLComponent extends React.Component<
  GraphiQLComponentProps,
  GraphiQLComponentState
> {
  componentDidMount() {
    this.props.dispatch({
      type: AppActionType.UPDATE_TITLE,
      title: 'GraphiQL',
    });
  }

  render() {
    return (
      <div className={this.props.classes.graphiql}>
        <GraphiQL fetcher={this.props.fetcher} />
      </div>
    );
  }
}

export default withStyles(styles, { withTheme: true })(GraphiQLComponent);
