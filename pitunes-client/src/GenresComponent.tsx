import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Genre } from './models';
import { AppContext } from './ResponsiveDrawer';

type GenresComponentProps = { fetcher: Fetcher };

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
    this.context.setTitle('Genres');
    this.props
      .fetcher({
        query: `query GenresQuery {
  genres {
    id
    name
  }
}
`,
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

GenresComponent.contextType = AppContext;
