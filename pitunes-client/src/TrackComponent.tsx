import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { Fetcher } from './fetcher';
import { Track } from './models';
import { AppContext } from './ResponsiveDrawer';

type TrackComponentProps = { fetcher: Fetcher } & RouteComponentProps<{
  id: string;
}>;
type TrackComponentState = { track?: Track };

export default class TrackComponent extends React.Component<
  TrackComponentProps,
  TrackComponentState
> {
  constructor(props: TrackComponentProps) {
    super(props);
    this.setState({});
  }

  componentDidMount() {
    this.props
      .fetcher({
        query: `query TrackQuery($id: ID!) {
  track(id: $id) {
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
}`,
        operationName: 'TrackQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { track } = res.data;
        this.context.setTitle(track.name);
        this.context.setTrack(track);
        this.setState({ track });
      });
  }

  render() {
    return <div></div>;
  }
}

TrackComponent.contextType = AppContext;
