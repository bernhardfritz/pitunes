import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Artist } from './models';
import { AppAction, AppActionType } from './App';
// eslint-disable-next-line import/no-webpack-loader-syntax
import ArtistsQuery from '!!raw-loader!./graphql/ArtistsQuery.graphql';

type ArtistsComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
};

type ArtistsComponentState = { artists: Artist[] };

export default class ArtistsComponent extends React.Component<
  ArtistsComponentProps,
  ArtistsComponentState
> {
  constructor(props: ArtistsComponentProps) {
    super(props);
    this.state = {
      artists: [],
    };
  }

  componentDidMount() {
    this.props.dispatch({ type: AppActionType.UPDATE_TITLE, title: 'Artists' });
    this.props
      .fetcher({
        query: ArtistsQuery,
        operationName: 'ArtistsQuery',
      })
      .then((res) => {
        this.setState({
          artists: res.data.artists,
        });
      });
  }

  render() {
    const { artists } = this.state;
    return (
      <List>
        {artists.map((artist) => (
          <ListItemLink
            key={artist.id}
            to={`/artists/${artist.id}`}
            primary={artist.name}
          ></ListItemLink>
        ))}
      </List>
    );
  }
}
