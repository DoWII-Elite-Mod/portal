import type { Article } from '@/shared/types';

import { getRandomFromArray } from './getRandomElementFromArray';

export const articles: Article[] = [
  ...[0, 1, 2, 4].map((_) => ({
    title: 'Lorem ipsum dolor',
    content:
      'Lorem ipsum dolor sit amet consectetur adipisicing elit. Delectus consectetur perspiciatis corrupti officia laboriosam quisquam? Ipsa tempora laborum voluptate. Explicabo temporibus repudiandae ea! Iure delectus inventore quaerat soluta nesciunt quidem.',
    date: 'September 5, 2022'
  }))
];

const getFactionImgSrc = (faction: string) => `/images/factions/${faction}.png`;

const FactionLabels: Record<string, string> = {
  chaos: 'Chaos Space Marines',
  eldars: 'Eldars',
  om: 'Ordo Malleus',
  ig: 'Imperial Guard',
  ork: 'Orks',
  sm: 'Space Marines',
  tyranids: 'Tyranids'
};

export const factions = [
  'chaos',
  'eldars',
  'om',
  'ig',
  'ork',
  'sm',
  'tyranids'
];

export const factionImageLinks = factions.map(getFactionImgSrc);

export const generateFaction = () => {
  const faction = getRandomFromArray(factions);

  return {
    name: FactionLabels[faction],
    imgSrc: getFactionImgSrc(faction)
  };
};
