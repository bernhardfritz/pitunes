import {
  createStyles,
  List,
  ListItem,
  ListItemText,
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
import { AppAction, AppActionType } from './App';
import { rotateRight } from './rotateRight';
// eslint-disable-next-line import/no-webpack-loader-syntax
import ArtistQuery from '!!raw-loader!./graphql/ArtistQuery.graphql';

const styles = (theme: Theme) =>
  createStyles({
    ul: {
      backgroundColor: theme.palette.background.default,
      padding: 0,
    },
    listSubheader: {
      top: 48,
    },
  });

type ArtistComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
} & RouteComponentProps<{
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
        query: ArtistQuery,
        operationName: 'ArtistQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { artist } = res.data;
        this.props.dispatch({
          type: AppActionType.UPDATE_TITLE,
          title: artist.name,
        });
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
            <ListSubheader className={this.props.classes.listSubheader}>Albums</ListSubheader>
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
            <ListSubheader className={this.props.classes.listSubheader}>Tracks</ListSubheader>
            {tracks.map((track, index) => (
              <ListItem
                button
                key={track.id}
                onClick={(_) =>
                  this.props.dispatch({
                    type: AppActionType.UPDATE_QUEUE,
                    queue: rotateRight([...tracks], index),
                  })
                }
              >
                <ListItemText primary={track.name} />
              </ListItem>
            ))}
          </ul>
        </li>
      </List>
    );
  }
}

export default withStyles(styles, { withTheme: true })(ArtistComponent);
