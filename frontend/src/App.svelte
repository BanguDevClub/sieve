<script lang="ts">
  import Button from '$lib/components/common/Button.svelte';
  import Select from '$lib/components/common/Select.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import DataTable from '$lib/components/table/DataTable.svelte';
  import PaginationBar from '$lib/components/table/PaginationBar.svelte';
  import FilterBar from '$lib/components/filter/FilterBar.svelte';
  import SqlConsole from '$lib/components/sql/SqlConsole.svelte';
  import AboutPage from '$lib/components/about/AboutPage.svelte';
  import ProgressModal from '$lib/components/progress/ProgressModal.svelte';
  import ParquetOptimizationModal from '$lib/components/parquet/ParquetOptimizationModal.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import type { Theme } from '$lib/types';
  import * as api from '$lib/api/sieveClient';

  let fileInputRef: HTMLInputElement | null = null;

  const THEME_OPTIONS = [
    { value: 'dark', label: 'Dark', icon: 'moon' },
    { value: 'light', label: 'Light', icon: 'sun' },
    { value: 'catppuccin-latte', label: 'Catppuccin Latte', icon: 'palette' },
    { value: 'catppuccin-frappe', label: 'Catppuccin Frappé', icon: 'palette' },
    { value: 'catppuccin-macchiato', label: 'Catppuccin Macchiato', icon: 'palette' },
    { value: 'catppuccin-mocha', label: 'Catppuccin Mocha', icon: 'palette' },
  ];

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const file = input.files[0];
      const path = (file as any).path || file.name;
      appState.handleFileSelection(path);
    }
  }

  function openFileAction() {
    appState.pickAndOpenFile(() => fileInputRef?.click());
  }

  // Listen for native drag & drop from OS
  if (typeof window !== 'undefined' && api.isTauri()) {
    import('@tauri-apps/api/event').then(({ listen }) => {
      listen<{ paths: string[] }>('tauri://drag-drop', (event) => {
        if (event.payload.paths && event.payload.paths.length > 0) {
          appState.handleFileSelection(event.payload.paths[0]);
        }
      });
    });
  }

  async function handleExport(format: 'csv' | 'parquet') {
    if (!appState.metadata) return;
    try {
      const target = `sieve_export_${Date.now()}.${format}`;
      const msg = await api.exportData(target, format);
      alert(msg);
    } catch (e: any) {
      alert(`Export failed: ${e?.message || e}`);
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

<input
  bind:this={fileInputRef}
  type="file"
  accept=".csv,.tsv,.tab,.txt,.parquet,.pq,.xlsx,.xls,.xlsb,.ods,.json,.jsonl,.ndjson"
  style="display: none;"
  onchange={handleFileSelect}
/>

<div class="sieve-app">
  <!-- Top Navigation Header -->
  <header class="sieve-topbar">
    <!-- Branding -->
    <div
      class="topbar-branding"
      onclick={() => appState.setActiveTab('grid')}
      onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && appState.setActiveTab('grid')}
      role="button"
      tabindex="0"
    >
      <div class="topbar-logo">
        <Icon name="database" size={18} color="var(--accent)" />
      </div>
      <div class="topbar-brand-text">
        <span class="brand-title">Sieve</span>
        <span class="brand-org">by BanguDevClub</span>
      </div>
    </div>

    <!-- Active File Badge / Open Button -->
    <div class="topbar-file-section">
      <Button
        variant="secondary"
        size="sm"
        icon="folder-open"
        onclick={openFileAction}
      >
        Open Dataset
      </Button>

      {#if appState.metadata}
        <div class="topbar-file-info" title={appState.metadata.file_path}>
          <span class="file-name">{appState.metadata.file_name}</span>
          {#if appState.metadata.file_format}
            <Badge variant={appState.metadata.is_optimized ? 'accent' : 'neutral'}>
              {appState.metadata.file_format.toUpperCase()}
            </Badge>
          {/if}
          <Badge variant="neutral">{formatBytes(appState.metadata.file_size_bytes)}</Badge>
          <Badge variant="accent">{appState.metadata.total_columns} cols</Badge>
          <Badge variant="info">{appState.metadata.total_rows.toLocaleString()} rows</Badge>

          {#if !appState.metadata.is_optimized}
            <Button
              variant="primary"
              size="sm"
              icon="bolt"
              onclick={() => appState.openOptimizeModal()}
              title="Optimize dataset into high-speed columnar Parquet"
            >
              Optimize to Parquet
            </Button>
          {:else}
            <Badge variant="accent">
              <Icon name="bolt" size={11} />
              Optimized
            </Badge>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Center: Navigation Tabs -->
    <nav class="topbar-nav">
      <button
        type="button"
        class="nav-tab {appState.activeTab === 'grid' ? 'active' : ''}"
        onclick={() => appState.setActiveTab('grid')}
      >
        <Icon name="table" size={14} />
        <span>Data Grid</span>
      </button>

      <button
        type="button"
        class="nav-tab {appState.activeTab === 'sql' ? 'active' : ''}"
        onclick={() => appState.setActiveTab('sql')}
      >
        <Icon name="terminal" size={14} />
        <span>DuckDB SQL</span>
      </button>

      <button
        type="button"
        class="nav-tab {appState.activeTab === 'about' ? 'active' : ''}"
        onclick={() => appState.setActiveTab('about')}
      >
        <Icon name="info" size={14} />
        <span>About</span>
      </button>
    </nav>

    <!-- Right: Export & Theme Selector -->
    <div class="topbar-right">
      {#if appState.metadata}
        <Button
          variant="ghost"
          size="sm"
          icon="download"
          onclick={() => handleExport('csv')}
          title="Export query as CSV"
        >
          CSV
        </Button>
        <Button
          variant="ghost"
          size="sm"
          icon="download"
          onclick={() => handleExport('parquet')}
          title="Export query as Parquet"
        >
          Parquet
        </Button>
      {/if}

      <div class="theme-picker-wrapper">
        <Select
          options={THEME_OPTIONS}
          bind:value={appState.theme}
          onchange={(t) => appState.setTheme(t as Theme)}
        />
      </div>
    </div>
  </header>

  <!-- Main View Area -->
  <main class="sieve-main">
    {#if appState.activeTab === 'grid'}
      {#if appState.metadata}
        <FilterBar />
      {/if}
      <DataTable />
      {#if appState.metadata}
        <PaginationBar />
      {/if}
    {:else if appState.activeTab === 'sql'}
      <SqlConsole />
    {:else if appState.activeTab === 'about'}
      <AboutPage />
    {/if}
  </main>

  <!-- Estimate & Progress Modal -->
  <ProgressModal />

  <!-- Parquet Data Structure Optimization Modal -->
  <ParquetOptimizationModal
    isOpen={appState.isOptimizeModalOpen}
    onclose={() => appState.closeOptimizeModal()}
  />
</div>

<style>
  .sieve-app {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-base);
  }

  .sieve-topbar {
    height: 48px;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    gap: 16px;
    flex-shrink: 0;
    z-index: 50;
  }

  .topbar-branding {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    user-select: none;
    outline: none;
  }

  .topbar-logo {
    width: 30px;
    height: 30px;
    border-radius: var(--radius-md);
    background-color: var(--accent-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .topbar-brand-text {
    display: flex;
    flex-direction: column;
  }

  .brand-title {
    font-size: 14px;
    font-weight: 700;
    line-height: 1.1;
    color: var(--text-main);
  }

  .brand-org {
    font-size: 10px;
    color: var(--text-dim);
  }

  .topbar-file-section {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .topbar-file-info {
    display: flex;
    align-items: center;
    gap: 6px;
    max-width: 360px;
    min-width: 0;
  }

  .file-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 140px;
  }

  .topbar-nav {
    display: flex;
    align-items: center;
    background-color: var(--bg-base);
    padding: 3px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
  }

  .nav-tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    font-size: 12px;
    font-weight: 500;
    color: var(--text-muted);
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .nav-tab:hover {
    color: var(--text-main);
  }

  .nav-tab.active {
    background-color: var(--bg-surface);
    color: var(--accent);
    box-shadow: var(--shadow-sm);
  }

  .topbar-right {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .theme-picker-wrapper {
    width: 180px;
  }

  .sieve-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }
</style>
