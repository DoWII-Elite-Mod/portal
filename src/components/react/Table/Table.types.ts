import type { ColumnDef, PaginationState } from '@tanstack/react-table';
import type { Dispatch, SetStateAction } from 'react';

export interface ITableProps<TData> {
  /**
   * Provided data array.
   */
  data: Array<TData>;
  /**
   * Manages table structure and cells.
   * For full column API, infer type ColumnDef exported from @efectivo/table
   */
  columns: Array<ColumnDef<TData>>;
  /**
   * Number of elements. Displayed in table's toolbar.
   */
  rowCount: number;
  /**
   * Empty View Table text.
   */
  tableEmptyViewText: string;
  /**
   * Number of elements in provided data set. Displayed in table's toolbar.
   */
  rowCountText: string;
  /**
   * Toolbar Search input placeholder text.
   */
  searchFilterPlaceholderText?: string;
  /**
   * Value from API. Sum of pages provided by paging.
   */
  pageCount: number;
  /**
   * Pagination object provided by state for controlled pagination.
   *   pageIndex: number - initial starting page index
   *   pageSize: number - number of rows rendered in table
   */
  pagination: PaginationState;
  /**
   * Regular set state function for setting new pagination object by table.
   */
  setPagination: Dispatch<SetStateAction<PaginationState>>;
  /**
   * Loading flag when fetching new data.
   */
  isLoading?: boolean;
  /**
   *   Search field visible
   */
  isSearchable?: boolean;
}
