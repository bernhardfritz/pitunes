import { List } from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Track } from './models';
import { AppContext } from './ResponsiveDrawer';

type PlaylistComponentProps = { fetcher: Fetcher } & RouteComponentProps<{
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
        this.context.setTitle(playlist.name);
        this.setState({
          tracks: playlist.tracks,
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

PlaylistComponent.contextType = AppContext;
