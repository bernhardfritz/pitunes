import React, { useState } from 'react';

type UploadComponentState = { responseText: string };

export const UploadComponent = () => {
  const [state, setState] = useState<UploadComponentState>({
    responseText: '',
  });

  const handleUpload = async (event: any) => {
    const formData = new FormData();
    for (const file of event.target.files) {
      formData.append('file', file);
    }
    const responseText = await fetch('/api/tracks', {
      method: 'post',
      body: formData,
    }).then((res) => res.text());
    setState({ responseText });
  };

  return (
    <>
      <input type="file" onChange={handleUpload} multiple></input>
      <textarea value={state.responseText} readOnly></textarea>
    </>
  );
};
