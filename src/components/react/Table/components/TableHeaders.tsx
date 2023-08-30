import type { Header } from '@tanstack/react-table';
import { flexRender } from '@tanstack/react-table';

import { clsxm } from '@/utils';

export const TableHeaders = <TData,>({
  column,
  getContext,
  id,
  colSpan,
  isPlaceholder
}: Header<TData, unknown>) => {
  const title = column.columnDef.header;

  return (
    <th
      key={id}
      colSpan={colSpan} 
      className="px-6 py-3 text-left text-sm font-medium tracking-wider text-gray-500"
    >
      {isPlaceholder ? null : (
        <div
          role="button"
          tabIndex={0}
          onKeyDown={column.getToggleSortingHandler()}
          className={clsxm(
            'flex flex-row items-center gap-2',
            column.getCanSort() && 'cursor-pointer select-none'
          )}
          onClick={column.getToggleSortingHandler()}
        >
          {flexRender(title, getContext())}
          {{
            asc: <div>up</div>,
            desc: <div>down</div>
          }[column.getIsSorted() as string] ??
            (title !== '' && <div>sort</div>)}
        </div>
      )}
    </th>
  );
};
