const UNITS = ['B', 'kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

const round = (x: number, n: number) => {
  const f = Math.pow(10, n);
  return Math.round((x + Number.EPSILON) * f) / f;
};

export const formatBytes = (bytes: number) => {
  let i;
  for (i = 0; bytes >= 1000; i++) {
    bytes /= 1000;
  }
  return `${round(bytes, 2)} ${UNITS[i]}`;
};
