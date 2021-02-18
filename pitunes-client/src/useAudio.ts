import { useEffect, useState } from 'react';

const audio = new Audio();

export const useAudio: () => [
  boolean,
  number,
  (src: string) => void,
  () => void,
  (currentTime: number) => void,
] = () => {
  const [paused, setPaused] = useState(audio.paused);
  const [currentTime, setCurrentTime] = useState(audio.currentTime);
  const play = (src: string) => {
    audio.src = src;
    audio.play();
  };
  const togglePaused = () => (audio.paused ? audio.play() : audio.pause());
  const seek = (currentTime: number) => (audio.currentTime = currentTime);

  useEffect(() => {
    const onPausedChange = () => setPaused(audio.paused);
    const onCurrentTimeChange = () => setCurrentTime(audio.currentTime);
    audio.addEventListener('play', onPausedChange);
    audio.addEventListener('pause', onPausedChange);
    audio.addEventListener('timeupdate', onCurrentTimeChange);
    return () => {
      audio.removeEventListener('play', onPausedChange);
      audio.removeEventListener('pause', onPausedChange);
      audio.removeEventListener('timeupdate', onCurrentTimeChange);
    };
  }, []);

  return [paused, currentTime, play, togglePaused, seek];
};
