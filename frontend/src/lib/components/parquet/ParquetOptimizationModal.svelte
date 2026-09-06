<script lang="ts">
  import Modal from '$lib/components/common/Modal.svelte';
  import Button from '$lib/components/common/Button.svelte';
  import Input from '$lib/components/common/Input.svelte';
  import Select from '$lib/components/common/Select.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState, getSameFolderParquetPath } from '$lib/stores/appState.svelte';
  import * as api from '$lib/api/sieveClient';
  import type { OptimizationResult, ParquetOptimizationOptions } from '$lib/types';

  let {
    isOpen = false,
    onclose,
  }: {
    isOpen: boolean;
    onclose: () => void;
  } = $props();

  let targetPath = $state('');
  let compression = $state<'ZSTD' | 'SNAPPY' | 'GZIP' | 'UNCOMPRESSED'>('ZSTD');
  let rowGroupSize = $state('122880');
  let sortColumn = $state('');
  let autoReload = $state(true);
  let isOptimizing = $state(false);
  let result = $state<OptimizationResult | null>(null);
  let errorMessage = $state('');

  let isPreOpen = $derived(!!appState.pendingFile);
  let currentSourcePath = $derived(appState.pendingFile || appState.metadata?.file_path || '');

  // Pre-fill target path saved in the same folder from the original file
  $effect(() => {
    if (isOpen) {
      if (appState.pendingTargetParquet) {
        targetPath = appState.pendingTargetParquet;
      } else if (appState.metadata?.file_path) {
        targetPath = getSameFolderParquetPath(appState.metadata.file_path);
      }
    } else {
      targetPath = '';
      result = null;
      errorMessage = '';
    }
  });

  const COMPRESSION_OPTIONS = [
    { value: 'ZSTD', label: 'Zstandard (ZSTD) - Recommended for query speed' },
    { value: 'SNAPPY', label: 'Snappy - Fastest write compression' },
    { value: 'GZIP', label: 'GZIP - Highest compression ratio' },
    { value: 'UNCOMPRESSED', label: 'Uncompressed - Raw columnar layout' },
  ];

  const ROW_GROUP_OPTIONS = [
    { value: '65536', label: '65,536 rows (Fine-grained pruning)' },
    { value: '122880', label: '122,880 rows (DuckDB Standard Default)' },
    { value: '250000', label: '250,000 rows (Balanced for Analytical Workloads)' },
    { value: '500000', label: '500,000 rows (High Throughput Scans)' },
  ];

  let columnOptions = $derived([
    { value: '', label: 'None (Preserve current order)' },
    ...(appState.metadata?.columns.map((c) => ({
      value: c.name,
      label: `${c.name} (${c.data_type})`,
    })) || []),
  ]);

  function getFolder(filePath: string): string {
    if (!filePath) return '';
    const lastSlash = Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\'));
    return lastSlash >= 0 ? filePath.substring(0, lastSlash) : '.';
  }

  function getFileName(filePath: string): string {
    if (!filePath) return '';
    const lastSlash = Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\'));
    return lastSlash >= 0 ? filePath.substring(lastSlash + 1) : filePath;
  }

  async function handleBrowse() {
    const chosen = await api.pickSaveParquetFile(targetPath || 'dataset.parquet');
    if (chosen) {
      targetPath = chosen;
    }
  }

  async function handleOptimize() {
    if (!targetPath) {
      errorMessage = 'Please specify a target destination path.';
      return;
    }

    isOptimizing = true;
    errorMessage = '';
    result = null;

    try {
      const source = appState.pendingFile || appState.metadata?.file_path || null;
      const options: ParquetOptimizationOptions = {
        target_path: targetPath,
        source_path: source,
        compression,
        row_group_size: Number(rowGroupSize),
        sort_column: sortColumn ? sortColumn : null,
      };

      const res = await api.optimizeToParquet(options);
      result = res;

      if (isPreOpen || autoReload) {
        // Automatically switch session to the newly generated Parquet file!
        appState.pendingFile = null;
        appState.pendingTargetParquet = '';
        await appState.openFile(res.target_path);
      }
    } catch (err: any) {
      errorMessage = err?.message || String(err);
    } finally {
      isOptimizing = false;
    }
  }

  function handleOpenOriginalDirectly() {
    appState.openPendingDirectly();
  }

  function formatBytes(bytes: number): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }

  function handleModalClose() {
    if (!isOptimizing) {
      result = null;
      errorMessage = '';
      appState.closeOptimizeModal();
      onclose();
    }
  }
</script>

<Modal
  {isOpen}
  title={isPreOpen ? 'Optimize Dataset Before Opening' : 'Parquet Data Structure Optimization'}
  icon="bolt"
  width="580px"
  onclose={handleModalClose}
>
  <div class="sieve-opt-body">
    {#if result}
      <!-- Success State -->
      <div class="sieve-opt-success">
        <div class="success-icon-badge">
          <Icon name="check" size={32} color="var(--success)" />
        </div>
        <h3 class="success-title">Optimization Complete!</h3>
        <p class="success-subtitle">
          Dataset successfully converted to ultra-fast columnar Apache Parquet in the same folder.
        </p>

        <div class="stats-comparison-grid">
          <div class="stat-card">
            <span class="stat-label">Original Size</span>
            <span class="stat-val">{formatBytes(result.original_size_bytes)}</span>
          </div>

          <div class="stat-card highlight">
            <span class="stat-label">Parquet Size</span>
            <span class="stat-val font-mono">{formatBytes(result.parquet_size_bytes)}</span>
          </div>

          <div class="stat-card highlight-green">
            <span class="stat-label">Space Saved</span>
            <span class="stat-val font-mono">
              -{result.space_saved_percent.toFixed(1)}%
            </span>
          </div>

          <div class="stat-card">
            <span class="stat-label">Time Elapsed</span>
            <span class="stat-val font-mono">{result.execution_time_ms} ms</span>
          </div>
        </div>

        <div class="autoreload-pill">
          <Icon name="bolt" size={14} color="var(--accent)" />
          <span>Active dataset switched to high-performance Parquet: {getFileName(result.target_path)}</span>
        </div>
      </div>
    {:else}
      <!-- Source File & Same-Folder Info Card -->
      {#if currentSourcePath}
        <div class="opt-file-card">
          <div class="opt-file-header">
            <div class="opt-file-title">
              <Icon name="file-text" size={15} color="var(--accent)" />
              <span class="opt-file-name">{getFileName(currentSourcePath)}</span>
            </div>
            <Badge variant="accent">
              <Icon name="folder" size={11} />
              Same Folder
            </Badge>
          </div>
          <div class="opt-file-dir" title={getFolder(currentSourcePath)}>
            <span class="opt-dir-label">Directory:</span>
            <span class="opt-dir-path font-mono">{getFolder(currentSourcePath)}</span>
          </div>
        </div>
      {/if}

      <!-- Explanation Hero Banner -->
      <div class="sieve-opt-banner">
        <div class="banner-icon">
          <Icon name="bolt" size={22} color="var(--accent)" />
        </div>
        <div class="banner-text">
          <h4>Faster Queries & Smaller Disk Footprint</h4>
          <p>
            Parquet compresses column values with min/max zone maps and dictionary encoding.
            DuckDB queries run <strong>10x–50x faster</strong> with zero-copy projection pushdown, while saving <strong>70%–90% disk space</strong>.
          </p>
        </div>
      </div>

      <!-- Settings Form -->
      <div class="opt-form">
        <!-- Target File Path in Same Folder -->
        <div class="form-group">
          <div class="form-label-row">
            <label class="form-label" for="target-path">Target Parquet Destination</label>
            <span class="form-label-badge">Saved in same folder</span>
          </div>
          <div class="path-input-row">
            <Input
              id="target-path"
              placeholder="/path/to/dataset.parquet"
              bind:value={targetPath}
              disabled={isOptimizing}
            />
            <Button
              variant="secondary"
              size="md"
              icon="folder-open"
              onclick={handleBrowse}
              disabled={isOptimizing}
            >
              Browse...
            </Button>
          </div>
        </div>

        <!-- Compression Algorithm -->
        <div class="form-group">
          <label class="form-label" for="opt-compression">Compression Algorithm</label>
          <Select
            options={COMPRESSION_OPTIONS}
            value={compression}
            onchange={(v) => (compression = v as any)}
            disabled={isOptimizing}
          />
          <span class="form-hint">
            <strong>ZSTD</strong> delivers the optimal balance of blazingly fast scans and high compression ratio.
          </span>
        </div>

        <div class="form-row">
          <!-- Row Group Size -->
          <div class="form-group flex-1">
            <label class="form-label" for="opt-row-group">Row Group Size</label>
            <Select
              options={ROW_GROUP_OPTIONS}
              value={rowGroupSize}
              onchange={(v) => (rowGroupSize = String(v))}
              disabled={isOptimizing}
            />
          </div>

          <!-- Clustering / Sort Column -->
          {#if !isPreOpen && columnOptions.length > 1}
            <div class="form-group flex-1">
              <label class="form-label" for="opt-sort-col">Cluster by Column (Optional)</label>
              <Select
                options={columnOptions}
                value={sortColumn}
                onchange={(v) => (sortColumn = String(v))}
                disabled={isOptimizing}
              />
            </div>
          {/if}
        </div>

        {#if !isPreOpen}
          <!-- Auto Reload Checkbox -->
          <label class="checkbox-row">
            <input
              type="checkbox"
              bind:checked={autoReload}
              disabled={isOptimizing}
            />
            <span>Automatically switch Sieve session to the optimized Parquet file</span>
          </label>
        {/if}

        {#if errorMessage}
          <div class="opt-error">
            <Icon name="xmark" size={14} color="var(--danger)" />
            <span>{errorMessage}</span>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  {#snippet footer()}
    <div class="opt-footer">
      {#if result}
        <Button
          variant="primary"
          size="md"
          icon="check"
          onclick={handleModalClose}
        >
          View Dataset in Grid
        </Button>
      {:else}
        <Button
          variant="ghost"
          size="md"
          onclick={handleModalClose}
          disabled={isOptimizing}
        >
          Cancel
        </Button>

        {#if isPreOpen}
          <Button
            variant="secondary"
            size="md"
            onclick={handleOpenOriginalDirectly}
            disabled={isOptimizing}
            title="Open original CSV/Excel/JSON file without converting"
          >
            Open Original Directly
          </Button>

          <Button
            variant="primary"
            size="md"
            icon="bolt"
            loading={isOptimizing}
            onclick={handleOptimize}
          >
            {isOptimizing ? 'Optimizing & Opening...' : '⚡ Optimize & Open'}
          </Button>
        {:else}
          <Button
            variant="primary"
            size="md"
            icon="bolt"
            loading={isOptimizing}
            onclick={handleOptimize}
          >
            {isOptimizing ? 'Optimizing Dataset...' : '⚡ Generate Parquet'}
          </Button>
        {/if}
      {/if}
    </div>
  {/snippet}
</Modal>

<style>
  .sieve-opt-body {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .opt-file-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .opt-file-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }

  .opt-file-title {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .opt-file-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .opt-file-dir {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--text-muted);
    min-width: 0;
  }

  .opt-dir-label {
    color: var(--text-dim);
    font-weight: 500;
  }

  .opt-dir-path {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sieve-opt-banner {
    display: flex;
    align-items: flex-start;
    gap: 14px;
    background: linear-gradient(135deg, var(--bg-surface-hover), var(--bg-surface));
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 12px 14px;
  }

  .banner-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
    border-radius: var(--radius-sm);
    background-color: var(--bg-base);
    border: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .banner-text h4 {
    margin: 0 0 3px 0;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-main);
  }

  .banner-text p {
    margin: 0;
    font-size: 12px;
    line-height: 1.5;
    color: var(--text-muted);
  }

  .opt-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .form-label-badge {
    font-size: 10px;
    color: var(--accent);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .flex-1 {
    flex: 1;
  }

  .form-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-main);
  }

  .path-input-row {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .form-hint {
    font-size: 11px;
    color: var(--text-dim);
    line-height: 1.4;
  }

  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-main);
    cursor: pointer;
    user-select: none;
    margin-top: 4px;
  }

  .opt-error {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-surface-hover);
    border: 1px solid var(--danger);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    font-size: 12px;
    color: var(--danger);
  }

  /* Success View */
  .sieve-opt-success {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 10px 0;
    gap: 14px;
  }

  .success-icon-badge {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background-color: var(--bg-surface-hover);
    border: 2px solid var(--success);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .success-title {
    margin: 0;
    font-size: 18px;
    font-weight: 700;
    color: var(--text-main);
  }

  .success-subtitle {
    margin: 0;
    font-size: 12px;
    color: var(--text-muted);
  }

  .stats-comparison-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
    width: 100%;
    margin-top: 6px;
  }

  .stat-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .stat-card.highlight {
    border-color: var(--accent);
  }

  .stat-card.highlight-green {
    border-color: var(--success);
    background: linear-gradient(135deg, var(--bg-surface), var(--bg-surface-hover));
  }

  .stat-label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-dim);
    font-weight: 600;
  }

  .stat-val {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-main);
  }

  .autoreload-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-surface);
    border: 1px solid var(--accent);
    border-radius: var(--radius-full);
    padding: 6px 14px;
    font-size: 12px;
    color: var(--text-main);
    max-width: 100%;
  }

  .autoreload-pill span {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .font-mono {
    font-family: var(--font-mono);
  }

  .opt-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    width: 100%;
  }
</style>
