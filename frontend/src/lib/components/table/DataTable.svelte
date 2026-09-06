<script lang="ts">
  import TableHeader from './TableHeader.svelte';
  import Button from '$lib/components/common/Button.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState } from '$lib/stores/appState.svelte';

  let fileInputRef: HTMLInputElement | null = null;

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const file = input.files[0];
      // In web preview or Tauri, pass path or name
      const path = (file as any).path || file.name;
      appState.handleFileSelection(path);
    }
  }

  function openFileAction() {
    appState.pickAndOpenFile(() => fileInputRef?.click());
  }

  function loadSampleData() {
    appState.loadSample();
  }

  function formatCellValue(
    val: any,
    colType?: string,
    colName?: string
  ): { display: string; isNull: boolean; isNum: boolean; isBool: boolean; isDate: boolean } {
    if (val === null || val === undefined) {
      return { display: 'null', isNull: true, isNum: false, isBool: false, isDate: false };
    }
    if (typeof val === 'boolean') {
      return { display: val ? 'true' : 'false', isNull: false, isNum: false, isBool: true, isDate: false };
    }

    const typeUpper = (colType || '').toUpperCase();
    const nameLower = (colName || '').toLowerCase();
    const isDateCol = typeUpper.includes('DATE') || nameLower.includes('date');
    const isTimeCol =
      typeUpper.includes('TIME') ||
      typeUpper.includes('TIMESTAMP') ||
      nameLower.includes('timestamp');

    if (isDateCol) {
      const num =
        typeof val === 'number'
          ? val
          : typeof val === 'string' && /^-?\d+$/.test(val.trim())
            ? Number(val)
            : NaN;
      if (!isNaN(num)) {
        try {
          // DuckDB Date32 is days since 1970-01-01 (e.g. 19755)
          const ms =
            Math.abs(num) < 200000
              ? num * 86400000
              : Math.abs(num) < 10000000000
                ? num * 1000
                : num;
          const d = new Date(ms);
          if (!isNaN(d.getTime())) {
            return {
              display: d.toISOString().split('T')[0],
              isNull: false,
              isNum: false,
              isBool: false,
              isDate: true,
            };
          }
        } catch {
          // fallback
        }
      }
      return { display: String(val), isNull: false, isNum: false, isBool: false, isDate: true };
    }

    if (isTimeCol) {
      return { display: String(val), isNull: false, isNum: false, isBool: false, isDate: true };
    }

    if (typeof val === 'number') {
      return { display: val.toLocaleString(), isNull: false, isNum: true, isBool: false, isDate: false };
    }
    return { display: String(val), isNull: false, isNum: false, isBool: false, isDate: false };
  }
</script>

<input
  bind:this={fileInputRef}
  type="file"
  accept=".csv,.tsv,.tab,.txt,.parquet,.pq,.xlsx,.xls,.xlsb,.ods,.json,.jsonl,.ndjson"
  style="display: none;"
  onchange={handleFileSelect}
/>

<div class="sieve-table-container">
  {#if !appState.metadata}
    <!-- Empty State / File Uploader -->
    <div class="sieve-empty-state">
      <div class="sieve-empty-card">
        <div class="sieve-empty-icon">
          <Icon name="database" size={48} color="var(--accent)" />
        </div>
        <h2 class="sieve-empty-title">Welcome to Sieve</h2>
        <p class="sieve-empty-subtitle">
          Engineered by <strong>BanguDevClub</strong> for instantaneous querying of massive CSV, Excel, Parquet, and JSON datasets using in-process DuckDB.
        </p>

        <div class="sieve-empty-actions">
          <Button
            variant="primary"
            size="lg"
            icon="folder-open"
            onclick={openFileAction}
          >
            Open Dataset
          </Button>

          <Button
            variant="secondary"
            size="lg"
            icon="database"
            onclick={loadSampleData}
          >
            Load Sample Dataset (100k Rows)
          </Button>
        </div>

        <div class="sieve-empty-features">
          <div class="feature-item">
            <Icon name="bolt" size={14} color="var(--accent)" />
            <span>Out-of-core streaming</span>
          </div>
          <div class="feature-item">
            <Icon name="filter" size={14} color="var(--accent)" />
            <span>Fuzzy & Multi-rule filters</span>
          </div>
          <div class="feature-item">
            <Icon name="terminal" size={14} color="var(--accent)" />
            <span>Custom DuckDB SQL</span>
          </div>
        </div>
      </div>
    </div>
  {:else if appState.pageResult}
    <!-- Active Data Table -->
    <div class="sieve-table-scroll-wrapper">
      <table class="sieve-table">
        <thead>
          <tr>
            <!-- Sticky Row Index Header -->
            <th class="sieve-th-index">#</th>

            <!-- Column Headers -->
            {#each appState.pageResult.displayed_columns as col (col.name)}
              <TableHeader column={col} />
            {/each}
          </tr>
        </thead>
        <tbody>
          {#if appState.pageResult.rows.length === 0}
            <tr>
              <td
                colspan={appState.pageResult.displayed_columns.length + 1}
                class="sieve-no-data"
              >
                <div class="sieve-no-data-content">
                  <Icon name="search" size={24} color="var(--text-dim)" />
                  <p>No records match the current filter criteria.</p>
                  {#if appState.filters.length > 0}
                    <Button
                      variant="secondary"
                      size="sm"
                      onclick={() => appState.clearFilters()}
                    >
                      Clear Filters
                    </Button>
                  {/if}
                </div>
              </td>
            </tr>
          {:else}
            {#each appState.pageResult.rows as row, rIdx}
              {@const globalRowIdx =
                (appState.pageResult.page - 1) * appState.pageResult.page_size + rIdx + 1}
              <tr class="sieve-tr {rIdx % 2 === 1 ? 'zebra' : ''}">
                <!-- Sticky Row Index Column -->
                <td class="sieve-td-index">{globalRowIdx}</td>

                {#each row as cell, cIdx}
                  {@const col = appState.pageResult.displayed_columns[cIdx]}
                  {@const formatted = formatCellValue(cell, col?.data_type, col?.name)}
                  <td
                    class="sieve-td {formatted.isNum ? 'align-right' : ''}"
                    title={String(cell ?? '')}
                  >
                    {#if formatted.isNull}
                      <span class="sieve-null-pill">null</span>
                    {:else if formatted.isBool}
                      <span class="sieve-bool-badge {cell ? 'is-true' : 'is-false'}">
                        {formatted.display}
                      </span>
                    {:else}
                      <span class="cell-text {formatted.isDate ? 'font-mono' : ''}">{formatted.display}</span>
                    {/if}
                  </td>
                {/each}
              </tr>
            {/each}
          {/if}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .sieve-table-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    background-color: var(--bg-base);
  }

  /* Empty State */
  .sieve-empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
  }

  .sieve-empty-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 40px;
    max-width: 540px;
    width: 100%;
    text-align: center;
    box-shadow: var(--shadow-md);
  }

  .sieve-empty-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 80px;
    margin: 0 auto 20px;
    background-color: var(--accent-subtle);
    border-radius: var(--radius-full);
  }

  .sieve-empty-title {
    font-size: 22px;
    font-weight: 700;
    color: var(--text-main);
    margin-bottom: 8px;
  }

  .sieve-empty-subtitle {
    font-size: 14px;
    color: var(--text-muted);
    line-height: 1.6;
    margin-bottom: 24px;
  }

  .sieve-empty-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin-bottom: 28px;
    flex-wrap: wrap;
  }

  .sieve-empty-features {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding-top: 20px;
    border-top: 1px solid var(--border-subtle);
    flex-wrap: wrap;
  }

  .feature-item {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-dim);
  }

  /* Scroll Wrapper */
  .sieve-table-scroll-wrapper {
    flex: 1;
    overflow: auto;
    position: relative;
    background-color: var(--bg-base);
  }

  .sieve-table {
    border-collapse: separate;
    border-spacing: 0;
    width: 100%;
    font-size: 13px;
    table-layout: fixed;
  }

  /* Sticky Header Row Index */
  .sieve-th-index {
    position: sticky;
    top: 0;
    left: 0;
    z-index: 30;
    width: 64px;
    min-width: 64px;
    max-width: 64px;
    height: 40px;
    background-color: var(--bg-surface);
    border-bottom: 2px solid var(--border-strong);
    border-right: 2px solid var(--border-strong);
    color: var(--text-dim);
    font-size: 11px;
    font-family: var(--font-mono);
    text-align: center;
    font-weight: 600;
    padding: 0;
  }

  /* Table Rows */
  .sieve-tr {
    height: 32px;
    transition: background-color var(--transition-fast);
  }

  .sieve-tr:hover {
    background-color: var(--bg-surface-hover) !important;
  }

  .sieve-tr.zebra {
    background-color: var(--bg-surface);
  }

  /* Row Index Cell */
  .sieve-td-index {
    position: sticky;
    left: 0;
    z-index: 5;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    border-right: 2px solid var(--border-strong);
    text-align: center;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
    padding: 0 6px;
    user-select: none;
  }

  .sieve-tr:hover .sieve-td-index {
    background-color: var(--bg-surface-hover);
    color: var(--accent);
  }

  /* Data Cells */
  .sieve-td {
    border-bottom: 1px solid var(--border-subtle);
    border-right: 1px solid var(--border-subtle);
    padding: 0 10px;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    height: 32px;
  }

  .sieve-td.align-right {
    text-align: right;
    font-family: var(--font-mono);
  }

  .cell-text {
    display: inline-block;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sieve-null-pill {
    font-size: 10px;
    font-family: var(--font-mono);
    color: var(--text-dim);
    background-color: var(--border-subtle);
    padding: 1px 5px;
    border-radius: var(--radius-sm);
    font-style: italic;
  }

  .sieve-bool-badge {
    font-size: 11px;
    font-family: var(--font-mono);
    padding: 1px 6px;
    border-radius: var(--radius-sm);
  }

  .sieve-bool-badge.is-true {
    color: var(--success);
    background-color: var(--success-subtle);
  }

  .sieve-bool-badge.is-false {
    color: var(--danger);
    background-color: var(--danger-subtle);
  }

  /* No Data Placeholder */
  .sieve-no-data {
    text-align: center;
    padding: 60px 20px;
  }

  .sieve-no-data-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    color: var(--text-muted);
  }
</style>
