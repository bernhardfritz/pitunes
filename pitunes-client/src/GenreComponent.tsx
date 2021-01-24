import { List, ListItem, ListItemText } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { Fetcher } from './fetcher';
import { Track } from './models';
import { AppAction, AppActionType } from './App';
import { rotateRight } from './rotateRight';

type GenreComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
} & RouteComponentProps<{
  id: string;
}>;

type GenreComponentState = { tracks: Track[] };

export default class GenreComponent extends React.Component<
  GenreComponentProps,
  GenreComponentState
> {
  constructor(props: any) {
    super(props);
    this.state = {
      tracks: [],
    };
  }

  componentDidMount() {
    this.props
      .fetcher({
        query: `query GenreTracksQuery($id: ID!) {
  genre(id: $id) {
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
        operationName: 'GenreTracksQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { genre } = res.data;
        this.props.dispatch({
          type: AppActionType.UPDATE_TITLE,
          title: genre.name,
        });
        this.setState({
          tracks: genre.tracks,
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
