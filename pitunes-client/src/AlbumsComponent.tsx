import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Album } from './models';
import { AppAction, AppActionType } from './App';

type AlbumsComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
};

type AlbumsComponentState = { albums: Album[] };

export default class AlbumsComponent extends React.Component<
  AlbumsComponentProps,
  AlbumsComponentState
> {
  constructor(props: AlbumsComponentProps) {
    super(props);
    this.state = {
      albums: [],
    };
  }

  componentDidMount() {
    this.props.dispatch({ type: AppActionType.UPDATE_TITLE, title: 'Albums' });
    this.props
      .fetcher({
        query: `query AlbumsQuery {
  albums {
    id
    name
  }
}`,
        operationName: 'AlbumsQuery',
      })
      .then((res) => {
        this.setState({
          albums: res.data.albums,
        });
      });
  }

  render() {
    const { albums } = this.state;
    return (
      <List>
        {albums.map((album) => (
          <ListItemLink
            key={album.id}
            to={`/albums/${album.id}`}
            primary={album.name}
          ></ListItemLink>
        ))}
      </List>
    );
  }
}
