import React from 'react';
import { AppContext } from './ResponsiveDrawer';

type UploadComponentState = { responseText: string };

type UploadComponentProps = {};

export class UploadComponent extends React.Component<
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
    this.context.setTitle('Upload');
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

UploadComponent.contextType = AppContext;
