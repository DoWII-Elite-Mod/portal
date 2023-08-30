import type { Row } from '@tanstack/react-table';
import { flexRender } from '@tanstack/react-table';

import { clsxm } from '@/utils';

import { TableCellSkeleton } from './TableCellSkeleton';

type TableRowsProps<TData> = Row<TData> & { isLoading: boolean };
export const TableRows = <TData,>({
  getVisibleCells,
  isLoading
}: TableRowsProps<TData>) => (
  <tr>
    {getVisibleCells().map((cell) => {
      console.log('cell.column.columnDef.cell', cell.column.columnDef.cell);

      return (
        <td
          key={cell.id}
          className={clsxm(
            'truncate border-y px-6 py-7',
            'border-gray-200 first:rounded-l-2xl first:border-l last:rounded-r-2xl last:border-r'
          )}
          role="cell"
        >
          {isLoading ? (
            <TableCellSkeleton />
          ) : (
            flexRender(cell.column.columnDef.cell, cell.getContext())
          )}
        </td>
      );
    })}
  </tr>
);
