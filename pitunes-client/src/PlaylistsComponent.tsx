import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Playlist } from './models';
import { AppAction, AppActionType } from './App';

type PlaylistsComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
};

type PlaylistsComponentState = { playlists: Playlist[] };

export default class PlaylistsComponent extends React.Component<
  PlaylistsComponentProps,
  PlaylistsComponentState
> {
  constructor(props: PlaylistsComponentProps) {
    super(props);
    this.state = {
      playlists: [],
    };
  }

  componentDidMount() {
    this.props.dispatch({
      type: AppActionType.UPDATE_TITLE,
      title: 'Playlists',
    });
    this.props
      .fetcher({
        query: `query PlaylistsQuery {
  playlists {
    id
    name
  }
}`,
        operationName: 'PlaylistsQuery',
      })
      .then((res) => {
        this.setState({
          playlists: res.data.playlists,
        });
      });
  }

  render() {
    const { playlists } = this.state;
    return (
      <List>
        {playlists.map((playlist) => (
          <ListItemLink
            key={playlist.id}
            to={`/playlists/${playlist.id}`}
            primary={playlist.name}
          ></ListItemLink>
        ))}
      </List>
    );
  }
}
