<script lang="ts">
  import Button from '$lib/components/common/Button.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import Input from '$lib/components/common/Input.svelte';
  import Select from '$lib/components/common/Select.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState } from '$lib/stores/appState.svelte';

  let jumpPage = $state<number | string>(1);

  $effect(() => {
    jumpPage = appState.currentPage;
  });

  const ROW_OPTIONS = [
    { value: '25', label: '25 rows' },
    { value: '50', label: '50 rows' },
    { value: '100', label: '100 rows' },
  ];

  function handleJumpSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (jumpPage) {
      appState.setPage(Number(jumpPage));
    }
  }

  let colStart = $derived(appState.colOffset + 1);
  let colEnd = $derived(
    Math.min(
      appState.colOffset + appState.colLimit,
      appState.metadata?.total_columns || 0
    )
  );
  let totalCols = $derived(appState.metadata?.total_columns || 0);
  let hasMoreCols = $derived(colEnd < totalCols);
  let hasPrevCols = $derived(appState.colOffset > 0);
</script>

<div class="sieve-pagination-bar">
  <!-- Left: Row Stats & Query Execution Time -->
  <div class="sieve-pagination-left">
    {#if appState.pageResult}
      <div class="sieve-stats-group">
        <Badge variant="accent">
          <Icon name="table" size={12} />
          {appState.pageResult.total_filtered_rows.toLocaleString()} rows
        </Badge>

        <Badge variant="neutral">
          <Icon name="bolt" size={12} color="var(--info)" />
          {appState.pageResult.execution_time_ms} ms
        </Badge>

        <span class="sieve-limit-hint">
          (Max 100 rows / 50 cols)
        </span>
      </div>
    {/if}
  </div>

  <!-- Center: Column Window Pagination (Max 50 columns) -->
  <div class="sieve-pagination-center">
    {#if totalCols > 0}
      <div class="sieve-col-nav">
        <Button
          variant="ghost"
          size="sm"
          icon="chevron-left"
          disabled={!hasPrevCols}
          onclick={() => appState.prevColBatch()}
          title="Previous 50 Columns"
        >
          Cols
        </Button>

        <span class="sieve-col-range-badge">
          Cols {colStart}–{colEnd} of {totalCols}
        </span>

        <Button
          variant="ghost"
          size="sm"
          iconRight="chevron-right"
          disabled={!hasMoreCols}
          onclick={() => appState.nextColBatch()}
          title="Next 50 Columns"
        >
          Cols
        </Button>
      </div>
    {/if}
  </div>

  <!-- Right: Row Pagination (100 rows limit) -->
  <div class="sieve-pagination-right">
    {#if appState.pageResult && appState.pageResult.total_pages > 0}
      <div class="sieve-page-nav">
        <Button
          variant="ghost"
          size="sm"
          icon="chevrons-left"
          disabled={appState.currentPage <= 1}
          onclick={() => appState.setPage(1)}
          title="First Page"
        />

        <Button
          variant="ghost"
          size="sm"
          icon="chevron-left"
          disabled={appState.currentPage <= 1}
          onclick={() => appState.prevPage()}
          title="Previous Page"
        />

        <!-- Custom Rows-per-page Selector -->
        <div class="sieve-page-size-selector">
          <Select
            size="sm"
            direction="up"
            options={ROW_OPTIONS}
            value={String(appState.pageSize)}
            onchange={(val) => appState.setPageSize(Number(val))}
          />
        </div>

        <form class="sieve-page-jumper" onsubmit={handleJumpSubmit}>
          <span class="jumper-label">Page</span>
          <div class="jumper-input-wrapper">
            <Input
              type="number"
              size="sm"
              textAlign="center"
              min={1}
              max={appState.pageResult.total_pages}
              bind:value={jumpPage}
              onkeydown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  if (jumpPage) appState.setPage(Number(jumpPage));
                }
              }}
            />
          </div>
          <span class="jumper-total">of {appState.pageResult.total_pages.toLocaleString()}</span>
        </form>

        <Button
          variant="ghost"
          size="sm"
          icon="chevron-right"
          disabled={appState.currentPage >= appState.pageResult.total_pages}
          onclick={() => appState.nextPage()}
          title="Next Page"
        />

        <Button
          variant="ghost"
          size="sm"
          icon="chevrons-right"
          disabled={appState.currentPage >= appState.pageResult.total_pages}
          onclick={() => appState.pageResult && appState.setPage(appState.pageResult.total_pages)}
          title="Last Page"
        />
      </div>
    {/if}
  </div>
</div>

<style>
  .sieve-pagination-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 44px;
    padding: 0 16px;
    background-color: var(--bg-surface);
    border-top: 1px solid var(--border-subtle);
    font-size: 12px;
    user-select: none;
    flex-shrink: 0;
    gap: 12px;
    position: relative;
    z-index: 25;
  }

  .sieve-pagination-left,
  .sieve-pagination-right {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 220px;
  }

  .sieve-pagination-right {
    justify-content: flex-end;
  }

  .sieve-stats-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .sieve-limit-hint {
    font-size: 11px;
    color: var(--text-dim);
  }

  .sieve-pagination-center {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .sieve-col-nav {
    display: flex;
    align-items: center;
    gap: 6px;
    background-color: var(--bg-base);
    padding: 2px 6px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
  }

  .sieve-col-range-badge {
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 500;
    color: var(--text-muted);
    padding: 0 4px;
  }

  .sieve-page-nav {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .sieve-page-jumper {
    display: flex;
    align-items: center;
    gap: 6px;
    margin: 0 4px;
  }

  .jumper-label,
  .jumper-total {
    color: var(--text-muted);
    font-size: 12px;
  }

  .sieve-page-size-selector {
    width: 106px;
  }

  .jumper-input-wrapper {
    width: 56px;
  }
</style>
