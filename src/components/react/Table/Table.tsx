import type { SortingState } from '@tanstack/react-table';
import {
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  useReactTable
} from '@tanstack/react-table';
import { useMemo, useState } from 'react';

import { TableHeaders, TableRows } from './components';
import type { ITableProps } from './Table.types';

export const Table = <TData,>({
  data,
  columns,
  rowCount,
  rowCountText,
  searchFilterPlaceholderText,
  tableEmptyViewText,
  pagination,
  setPagination,
  pageCount,
  isLoading = false
}: ITableProps<TData>) => {
  const [sorting, setSorting] = useState<SortingState>([]);

  const tableData = useMemo(
    () => (isLoading ? (Array(6).fill({}) as Array<TData>) : data),
    [isLoading, data]
  );

  const {
    getHeaderGroups,
    getRowModel,
    setPageIndex,
    nextPage,
    previousPage,
    getCanNextPage,
    getCanPreviousPage
  } = useReactTable({
    data: tableData,
    columns,
    pageCount,

    state: {
      sorting
    },
    onPaginationChange: setPagination,
    getFilteredRowModel: getFilteredRowModel(),
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    manualPagination: true
  });

  const handleNextPage = () => nextPage();
  const handlePreviousPage = () => previousPage();

  console.log('getRowModel', getRowModel());

  return (
    <div className="overflow-x-auto rounded-3xl border border-gray-900 bg-gray-900">
      <table className="w-full border-separate border-spacing-y-2">
        <thead>
          {getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((headerData) => (
                <TableHeaders key={headerData.id} {...headerData} />
              ))}
            </tr>
          ))}
        </thead>
        <tbody className="bg-gray-900">
          {getRowModel().rows.map((row) => (
            <TableRows key={row.id} isLoading={isLoading} {...row} />
          ))}
        </tbody>
      </table>
    </div>
  );
};
