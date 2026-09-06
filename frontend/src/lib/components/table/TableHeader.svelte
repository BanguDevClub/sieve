<script lang="ts">
  import Icon from '$lib/icons/Icon.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import type { ColumnInfo } from '$lib/types';

  let { column }: { column: ColumnInfo } = $props();

  let isResizing = $state(false);
  let startX = 0;
  let startWidth = 0;

  function onResizeStart(e: PointerEvent) {
    e.preventDefault();
    e.stopPropagation();
    isResizing = true;
    startX = e.clientX;
    startWidth = appState.columnWidths[column.name] || 160;

    const target = e.currentTarget as HTMLElement;
    target.setPointerCapture(e.pointerId);
  }

  function onResizeMove(e: PointerEvent) {
    if (!isResizing) return;
    const delta = e.clientX - startX;
    appState.setColumnWidth(column.name, startWidth + delta);
  }

  function onResizeEnd(e: PointerEvent) {
    if (!isResizing) return;
    isResizing = false;
    const target = e.currentTarget as HTMLElement;
    try {
      target.releasePointerCapture(e.pointerId);
    } catch (_) {}
  }

  let isSorted = $derived(appState.sortRule?.column === column.name);
  let isAsc = $derived(appState.sortRule?.ascending ?? true);
</script>

<th
  class="sieve-th"
  style="width: {appState.columnWidths[column.name] || 160}px; min-width: {appState.columnWidths[column.name] || 160}px;"
>
  <div
    class="sieve-th-content"
    onclick={() => appState.toggleSort(column.name)}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Enter' && appState.toggleSort(column.name)}
  >
    <div class="sieve-th-title-group">
      <span class="sieve-th-name" title={column.name}>{column.name}</span>
      <Badge variant="neutral" class="col-type-badge">{column.data_type}</Badge>
    </div>

    <div class="sieve-th-sort-icon">
      {#if isSorted}
        <Icon
          name={isAsc ? 'chevron-up' : 'chevron-down'}
          size={12}
          color="var(--accent)"
        />
      {:else}
        <Icon name="arrow-up-down" size={10} color="var(--text-dim)" class="sort-idle" />
      {/if}
    </div>
  </div>

  <!-- Draggable Resize Handle -->
  <div
    class="sieve-th-resize-handle {isResizing ? 'active' : ''}"
    onpointerdown={onResizeStart}
    onpointermove={onResizeMove}
    onpointerup={onResizeEnd}
    onpointercancel={onResizeEnd}
    role="separator"
    aria-orientation="vertical"
    title="Drag to resize column"
  ></div>
</th>

<style>
  .sieve-th {
    position: sticky;
    top: 0;
    z-index: 10;
    background-color: var(--bg-surface);
    border-bottom: 2px solid var(--border-strong);
    border-right: 1px solid var(--border-subtle);
    padding: 0;
    text-align: left;
    user-select: none;
    height: 40px;
    box-sizing: border-box;
  }

  .sieve-th-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding: 0 10px;
    cursor: pointer;
    outline: none;
    transition: background-color var(--transition-fast);
  }

  .sieve-th-content:hover {
    background-color: var(--bg-surface-hover);
  }

  .sieve-th-title-group {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
    overflow: hidden;
  }

  .sieve-th-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global(.col-type-badge) {
    font-size: 9px !important;
    padding: 1px 4px !important;
    opacity: 0.8;
  }

  .sieve-th-sort-icon {
    display: flex;
    align-items: center;
    margin-left: 4px;
    flex-shrink: 0;
  }

  :global(.sort-idle) {
    opacity: 0.3;
  }

  .sieve-th-resize-handle {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    width: 6px;
    cursor: col-resize;
    z-index: 20;
    transition: background-color var(--transition-fast);
  }

  .sieve-th-resize-handle:hover,
  .sieve-th-resize-handle.active {
    background-color: var(--accent);
  }
</style>
