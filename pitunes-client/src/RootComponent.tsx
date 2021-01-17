import React from 'react';
import { AppAction, AppActionType } from './App';

type RootComponentProps = { dispatch: React.Dispatch<AppAction> };

type RootComponentState = {};

export default class RootComponent extends React.Component<
  RootComponentProps,
  RootComponentState
> {
  constructor(props: RootComponentProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.props.dispatch({ type: AppActionType.UPDATE_TITLE, title: 'piTunes' });
  }

  render() {
    return null; // TODO
  }
}
