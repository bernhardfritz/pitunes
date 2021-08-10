type Duration = {
  ms: number;
  s: number;
  min: number;
  h: number;
};

const toDuration = (durationInMillis: number): Duration => {
  const ms = durationInMillis % 1000;
  durationInMillis = (durationInMillis - ms) / 1000;
  const s = durationInMillis % 60;
  durationInMillis = (durationInMillis - s) / 60;
  const min = durationInMillis % 60;
  durationInMillis = (durationInMillis - min) / 60;
  const h = durationInMillis;

  return {
    ms,
    s,
    min,
    h,
  };
};

export const formatDuration = (durationInMillis: number) => {
  const duration = toDuration(durationInMillis);
  return (
    (duration.h > 0 ? `${String(duration.h).padStart(2, '0')}:` : '') +
    `${String(duration.min).padStart(2, '0')}:${String(duration.s).padStart(
      2,
      '0'
    )}`
  );
};

export const formatEta = (durationInMillis: number) => {
  const duration = toDuration(durationInMillis);
  return duration.h > 0
    ? `${duration.h} h ${duration.min} min ${duration.s} s`
    : duration.min > 0
    ? `${duration.min} min ${duration.s} s`
    : `${duration.s} s`;
};
