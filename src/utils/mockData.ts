import type { Article } from '@/shared/types';

import watchIcon from '../../public/static/ran_improved_focus_large.png';
import playIcon from '../../public/static/sta_terminator_honors_large.png';
import competeIcon from '../../public/static/str_power_sword_aptit_large.png';

export const pageElements = [
  {
    title: 'Play',
    description: 'Download the Dawn of War II: Elite mod and join the games.',
    icon: playIcon
  },
  {
    title: 'Compete',
    description:
      'Get rated on our custom leaderboards or play in our monthly tournaments.',
    icon: competeIcon
  },
  {
    title: 'Watch',
    description: 'See live streams or browse 1600 shoutcasted Elite matches.',
    icon: watchIcon
  }
];

export const articles: Article[] = [
  {
    title: 'Tournament 1v1',
    content: 'Awesome 1v1 tournament',
    date: 'December 1, 2022',
    tournament: true
  },
  ...[0, 1, 2].map(_ => ({
    title: 'Lorem ipsum dolor',
    content:
      'Lorem ipsum dolor sit amet consectetur adipisicing elit. Delectus consectetur perspiciatis corrupti officia laboriosam quisquam? Ipsa tempora laborum voluptate. Explicabo temporibus repudiandae ea! Iure delectus inventore quaerat soluta nesciunt quidem.',
    date: 'September 5, 2022'
  }))
];
