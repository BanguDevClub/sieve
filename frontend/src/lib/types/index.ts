export type Theme =
  | 'dark'
  | 'light'
  | 'catppuccin-latte'
  | 'catppuccin-frappe'
  | 'catppuccin-macchiato'
  | 'catppuccin-mocha';

export interface ColumnInfo {
  name: string;
  data_type: string;
}

export interface CsvMetadata {
  file_path: string;
  file_name: string;
  file_size_bytes: number;
  total_rows: number;
  total_columns: number;
  columns: ColumnInfo[];
  load_time_ms: number;
  file_format?: 'csv' | 'parquet' | 'excel' | 'json' | 'tsv' | string;
  is_optimized?: boolean;
}

export interface ParquetOptimizationOptions {
  target_path: string;
  source_path?: string | null;
  compression: 'ZSTD' | 'SNAPPY' | 'GZIP' | 'UNCOMPRESSED';
  row_group_size: number;
  sort_column?: string | null;
}

export interface OptimizationResult {
  target_path: string;
  original_size_bytes: number;
  parquet_size_bytes: number;
  space_saved_percent: number;
  execution_time_ms: number;
  total_rows: number;
}

export type FilterOperator =
  | 'equals'
  | 'contains'
  | 'fuzzy'
  | 'starts_with'
  | 'ends_with'
  | 'in'
  | 'gt'
  | 'lt'
  | 'gte'
  | 'lte'
  | 'is_empty'
  | 'is_not_empty';

export interface FilterRule {
  id: string;
  column: string;
  operator: FilterOperator;
  value: string;
}

export interface SortRule {
  column: string;
  ascending: boolean;
}

export interface PageRequest {
  page: number;
  page_size: number;
  col_offset: number;
  col_limit: number;
  filters: FilterRule[];
  filter_conjunction: 'AND' | 'OR';
  sort?: SortRule | null;
}

export interface PageResult {
  page: number;
  page_size: number;
  total_filtered_rows: number;
  total_pages: number;
  col_offset: number;
  col_limit: number;
  total_columns: number;
  displayed_columns: ColumnInfo[];
  rows: (string | number | boolean | null)[][];
  execution_time_ms: number;
}

export interface SqlResult {
  columns: ColumnInfo[];
  rows: (string | number | boolean | null)[][];
  total_rows: number;
  execution_time_ms: number;
  sql_query: string;
}

export interface ProgressUpdate {
  phase: string;
  percentage: number;
  elapsed_ms: number;
  estimated_rows: number;
  message: string;
}

export type ActiveTab = 'grid' | 'sql' | 'schema' | 'about';
