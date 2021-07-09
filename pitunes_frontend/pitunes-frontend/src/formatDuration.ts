export const formatDuration = (durationInMillis: number) => {
    const durationInSeconds = durationInMillis / 1000;
    const minutes = Math.floor(durationInSeconds / 60);
    const seconds = Math.floor(durationInSeconds - minutes * 60);
    return `${minutes}:${String(seconds).padStart(2, '0')}`;
}