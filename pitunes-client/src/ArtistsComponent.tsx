import { List } from '@material-ui/core';
import React from 'react';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Artist } from './models';
import { AppContext } from './ResponsiveDrawer';

type ArtistsComponentProps = { fetcher: Fetcher };

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
    this.context.setTitle('Artists');
    this.props
      .fetcher({
        query: `query ArtistsQuery {
  artists {
    id
    name
  }
}
`,
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

ArtistsComponent.contextType = AppContext;
