import type { ColumnDef } from '@tanstack/react-table';

import type { IPlayer } from './Ladder.astro';

export const columns: Array<ColumnDef<IPlayer>> = [
  {
    id: 'name',
    accessorKey: 'name',
    header: 'Name',
    cell: (info) => {
      const name = info.getValue<string>();
      const avatarUrl = info.row.original.avatar;
      const imageAlt = `${name} avatar`;

      return (
        <div className="flex flex-row items-center gap-3">
          <span className="font-bold text-gray-300">{info.row.index + 1}.</span>
          <img
            className="mr-2 inline-block h-12 w-12 rounded opacity-50 outline-1 outline-black/70 group-hover:opacity-100 md:mr-0"
            src={avatarUrl}
            alt={imageAlt}
          />
          <span className="text-gray-200">{name}</span>
        </div>
      );
    }
  },
  {
    id: 'faction',
    accessorKey: 'race',
    header: 'Faction'
  },
  {
    id: 'rating',
    accessorKey: 'rating',
    header: 'Rating'
  },
  {
    id: 'winRatio',
    accessorKey: 'winRatio',
    header: 'Win Ratio',
    cell: (info) => `${info.getValue()} %`
  }
];
