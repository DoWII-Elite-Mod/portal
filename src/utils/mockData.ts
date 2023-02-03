import type { Article } from '@/shared/types';

export const articles: Article[] = [
  ...[0, 1, 2, 4].map((_) => ({
    title: 'Lorem ipsum dolor',
    content:
      'Lorem ipsum dolor sit amet consectetur adipisicing elit. Delectus consectetur perspiciatis corrupti officia laboriosam quisquam? Ipsa tempora laborum voluptate. Explicabo temporibus repudiandae ea! Iure delectus inventore quaerat soluta nesciunt quidem.',
    date: 'September 5, 2022'
  }))
];

export const factionImageLinks = [
  'chaos',
  'eldar',
  'gk',
  'ig',
  'ork',
  'sm',
  'tyranid'
].map((link) => `/images/factions/${link}.png`);
