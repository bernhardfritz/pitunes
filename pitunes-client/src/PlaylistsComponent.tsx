import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Playlist } from './models';
import { AppAction, AppActionType } from './App';
// eslint-disable-next-line import/no-webpack-loader-syntax
import PlaylistsQuery from '!!raw-loader!./graphql/PlaylistsQuery.graphql';

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
        query: PlaylistsQuery,
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
