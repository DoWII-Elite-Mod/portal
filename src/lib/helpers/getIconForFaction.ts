import type { TFactions } from '@/shared/types';

export const getIconForFaction = (faction: TFactions) => {
  switch (faction) {
    case 'Eldars':
      return '/images/factions/eldars.png';
    case 'Chaos Space Marines':
      return '/images/factions/chaos.png';
    case 'Orks':
      return '/images/factions/ork.png';
    case 'Imperial Guard':
      return '/images/factions/ig.png';
    case 'Space Marines':
      return '/images/factions/sm.png';
    case 'Ordo Malleus':
      return '/images/factions/om.png';
    case 'Tyranids':
      return '/images/factions/tyranids.png';
    default:
      return '';
  }
};
