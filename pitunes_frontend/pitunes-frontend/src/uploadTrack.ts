export const uploadTrack = (
  file: File,
  progressHandler: (event: ProgressEvent) => void
) =>
  new Promise((resolve, reject) => {
    const formData = new FormData();
    formData.append('file', file);
    const xhr = new XMLHttpRequest();
    xhr.responseType = 'json';
    xhr.onload = () => {
      if (xhr.status === 201) {
        resolve(xhr.response);
      } else {
        reject(
          `Unexpected HTTP response status code: ${xhr.status} ${xhr.statusText}`
        );
      }
    };
    xhr.onerror = () => {
      reject('The upload failed due to an error');
    };
    xhr.upload.onprogress = progressHandler;
    xhr.open('POST', '/api/tracks');
    xhr.send(formData);
  });
