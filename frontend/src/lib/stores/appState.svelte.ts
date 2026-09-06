import type {
  Theme,
  CsvMetadata,
  PageResult,
  FilterRule,
  SortRule,
  ProgressUpdate,
  ActiveTab,
} from '$lib/types';
import * as api from '$lib/api/sieveClient';

export function getSameFolderParquetPath(sourcePath: string): string {
  if (!sourcePath) return 'dataset.parquet';
  const lastSlash = Math.max(sourcePath.lastIndexOf('/'), sourcePath.lastIndexOf('\\'));
  const dotIndex = sourcePath.lastIndexOf('.');
  if (dotIndex > lastSlash && dotIndex > 0) {
    return sourcePath.substring(0, dotIndex) + '.parquet';
  }
  return sourcePath + '.parquet';
}

class AppStateStore {
  // Theme state
  theme = $state<Theme>('dark');
  activeTab = $state<ActiveTab>('grid');

  // Loading & Progress
  isLoading = $state<boolean>(false);
  progress = $state<ProgressUpdate | null>(null);

  // Metadata & Data Results
  metadata = $state<CsvMetadata | null>(null);
  pageResult = $state<PageResult | null>(null);
  
  // Pagination (Strict: 100 rows, 50 columns)
  currentPage = $state<number>(1);
  pageSize = $state<number>(100);
  colOffset = $state<number>(0);
  colLimit = $state<number>(50);

  // Filters & Sorting
  filters = $state<FilterRule[]>([]);
  filterConjunction = $state<'AND' | 'OR'>('AND');
  sortRule = $state<SortRule | null>(null);

  // Interactive column widths
  columnWidths = $state<Record<string, number>>({});

  // Parquet Data Structure Optimization Modal
  isOptimizeModalOpen = $state<boolean>(false);
  pendingFile = $state<string | null>(null);
  pendingTargetParquet = $state<string>('');

  constructor() {
    if (typeof window !== 'undefined') {
      const savedTheme = localStorage.getItem('sieve_theme') as Theme;
      if (savedTheme) {
        this.setTheme(savedTheme);
      } else {
        this.setTheme('dark');
      }

      api.onProgressUpdate((update) => {
        this.progress = update;
      });
    }
  }

  setTheme(newTheme: Theme) {
    this.theme = newTheme;
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', newTheme);
      localStorage.setItem('sieve_theme', newTheme);
    }
  }

  setActiveTab(tab: ActiveTab) {
    this.activeTab = tab;
  }

  handleFileSelection(path: string) {
    if (!path) return;
    const ext = path.split('.').pop()?.toLowerCase() || '';
    if (ext === 'parquet' || ext === 'pq') {
      // Already an optimized Parquet file! Open directly.
      this.pendingFile = null;
      this.pendingTargetParquet = '';
      this.openFile(path);
    } else {
      // Offer Parquet optimization before opening, with target saved in the same folder!
      this.pendingFile = path;
      this.pendingTargetParquet = getSameFolderParquetPath(path);
      this.isOptimizeModalOpen = true;
    }
  }

  openPendingDirectly() {
    if (this.pendingFile) {
      const p = this.pendingFile;
      this.pendingFile = null;
      this.pendingTargetParquet = '';
      this.isOptimizeModalOpen = false;
      this.openFile(p);
    }
  }

  openOptimizeModal() {
    if (this.metadata) {
      this.pendingFile = this.metadata.file_path;
      this.pendingTargetParquet = getSameFolderParquetPath(this.metadata.file_path);
    }
    this.isOptimizeModalOpen = true;
  }

  closeOptimizeModal() {
    this.isOptimizeModalOpen = false;
    this.pendingFile = null;
    this.pendingTargetParquet = '';
  }

  async pickAndOpenFile(fallbackTrigger?: () => void) {
    if (api.isTauri()) {
      try {
        const path = await api.pickCsvFile();
        if (path) {
          this.handleFileSelection(path);
        }
      } catch (err: any) {
        alert(`Error opening file dialog: ${err?.message || err}`);
      }
    } else if (fallbackTrigger) {
      fallbackTrigger();
    }
  }

  async loadSample() {
    this.isLoading = true;
    this.progress = {
      phase: 'Connecting',
      percentage: 5,
      elapsed_ms: 0,
      estimated_rows: 100000,
      message: 'Generating and opening 100,000 record sample dataset...',
    };

    try {
      const meta = await api.loadSampleDataset();
      this.metadata = meta;
      this.currentPage = 1;
      this.colOffset = 0;
      this.filters = [];
      this.sortRule = null;

      const widths: Record<string, number> = {};
      meta.columns.forEach((col) => {
        widths[col.name] = Math.max(140, Math.min(280, col.name.length * 12 + 40));
      });
      this.columnWidths = widths;

      await this.refreshData();
    } catch (err: any) {
      alert(`Failed to load sample dataset: ${err?.message || err}`);
    } finally {
      this.isLoading = false;
      this.progress = null;
    }
  }

  async openFile(path: string) {
    this.isLoading = true;
    this.progress = {
      phase: 'Connecting',
      percentage: 5,
      elapsed_ms: 0,
      estimated_rows: 0,
      message: 'Opening dataset with DuckDB...',
    };

    try {
      const meta = await api.openCsv(path);
      this.metadata = meta;
      this.currentPage = 1;
      this.colOffset = 0;
      this.filters = [];
      this.sortRule = null;
      
      // Auto-initialize default column widths
      const widths: Record<string, number> = {};
      meta.columns.forEach((col) => {
        widths[col.name] = Math.max(140, Math.min(280, col.name.length * 12 + 40));
      });
      this.columnWidths = widths;

      await this.refreshData();
    } catch (err: any) {
      alert(`Failed to load file: ${err?.message || err}`);
    } finally {
      this.isLoading = false;
      this.progress = null;
    }
  }

  async refreshData(customMessage?: string) {
    if (!this.metadata) return;
    this.isLoading = true;
    const isPaging = !!customMessage && (customMessage.includes('page') || customMessage.includes('Navigating') || customMessage.includes('rows per page'));
    this.progress = {
      phase: isPaging ? 'Paging' : 'Querying',
      percentage: 80,
      elapsed_ms: 0,
      estimated_rows: this.pageResult?.total_filtered_rows ?? this.metadata.total_rows,
      message:
        customMessage ||
        `Fetching page ${this.currentPage} (${this.pageSize} rows / ${this.colLimit} cols)...`,
    };

    const startTime = Date.now();
    try {
      const res = await api.queryPage({
        page: this.currentPage,
        page_size: this.pageSize,
        col_offset: this.colOffset,
        col_limit: this.colLimit,
        filters: this.filters,
        filter_conjunction: this.filterConjunction,
        sort: this.sortRule,
      });
      this.pageResult = res;

      // Provide a brief minimum perceptible display time so the loading modal is clearly visible
      const elapsed = Date.now() - startTime;
      const minDisplayMs = 240;
      if (elapsed < minDisplayMs) {
        await new Promise((r) => setTimeout(r, minDisplayMs - elapsed));
      }
    } catch (err: any) {
      console.error('Data query error:', err);
    } finally {
      this.isLoading = false;
      this.progress = null;
    }
  }

  setPageSize(size: number) {
    const clamped = Math.min(100, Math.max(1, size));
    if (clamped !== this.pageSize) {
      this.pageSize = clamped;
      this.currentPage = 1;
      this.refreshData(`Updating page size to ${clamped} rows per page...`);
    }
  }

  setPage(p: number) {
    if (!this.pageResult) return;
    const target = Math.max(1, Math.min(p, this.pageResult.total_pages));
    if (target !== this.currentPage) {
      this.currentPage = target;
      this.refreshData(`Navigating to page ${target} of ${this.pageResult.total_pages}...`);
    }
  }

  nextPage() {
    if (this.pageResult && this.currentPage < this.pageResult.total_pages) {
      this.setPage(this.currentPage + 1);
    }
  }

  prevPage() {
    if (this.currentPage > 1) {
      this.setPage(this.currentPage - 1);
    }
  }

  setColOffset(offset: number) {
    if (!this.metadata) return;
    const maxOffset = Math.max(0, this.metadata.total_columns - this.colLimit);
    this.colOffset = Math.max(0, Math.min(offset, maxOffset));
    const startCol = this.colOffset + 1;
    const endCol = Math.min(this.colOffset + this.colLimit, this.metadata.total_columns);
    this.refreshData(`Loading columns ${startCol}–${endCol} of ${this.metadata.total_columns}...`);
  }

  nextColBatch() {
    if (!this.metadata) return;
    if (this.colOffset + this.colLimit < this.metadata.total_columns) {
      this.setColOffset(this.colOffset + this.colLimit);
    }
  }

  prevColBatch() {
    if (this.colOffset > 0) {
      this.setColOffset(Math.max(0, this.colOffset - this.colLimit));
    }
  }

  toggleSort(colName: string) {
    if (this.sortRule && this.sortRule.column === colName) {
      if (this.sortRule.ascending) {
        this.sortRule = { column: colName, ascending: false };
      } else {
        this.sortRule = null;
      }
    } else {
      this.sortRule = { column: colName, ascending: true };
    }
    this.currentPage = 1;
    const dir = this.sortRule
      ? this.sortRule.ascending
        ? 'ascending'
        : 'descending'
      : 'default';
    this.refreshData(`Sorting by ${colName} (${dir})...`);
  }

  setColumnWidth(colName: string, width: number) {
    this.columnWidths = {
      ...this.columnWidths,
      [colName]: Math.max(80, Math.min(600, width)),
    };
  }

  // Filter actions
  addFilter() {
    if (!this.metadata || this.metadata.columns.length === 0) return;
    const firstCol = this.metadata.columns[0].name;
    this.filters = [
      ...this.filters,
      {
        id: Math.random().toString(36).substring(2, 9),
        column: firstCol,
        operator: 'contains',
        value: '',
      },
    ];
  }

  removeFilter(id: string) {
    this.filters = this.filters.filter((f) => f.id !== id);
    this.currentPage = 1;
    this.refreshData();
  }

  clearFilters() {
    this.filters = [];
    this.currentPage = 1;
    this.refreshData();
  }

  async cancelOperation() {
    await api.cancelOperation();
    this.isLoading = false;
    this.progress = null;
  }
}

export const appState = new AppStateStore();
