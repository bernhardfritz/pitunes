import React from 'react';
import { useAudio } from './useAudio';

export interface WithAudio {
  paused: boolean;
  currentTime: number;
  play: (src: string) => void;
  togglePaused: () => void;
  seek: (currentTime: number) => void;
}

export const withAudio = (Component: any) => {
  return (props: any) => {
    const [paused, currentTime, play, togglePaused, seek] = useAudio();

    const withAudioProps: WithAudio = {
      paused,
      currentTime,
      play,
      togglePaused,
      seek,
    };

    return <Component {...withAudioProps} {...props}></Component>;
  };
};
