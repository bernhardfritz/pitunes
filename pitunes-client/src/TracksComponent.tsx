import { List, ListItem, ListItemText } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import { Track } from './models';
import { AppAction, AppActionType } from './App';
import { rotateRight } from './rotateRight';
// eslint-disable-next-line import/no-webpack-loader-syntax
import TracksQuery from '!!raw-loader!./graphql/TracksQuery.graphql';

type TracksComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
};

type TracksComponentState = { tracks: Track[] };

export default class TracksComponent extends React.Component<
  TracksComponentProps,
  TracksComponentState
> {
  constructor(props: TracksComponentProps) {
    super(props);
    this.state = {
      tracks: [],
    };
  }

  componentDidMount() {
    this.props.dispatch({ type: AppActionType.UPDATE_TITLE, title: 'Tracks' });
    this.props
      .fetcher({
        query: TracksQuery,
        operationName: 'TracksQuery',
      })
      .then((res) => {
        this.setState({
          tracks: res.data.tracks,
        });
      });
  }

  render() {
    const { tracks } = this.state;
    return (
      <List>
        {tracks.map((track, index) => (
          <ListItem
            button
            key={track.id}
            onClick={(_) =>
              this.props.dispatch({
                type: AppActionType.UPDATE_QUEUE,
                queue: rotateRight([...tracks], index),
              })
            }
          >
            <ListItemText primary={track.name} />
          </ListItem>
        ))}
      </List>
    );
  }
}
