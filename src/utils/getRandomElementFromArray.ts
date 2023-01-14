export const getRandomFromArray = (arr: string[]) =>
  arr[Math.floor(Math.random() * arr.length)] ?? '';
