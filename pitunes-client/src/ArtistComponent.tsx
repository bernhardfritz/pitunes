import {
  createStyles,
  List,
  ListSubheader,
  Theme,
  WithStyles,
  withStyles,
} from '@material-ui/core';
import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { Fetcher } from './fetcher';
import ListItemLink from './ListItemLink';
import { Album, Track } from './models';
import { AppContext } from './ResponsiveDrawer';

const styles = (theme: Theme) =>
  createStyles({
    ul: {
      backgroundColor: theme.palette.background.default,
      padding: 0,
    },
  });

type ArtistComponentProps = { fetcher: Fetcher } & RouteComponentProps<{
  id: string;
}> &
  WithStyles<typeof styles, true>;

type ArtistComponentState = { albums: Album[]; tracks: Track[] };

class ArtistComponent extends React.Component<
  ArtistComponentProps,
  ArtistComponentState
> {
  constructor(props: ArtistComponentProps) {
    super(props);
    this.state = {
      albums: [],
      tracks: [],
    };
  }

  componentDidMount() {
    this.props
      .fetcher({
        query: `query ArtistQuery($id: ID!) {
  artist(id: $id) {
    id
    name
    albums {
      id
      name
    }
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
        operationName: 'ArtistQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { artist } = res.data;
        this.context.setTitle(artist.name);
        this.setState({
          albums: artist.albums,
          tracks: artist.tracks,
        });
      });
  }

  render() {
    const { albums, tracks } = this.state;
    return (
      <List subheader={<li />}>
        <li>
          <ul className={this.props.classes.ul}>
            <ListSubheader>Albums</ListSubheader>
            {albums.map((album) => (
              <ListItemLink
                key={album.id}
                to={`/albums/${album.id}`}
                primary={album.name}
              ></ListItemLink>
            ))}
          </ul>
        </li>
        <li>
          <ul className={this.props.classes.ul}>
            <ListSubheader>Tracks</ListSubheader>
            {tracks.map((track) => (
              <ListItemLink
                key={track.id}
                to={`/tracks/${track.id}`}
                primary={track.name}
              ></ListItemLink>
            ))}
          </ul>
        </li>
      </List>
    );
  }
}

ArtistComponent.contextType = AppContext;

export default withStyles(styles, { withTheme: true })(ArtistComponent);
