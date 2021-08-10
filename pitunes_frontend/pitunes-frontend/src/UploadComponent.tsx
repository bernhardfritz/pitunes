import {
  Link,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@material-ui/core';
import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { formatBytes } from './formatBytes';
import { formatEta } from './formatDuration';
import { LinearProgressWithLabel } from './LinearProgressWithLabel';
import { UploadDropZoneComponent } from './UploadDropZoneComponent';
import { uploadTrack } from './uploadTrack';
import { useStateWithDispatchCallback } from './useStateWithDispatchCallback';

type UploadComponentProps = { playerVisible: boolean };

type UploadQueueItem = {
  file: File;
  loaded: number;
  total: number;
  timeStamp: number;
  speed: number;
  progress: number;
  trackId?: string;
};

type UploadComponentState = {
  uploadQueue: UploadQueueItem[];
};

export const UploadComponent = (props: UploadComponentProps) => {
  const [state, setState] = useStateWithDispatchCallback<UploadComponentState>({
    uploadQueue: [],
  });

  const handleUpload = (files: any) => {
    setState(
      ({ uploadQueue }) => ({
        uploadQueue: [
          ...uploadQueue,
          ...(() => {
            const uploadQueue = [];
            for (const file of files) {
              uploadQueue.push({
                file,
                loaded: 0,
                total: file.size,
                timeStamp: 0,
                speed: 0,
                progress: 0,
              });
            }
            return uploadQueue;
          })(),
        ],
      }),
      (state: UploadComponentState) => {
        state.uploadQueue.reduce(
          async (prev: any, curr: any, index: number) => {
            await prev;
            return uploadTrack(curr.file, (event) => {
              setState(({ uploadQueue }) => ({
                uploadQueue: [
                  ...uploadQueue.slice(0, index),
                  {
                    ...uploadQueue[index],
                    loaded: event.loaded,
                    total: event.total,
                    timeStamp: event.timeStamp,
                    speed:
                      (event.loaded - uploadQueue[index].loaded) /
                      ((event.timeStamp - uploadQueue[index].timeStamp) / 1000), // speed in bytes per second
                    progress: (event.loaded * 100) / event.total,
                  },
                  ...uploadQueue.slice(index + 1),
                ],
              }));
            }).then((res: any) => {
              const trackId = /.*\/(.*)\.mp3$/.exec(res[0])?.[1];
              setState(({ uploadQueue }) => ({
                uploadQueue: [
                  ...uploadQueue.slice(0, index),
                  {
                    ...uploadQueue[index],
                    trackId,
                  },
                  ...uploadQueue.slice(index + 1),
                ],
              }));
            });
          },
          Promise.resolve()
        );
      }
    );
  };

  return state.uploadQueue.length === 0 ? (
    <UploadDropZoneComponent
      playerVisible={props.playerVisible}
      onUpload={handleUpload}
    ></UploadDropZoneComponent>
  ) : (
    <TableContainer>
      <Table>
        <colgroup>
          <col width="40%" />
          <col width="10%" />
          <col width="10%" />
          <col width="10%" />
          <col width="10%" />
          <col width="20%" />
        </colgroup>
        <TableHead>
          <TableRow>
            <TableCell>Filename</TableCell>
            <TableCell align="right">Uploaded</TableCell>
            <TableCell align="right">Size</TableCell>
            <TableCell align="right">Speed</TableCell>
            <TableCell align="right">ETA</TableCell>
            <TableCell align="right">Progress</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {state.uploadQueue.map((item) => (
            <TableRow key={item.file.name}>
              <TableCell>
                {item.trackId ? (
                  <Link
                    component={RouterLink}
                    to={`/tracks/${item.trackId}`}
                    target="_blank"
                    rel="noopener"
                  >
                    {item.file.name}
                  </Link>
                ) : (
                  item.file.name
                )}
              </TableCell>
              <TableCell align="right">{formatBytes(item.loaded)}</TableCell>
              <TableCell align="right">{formatBytes(item.total)}</TableCell>
              <TableCell align="right">
                {formatBytes(item.loaded < item.total ? item.speed : 0)}/s
              </TableCell>
              <TableCell align="right">
                {formatEta(
                  item.speed > 0
                    ? ((item.total - item.loaded) / item.speed) * 1000
                    : 0
                )}
              </TableCell>
              <TableCell align="right">
                <LinearProgressWithLabel value={item.progress} />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
