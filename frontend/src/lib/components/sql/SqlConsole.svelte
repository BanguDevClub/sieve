<script lang="ts">
  import Button from '$lib/components/common/Button.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import * as api from '$lib/api/sieveClient';
  import type { SqlResult } from '$lib/types';

  let sqlQuery = $state('SELECT * FROM sieve LIMIT 20;');
  let isExecuting = $state(false);
  let sqlResult = $state<SqlResult | null>(null);
  let errorMessage = $state('');

  const PRESETS = [
    { label: 'Preview 20', query: 'SELECT * FROM sieve LIMIT 20;' },
    { label: 'Count Rows', query: 'SELECT COUNT(*) AS total_rows FROM sieve;' },
    { label: 'Summarize Schema', query: 'SUMMARIZE sieve;' },
    {
      label: 'Sample Group By',
      query:
        'SELECT department, COUNT(*) as headcount, ROUND(AVG(salary), 2) as avg_salary\nFROM sieve\nGROUP BY department\nORDER BY headcount DESC;',
    },
  ];

  async function executeQuery() {
    if (!sqlQuery.trim()) return;
    isExecuting = true;
    errorMessage = '';

    try {
      const res = await api.runCustomSql(sqlQuery);
      sqlResult = res;
    } catch (err: any) {
      errorMessage = err?.message || String(err);
      sqlResult = null;
    } finally {
      isExecuting = false;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      executeQuery();
    }
  }

  function applyPreset(query: string) {
    sqlQuery = query;
    executeQuery();
  }
</script>

<div class="sieve-sql-console">
  <!-- Query Editor Section -->
  <div class="sql-editor-container">
    <div class="sql-editor-toolbar">
      <div class="toolbar-left">
        <Icon name="terminal" size={15} color="var(--accent)" />
        <span class="toolbar-title">DuckDB SQL Console</span>
        <span class="toolbar-shortcut">Ctrl+Enter to Run</span>
      </div>

      <div class="toolbar-presets">
        <span class="presets-label">Presets:</span>
        {#each PRESETS as preset}
          <button
            type="button"
            class="preset-btn"
            onclick={() => applyPreset(preset.query)}
          >
            {preset.label}
          </button>
        {/each}
      </div>

      <div class="toolbar-actions">
        <Button
          variant="primary"
          size="sm"
          icon="play"
          loading={isExecuting}
          onclick={executeQuery}
        >
          Run SQL
        </Button>
      </div>
    </div>

    <div class="sql-textarea-wrapper">
      <textarea
        bind:value={sqlQuery}
        onkeydown={handleKeyDown}
        placeholder="Enter DuckDB SQL query... (e.g. SELECT * FROM sieve LIMIT 50)"
        class="sql-textarea"
        spellcheck="false"
      ></textarea>
    </div>

    {#if errorMessage}
      <div class="sql-error-box">
        <Icon name="xmark" size={14} color="var(--danger)" />
        <span class="sql-error-text">{errorMessage}</span>
      </div>
    {/if}
  </div>

  <!-- Results Section -->
  <div class="sql-results-container">
    <div class="sql-results-header">
      <div class="results-header-left">
        <span class="results-title">Query Results</span>
        {#if sqlResult}
          <Badge variant="accent">
            <Icon name="table" size={12} />
            {sqlResult.total_rows} rows
          </Badge>
          <Badge variant="neutral">
            <Icon name="bolt" size={12} color="var(--info)" />
            {sqlResult.execution_time_ms} ms
          </Badge>
        {/if}
      </div>
    </div>

    <div class="sql-results-table-wrapper">
      {#if !sqlResult}
        <div class="sql-results-empty">
          <Icon name="terminal" size={32} color="var(--text-dim)" />
          <p>Run a DuckDB SQL script above to view query execution results.</p>
        </div>
      {:else if sqlResult.rows.length === 0}
        <div class="sql-results-empty">
          <Icon name="check" size={24} color="var(--success)" />
          <p>Query executed successfully. 0 rows returned.</p>
        </div>
      {:else}
        <table class="sql-table">
          <thead>
            <tr>
              <th class="sql-th-idx">#</th>
              {#each sqlResult.columns as col}
                <th class="sql-th">{col.name}</th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each sqlResult.rows as row, rIdx}
              <tr class="sql-tr {rIdx % 2 === 1 ? 'zebra' : ''}">
                <td class="sql-td-idx">{rIdx + 1}</td>
                {#each row as cell}
                  <td class="sql-td">
                    {#if cell === null || cell === undefined}
                      <span class="sieve-null-pill">null</span>
                    {:else}
                      {String(cell)}
                    {/if}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
</div>

<style>
  .sieve-sql-console {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background-color: var(--bg-base);
  }

  /* Editor Container */
  .sql-editor-container {
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
  }

  .sql-editor-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-subtle);
    gap: 12px;
    flex-wrap: wrap;
  }

  .toolbar-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toolbar-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-main);
  }

  .toolbar-shortcut {
    font-size: 11px;
    color: var(--text-dim);
    font-family: var(--font-mono);
    background-color: var(--bg-base);
    padding: 1px 6px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-subtle);
  }

  .toolbar-presets {
    display: flex;
    align-items: center;
    gap: 6px;
    overflow-x: auto;
  }

  .presets-label {
    font-size: 11px;
    color: var(--text-dim);
  }

  .preset-btn {
    background-color: var(--bg-base);
    border: 1px solid var(--border-subtle);
    color: var(--text-muted);
    font-size: 11px;
    padding: 3px 8px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background-color var(--transition-fast), border-color var(--transition-fast);
  }

  .preset-btn:hover {
    background-color: var(--bg-surface-hover);
    border-color: var(--accent);
    color: var(--text-main);
  }

  .sql-textarea-wrapper {
    padding: 12px 16px;
  }

  .sql-textarea {
    width: 100%;
    height: 110px;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 10px 12px;
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.5;
    color: var(--text-main);
    resize: vertical;
    outline: none;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  }

  .sql-textarea:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-subtle);
  }

  .sql-error-box {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 0 16px 12px;
    padding: 8px 12px;
    background-color: var(--danger-subtle);
    border: 1px solid var(--danger);
    border-radius: var(--radius-md);
    color: var(--danger);
    font-size: 12px;
    font-family: var(--font-mono);
  }

  /* Results Container */
  .sql-results-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sql-results-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
  }

  .results-header-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .results-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
  }

  .sql-results-table-wrapper {
    flex: 1;
    overflow: auto;
  }

  .sql-results-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 200px;
    gap: 12px;
    color: var(--text-dim);
    font-size: 13px;
  }

  .sql-table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    font-size: 12px;
  }

  .sql-th-idx {
    position: sticky;
    top: 0;
    left: 0;
    z-index: 20;
    background-color: var(--bg-surface);
    border-bottom: 2px solid var(--border-strong);
    border-right: 2px solid var(--border-strong);
    padding: 8px;
    width: 48px;
    text-align: center;
    font-family: var(--font-mono);
    color: var(--text-dim);
  }

  .sql-th {
    position: sticky;
    top: 0;
    z-index: 10;
    background-color: var(--bg-surface);
    border-bottom: 2px solid var(--border-strong);
    border-right: 1px solid var(--border-subtle);
    padding: 8px 10px;
    text-align: left;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
    white-space: nowrap;
  }

  .sql-tr.zebra {
    background-color: var(--bg-surface);
  }

  .sql-tr:hover {
    background-color: var(--bg-surface-hover) !important;
  }

  .sql-td-idx {
    position: sticky;
    left: 0;
    z-index: 5;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    border-right: 2px solid var(--border-strong);
    text-align: center;
    font-family: var(--font-mono);
    color: var(--text-dim);
    padding: 6px;
  }

  .sql-td {
    border-bottom: 1px solid var(--border-subtle);
    border-right: 1px solid var(--border-subtle);
    padding: 6px 10px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sieve-null-pill {
    font-size: 10px;
    font-family: var(--font-mono);
    color: var(--text-dim);
    background-color: var(--border-subtle);
    padding: 1px 4px;
    border-radius: var(--radius-sm);
    font-style: italic;
  }
</style>
