import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import { AppAction, AppActionType } from './App';
import { Fetcher } from './fetcher';
import { Track } from './models';
// eslint-disable-next-line import/no-webpack-loader-syntax
import TrackQuery from '!!raw-loader!./graphql/TrackQuery.graphql';

type TrackComponentProps = {
  dispatch: React.Dispatch<AppAction>;
  fetcher: Fetcher;
} & RouteComponentProps<{
  id: string;
}>;

type TrackComponentState = { track?: Track };

export default class TrackComponent extends React.Component<
  TrackComponentProps,
  TrackComponentState
> {
  constructor(props: TrackComponentProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.props
      .fetcher({
        query: TrackQuery,
        operationName: 'TrackQuery',
        variables: {
          id: this.props.match.params.id,
        },
      })
      .then((res) => {
        const { track } = res.data;
        this.props.dispatch({
          type: AppActionType.UPDATE_TITLE,
          title: track.name,
        });
        this.setState({ track });
      });
  }

  render() {
    return null; // TODO
  }
}
