import { ListItem, ListItemText } from '@material-ui/core';
import React from 'react';
import { AppActionType, AppContext } from './App';
import { Track } from './models';
import { rotateRight } from './rotateRight';

type TrackListItemProps = { tracks: Track[] };

export const TrackListItems = ({ tracks }: TrackListItemProps) => (
  <AppContext.Consumer>
    {({ dispatch }) =>
      tracks.map((track, index) => (
        <ListItem
          key={track.id}
          button
          onClick={() =>
            dispatch({
              type: AppActionType.UPDATE_QUEUE,
              queue: rotateRight([...tracks], index),
            })
          }
        >
          <ListItemText primary={track.name} />
        </ListItem>
      ))
    }
  </AppContext.Consumer>
);
