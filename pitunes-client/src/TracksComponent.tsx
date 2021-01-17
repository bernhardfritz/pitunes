import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Track } from './models';
import { AppAction, AppActionType } from './App';
import { rotateRight } from './rotateRight';

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
        query: `query TracksQuery {
  tracks {
    id
    name
    duration
    album {
      id
      name
    }
    artist {
      id
      name
    }
    genre {
      id
      name
    }
    trackNumber
  }
}`,
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
          <ListItemLink
            key={track.id}
            to={`/tracks/${track.id}`}
            primary={track.name}
            onClick={(_) =>
              this.props.dispatch({
                type: AppActionType.UPDATE_QUEUE,
                queue: rotateRight([...tracks], index),
              })
            }
          ></ListItemLink>
        ))}
      </List>
    );
  }
}
