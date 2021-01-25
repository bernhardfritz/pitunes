import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Genre } from './models';
import { AppAction, AppActionType } from './App';
// eslint-disable-next-line import/no-webpack-loader-syntax
import GenresQuery from '!!raw-loader!./graphql/GenresQuery.graphql';

type GenresComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
};

type GenresComponentState = { genres: Genre[] };

export default class GenresComponent extends React.Component<
  GenresComponentProps,
  GenresComponentState
> {
  constructor(props: any) {
    super(props);
    this.state = {
      genres: [],
    };
  }

  componentDidMount() {
    this.props.dispatch({ type: AppActionType.UPDATE_TITLE, title: 'Genres' });
    this.props
      .fetcher({
        query: GenresQuery,
        operationName: 'GenresQuery',
      })
      .then((res) => {
        this.setState({
          genres: res.data.genres,
        });
      });
  }

  render() {
    const { genres } = this.state;
    return (
      <List>
        {genres.map((genre) => (
          <ListItemLink
            key={genre.id}
            to={`/genres/${genre.id}`}
            primary={genre.name}
          ></ListItemLink>
        ))}
      </List>
    );
  }
}
