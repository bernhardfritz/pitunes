import { List, ListItem, ListItemText } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { AppAction, AppActionType } from './App';
import { Fetcher } from './fetcher';
import { Track } from './models';
import { rotateRight } from './rotateRight';

type PlaylistComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
} & RouteComponentProps<{
  id: string;
}>;

type PlaylistComponentState = { tracks: Track[] };

export default class PlaylistComponent extends React.Component<
  PlaylistComponentProps,
  PlaylistComponentState
> {
  constructor(props: PlaylistComponentProps) {
    super(props);
    this.state = {
      tracks: [],
    };
  }

  componentDidMount() {
    this.props
      .fetcher({
        query: `query PlaylistTracksQuery($id: ID!) {
  playlist(id: $id) {
    id
    name
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
  }
}`,
        operationName: 'PlaylistTracksQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { playlist } = res.data;
        this.props.dispatch({
          type: AppActionType.UPDATE_TITLE,
          title: playlist.name,
        });
        this.setState({
          tracks: playlist.tracks,
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
