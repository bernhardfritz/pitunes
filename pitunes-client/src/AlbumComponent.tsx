import { List, ListItem, ListItemText } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { Fetcher } from './fetcher';
import { Track } from './models';
import { AppAction, AppActionType } from './App';
import { rotateRight } from './rotateRight';

type AlbumComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
} & RouteComponentProps<{
  id: string;
}>;

type AlbumComponentState = { tracks: Track[] };

export default class AlbumComponent extends React.Component<
  AlbumComponentProps,
  AlbumComponentState
> {
  constructor(props: AlbumComponentProps) {
    super(props);
    this.state = {
      tracks: [],
    };
  }

  componentDidMount() {
    this.props
      .fetcher({
        query: `query AlbumTracksQuery($id: ID!) {
  album(id: $id) {
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
        operationName: 'AlbumTracksQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { album } = res.data;
        this.props.dispatch({
          type: AppActionType.UPDATE_TITLE,
          title: album.name,
        });
        this.setState({
          tracks: album.tracks,
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
