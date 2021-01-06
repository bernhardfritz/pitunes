import { List } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Track } from './models';
import { AppContext } from './ResponsiveDrawer';

type AlbumComponentProps = { fetcher: Fetcher } & RouteComponentProps<{
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
        this.context.setTitle(album.name);
        this.setState({
          tracks: album.tracks,
        });
      });
  }

  render() {
    const { tracks } = this.state;
    return (
      <List>
        {tracks.map((track) => (
          <ListItemLink
            key={track.id}
            to={`/tracks/${track.id}`}
            primary={track.name}
          ></ListItemLink>
        ))}
      </List>
    );
  }
}

AlbumComponent.contextType = AppContext;
