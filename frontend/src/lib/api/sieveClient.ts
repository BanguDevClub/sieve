import type {
  CsvMetadata,
  PageRequest,
  PageResult,
  SqlResult,
  ProgressUpdate,
  ParquetOptimizationOptions,
  OptimizationResult,
} from '$lib/types';

// Check if running inside Tauri desktop environment
export function isTauri(): boolean {
  return (
    typeof window !== 'undefined' &&
    ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)
  );
}

// Global listener for progress events
let progressCallback: ((update: ProgressUpdate) => void) | null = null;

export function onProgressUpdate(cb: (update: ProgressUpdate) => void) {
  progressCallback = cb;
}

// In Tauri, attach listener
if (isTauri()) {
  import('@tauri-apps/api/event').then(({ listen }) => {
    listen<ProgressUpdate>('sieve://progress', (event) => {
      progressCallback?.(event.payload);
    });
  });
}

// -------------------------------------------------------------
// Mock Data Generator for Browser Preview Mode
// -------------------------------------------------------------
const MOCK_COLUMNS = [
  { name: 'id', data_type: 'INTEGER' },
  { name: 'full_name', data_type: 'VARCHAR' },
  { name: 'email', data_type: 'VARCHAR' },
  { name: 'department', data_type: 'VARCHAR' },
  { name: 'job_title', data_type: 'VARCHAR' },
  { name: 'salary', data_type: 'DOUBLE' },
  { name: 'country', data_type: 'VARCHAR' },
  { name: 'city', data_type: 'VARCHAR' },
  { name: 'is_active', data_type: 'BOOLEAN' },
  { name: 'score', data_type: 'DOUBLE' },
  { name: 'hire_date', data_type: 'DATE' },
  { name: 'project_count', data_type: 'INTEGER' },
  { name: 'lead_engineer', data_type: 'VARCHAR' },
  { name: 'budget_code', data_type: 'VARCHAR' },
  { name: 'region', data_type: 'VARCHAR' },
  { name: 'cost_center', data_type: 'VARCHAR' },
  { name: 'division', data_type: 'VARCHAR' },
  { name: 'seniority_level', data_type: 'VARCHAR' },
  { name: 'clearance', data_type: 'VARCHAR' },
  { name: 'notes', data_type: 'VARCHAR' },
];

for (let i = 21; i <= 60; i++) {
  MOCK_COLUMNS.push({ name: `metric_col_${i}`, data_type: i % 2 === 0 ? 'DOUBLE' : 'INTEGER' });
}

let mockRows: any[][] = [];
function generateMockRows(count = 100000) {
  if (mockRows.length > 0) return;
  const depts = ['Engineering', 'Data Science', 'Operations', 'Finance', 'Design', 'Legal', 'Marketing'];
  const countries = ['Brazil', 'USA', 'Germany', 'Japan', 'UK', 'Canada', 'France', 'Australia'];
  const names = ['Ana Silva', 'Lucas Santos', 'Sophia Meyer', 'Mateo Costa', 'Elena Vance', 'Liam Chen', 'Beatriz Lima', 'Arthur Pendelton', 'Clara Rocha', 'Gabriel Souza'];

  for (let i = 1; i <= count; i++) {
    const row = [
      i,
      `${names[i % names.length]} ${i}`,
      `user${i}@bangudev.club`,
      depts[i % depts.length],
      `Specialist ${((i % 5) + 1)}`,
      Math.round(45000 + (i * 137) % 110000),
      countries[i % countries.length],
      `Metro ${(i % 30) + 1}`,
      i % 6 !== 0,
      Math.round((((i * 3.7) % 100) + 0.1) * 10) / 10,
      `2023-${String((i % 12) + 1).padStart(2, '0')}-${String((i % 28) + 1).padStart(2, '0')}`,
      (i % 15) + 1,
      `Lead ${((i % 10) + 1)}`,
      `BC-${(i % 50) + 100}`,
      `Region-${(i % 4) + 1}`,
      `CC-${(i % 8) + 10}`,
      `Div-${(i % 6) + 1}`,
      `L${(i % 5) + 1}`,
      i % 3 === 0 ? 'Top Secret' : 'Standard',
      `Auto-generated record #${i} for Sieve benchmark`,
    ];

    for (let c = 21; c <= 60; c++) {
      row.push(c % 2 === 0 ? Math.round((i * 1.414) % 10000) / 100 : (i * 7) % 500);
    }
    mockRows.push(row);
  }
}

// -------------------------------------------------------------
// Sieve API Client Methods
// -------------------------------------------------------------

export async function pickCsvFile(): Promise<string | null> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<string | null>('pick_csv_file');
  }
  return null;
}

export async function loadSampleDataset(): Promise<CsvMetadata> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<CsvMetadata>('load_sample_dataset');
  }
  return openCsv('__sample__');
}

export async function openCsv(filePath: string): Promise<CsvMetadata> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<CsvMetadata>('open_csv_file', { filePath });
  }

  // Simulated browser delay with realistic multi-phase progress updates
  generateMockRows(100000);
  const phases = [
    { phase: 'Initializing', pct: 15, msg: 'Mapping CSV into DuckDB virtual view...' },
    { phase: 'Inspecting Schema', pct: 40, msg: 'Inferring 60 column types...' },
    { phase: 'Counting Records', pct: 75, msg: 'Counting 100,000 records in parallel...' },
    { phase: 'Complete', pct: 100, msg: '100,000 records ready for querying.' },
  ];

  for (let p of phases) {
    progressCallback?.({
      phase: p.phase,
      percentage: p.pct,
      elapsed_ms: p.pct * 10,
      estimated_rows: 100000,
      message: p.msg,
    });
    await new Promise((r) => setTimeout(r, 140));
  }

  return {
    file_path: filePath,
    file_name: filePath.split('/').pop() || 'huge_dataset_100k.csv',
    file_size_bytes: 48500000,
    total_rows: 100000,
    total_columns: MOCK_COLUMNS.length,
    columns: MOCK_COLUMNS,
    load_time_ms: 560,
  };
}

export async function queryPage(req: PageRequest): Promise<PageResult> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<PageResult>('query_page', { req });
  }

  generateMockRows();
  const start = performance.now();

  let filtered = mockRows;
  if (req.filters && req.filters.length > 0) {
    filtered = mockRows.filter((row) => {
      const matches = req.filters.map((f) => {
        const colIdx = MOCK_COLUMNS.findIndex((c) => c.name === f.column);
        if (colIdx < 0) return true;
        const cellVal = String(row[colIdx] ?? '').toLowerCase();
        const searchVal = f.value.toLowerCase().trim();

        switch (f.operator) {
          case 'equals':
            return cellVal === searchVal;
          case 'contains':
          case 'fuzzy':
            return cellVal.includes(searchVal);
          case 'starts_with':
            return cellVal.startsWith(searchVal);
          case 'ends_with':
            return cellVal.endsWith(searchVal);
          case 'in':
            const items = searchVal.split(',').map((s) => s.trim());
            return items.includes(cellVal);
          case 'gt':
            return Number(row[colIdx]) > Number(f.value);
          case 'lt':
            return Number(row[colIdx]) < Number(f.value);
          case 'gte':
            return Number(row[colIdx]) >= Number(f.value);
          case 'lte':
            return Number(row[colIdx]) <= Number(f.value);
          case 'is_empty':
            return row[colIdx] === null || String(row[colIdx]).trim() === '';
          case 'is_not_empty':
            return row[colIdx] !== null && String(row[colIdx]).trim() !== '';
          default:
            return true;
        }
      });

      return req.filter_conjunction === 'OR'
        ? matches.some(Boolean)
        : matches.every(Boolean);
    });
  }

  // Sort if requested
  if (req.sort) {
    const colIdx = MOCK_COLUMNS.findIndex((c) => c.name === req.sort?.column);
    if (colIdx >= 0) {
      const asc = req.sort.ascending ? 1 : -1;
      filtered = [...filtered].sort((a, b) => {
        if (a[colIdx] < b[colIdx]) return -1 * asc;
        if (a[colIdx] > b[colIdx]) return 1 * asc;
        return 0;
      });
    }
  }

  const page = Math.max(1, req.page);
  const pageSize = Math.min(100, Math.max(1, req.page_size));
  const offset = (page - 1) * pageSize;
  const pagedRows = filtered.slice(offset, offset + pageSize);

  const colOffset = req.col_offset || 0;
  const colLimit = Math.min(50, req.col_limit || 50);
  const displayedColumns = MOCK_COLUMNS.slice(colOffset, colOffset + colLimit);

  const slicedData = pagedRows.map((r) => r.slice(colOffset, colOffset + colLimit));

  return {
    page,
    page_size: pageSize,
    total_filtered_rows: filtered.length,
    total_pages: Math.ceil(filtered.length / pageSize),
    col_offset: colOffset,
    col_limit: displayedColumns.length,
    total_columns: MOCK_COLUMNS.length,
    displayed_columns: displayedColumns,
    rows: slicedData,
    execution_time_ms: Math.round(performance.now() - start),
  };
}

export async function runCustomSql(
  query: string,
  maxRows = 100
): Promise<SqlResult> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<SqlResult>('run_custom_sql', { query, maxRows });
  }

  const start = performance.now();
  generateMockRows();

  // Simple mock handling for common queries
  let cols = MOCK_COLUMNS.slice(0, 8);
  let rows = mockRows.slice(0, Math.min(maxRows, 50)).map((r) => r.slice(0, 8));

  if (query.toLowerCase().includes('count(*)')) {
    cols = [{ name: 'total_records', data_type: 'BIGINT' }];
    rows = [[mockRows.length]];
  } else if (query.toLowerCase().includes('group by department')) {
    cols = [
      { name: 'department', data_type: 'VARCHAR' },
      { name: 'count', data_type: 'BIGINT' },
      { name: 'avg_salary', data_type: 'DOUBLE' },
    ];
    rows = [
      ['Engineering', 28571, 98450.25],
      ['Data Science', 14285, 104200.8],
      ['Operations', 14285, 62100.5],
      ['Finance', 14285, 87400.0],
      ['Design', 14285, 76300.0],
      ['Marketing', 14285, 71200.0],
    ];
  }

  return {
    columns: cols,
    rows,
    total_rows: rows.length,
    execution_time_ms: Math.round(performance.now() - start),
    sql_query: query,
  };
}

export async function exportData(
  targetPath: string,
  format: string,
  customSql?: string
): Promise<string> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<string>('export_data', { targetPath, format, customSql });
  }

  await new Promise((r) => setTimeout(r, 400));
  return `Exported mock dataset to ${targetPath} (${format.toUpperCase()})`;
}

export async function cancelOperation(): Promise<void> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<void>('cancel_current_operation');
  }
}

export async function optimizeToParquet(
  options: ParquetOptimizationOptions
): Promise<OptimizationResult> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<OptimizationResult>('optimize_to_parquet', { options });
  }

  // Web/Mock preview fallback
  await new Promise((r) => setTimeout(r, 600));
  return {
    target_path: options.target_path,
    original_size_bytes: 52428800, // 50 MB
    parquet_size_bytes: 8388608,   // 8 MB
    space_saved_percent: 84.0,
    execution_time_ms: 320,
    total_rows: 100000,
  };
}

export async function pickSaveParquetFile(defaultName?: string): Promise<string | null> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<string | null>('pick_save_parquet_file', { defaultName });
  }
  return defaultName || 'optimized_dataset.parquet';
}
