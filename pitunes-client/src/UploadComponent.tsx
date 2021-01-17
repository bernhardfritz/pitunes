import React from 'react';
import { AppAction, AppActionType } from './App';

type UploadComponentProps = { dispatch: React.Dispatch<AppAction> };

type UploadComponentState = { responseText: string };

export default class UploadComponent extends React.Component<
  UploadComponentProps,
  UploadComponentState
> {
  constructor(props: UploadComponentProps) {
    super(props);
    this.state = {
      responseText: '',
    };
  }

  componentDidMount() {
    this.props.dispatch({ type: AppActionType.UPDATE_TITLE, title: 'Upload' });
  }

  handleUpload = async (event: any) => {
    const formData = new FormData();
    for (const file of event.target.files) {
      formData.append('file', file);
    }
    const responseText = await fetch('/api/tracks', {
      method: 'post',
      body: formData,
    }).then((res) => res.text());
    this.setState({ responseText });
  };

  render() {
    return (
      <div>
        <input type="file" onChange={this.handleUpload} multiple></input>
        <textarea value={this.state.responseText} readOnly></textarea>
      </div>
    );
  }
}
