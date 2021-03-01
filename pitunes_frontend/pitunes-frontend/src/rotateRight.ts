export const rotateRight = <T>(array: T[], n: number) => {
  n = n % array.length;
  if (n < 0) {
    n += array.length;
  }
  array.push(...array.splice(0, n));
  return array;
};
