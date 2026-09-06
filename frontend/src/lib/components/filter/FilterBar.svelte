<script lang="ts">
  import Button from '$lib/components/common/Button.svelte';
  import Select from '$lib/components/common/Select.svelte';
  import Input from '$lib/components/common/Input.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import type { FilterOperator } from '$lib/types';

  const OPERATOR_OPTIONS = [
    { value: 'contains', label: 'Contains (Fuzzy)', icon: 'search' },
    { value: 'equals', label: 'Exact Match (=)', icon: 'check' },
    { value: 'starts_with', label: 'Starts With', icon: 'chevron-right' },
    { value: 'ends_with', label: 'Ends With', icon: 'chevron-left' },
    { value: 'in', label: 'In List (a, b, c)', icon: 'table' },
    { value: 'gt', label: 'Greater Than (>)', icon: 'chevron-right' },
    { value: 'lt', label: 'Less Than (<)', icon: 'chevron-left' },
    { value: 'gte', label: 'Greater/Equal (>=)', icon: 'check' },
    { value: 'lte', label: 'Less/Equal (<=)', icon: 'check' },
    { value: 'is_empty', label: 'Is Empty / Null', icon: 'xmark' },
    { value: 'is_not_empty', label: 'Is Not Empty', icon: 'check' },
  ];

  let columnOptions = $derived(
    appState.metadata?.columns.map((c) => ({
      value: c.name,
      label: c.name,
      description: c.data_type,
    })) || []
  );

  function toggleConjunction() {
    appState.filterConjunction = appState.filterConjunction === 'AND' ? 'OR' : 'AND';
    if (appState.filters.length > 1) {
      appState.currentPage = 1;
      appState.refreshData();
    }
  }

  function handleFilterSubmit() {
    appState.currentPage = 1;
    appState.refreshData();
  }
</script>

<div class="sieve-filter-bar">
  <div class="sieve-filter-header">
    <div class="filter-header-left">
      <Icon name="filter" size={15} color="var(--accent)" />
      <span class="filter-title">Advanced Filters</span>
      {#if appState.filters.length > 0}
        <Badge variant="accent">{appState.filters.length} active</Badge>
      {/if}
    </div>

    <div class="filter-header-right">
      {#if appState.filters.length > 1}
        <button
          type="button"
          class="conjunction-toggle {appState.filterConjunction.toLowerCase()}"
          onclick={toggleConjunction}
          title="Toggle condition joining logic"
        >
          Match: <strong>{appState.filterConjunction}</strong> conditions
        </button>
      {/if}

      <Button
        variant="ghost"
        size="sm"
        icon="plus"
        onclick={() => appState.addFilter()}
      >
        Add Filter
      </Button>

      {#if appState.filters.length > 0}
        <Button
          variant="danger"
          size="sm"
          icon="trash"
          onclick={() => appState.clearFilters()}
        >
          Clear
        </Button>
      {/if}
    </div>
  </div>

  {#if appState.filters.length > 0}
    <div class="sieve-filter-rules">
      {#each appState.filters as filter, idx (filter.id)}
        <div class="sieve-filter-row">
          {#if idx > 0}
            <span class="rule-connector">{appState.filterConjunction}</span>
          {:else}
            <span class="rule-connector initial">WHERE</span>
          {/if}

          <!-- Column Select -->
          <div class="filter-field col-picker">
            <Select
              options={columnOptions}
              bind:value={filter.column}
              searchable={true}
              placeholder="Select Column..."
              onchange={() => handleFilterSubmit()}
            />
          </div>

          <!-- Operator Select -->
          <div class="filter-field op-picker">
            <Select
              options={OPERATOR_OPTIONS}
              bind:value={filter.operator}
              placeholder="Operator..."
              onchange={() => handleFilterSubmit()}
            />
          </div>

          <!-- Value Input -->
          <div class="filter-field val-input">
            <Input
              bind:value={filter.value}
              placeholder={filter.operator === 'in' ? 'Val1, Val2, Val3...' : 'Filter value...'}
              disabled={filter.operator === 'is_empty' || filter.operator === 'is_not_empty'}
              clearable={true}
              onkeydown={(e) => e.key === 'Enter' && handleFilterSubmit()}
            />
          </div>

          <!-- Delete Row -->
          <Button
            variant="ghost"
            size="sm"
            icon="xmark"
            title="Remove this condition"
            onclick={() => appState.removeFilter(filter.id)}
          />
        </div>
      {/each}

      <div class="sieve-filter-actions-row">
        <Button
          variant="primary"
          size="sm"
          icon="play"
          onclick={handleFilterSubmit}
        >
          Apply Filters
        </Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .sieve-filter-bar {
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    padding: 10px 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    flex-shrink: 0;
  }

  .sieve-filter-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .filter-header-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .filter-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-main);
  }

  .filter-header-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .conjunction-toggle {
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    color: var(--text-muted);
    font-size: 11px;
    padding: 3px 8px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background-color var(--transition-fast), border-color var(--transition-fast);
  }

  .conjunction-toggle strong {
    color: var(--accent);
  }

  .conjunction-toggle:hover {
    border-color: var(--accent);
    background-color: var(--bg-surface-hover);
  }

  .sieve-filter-rules {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-top: 4px;
  }

  .sieve-filter-row {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-base);
    padding: 6px 10px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
  }

  .rule-connector {
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 700;
    color: var(--accent);
    width: 44px;
    text-align: center;
    flex-shrink: 0;
  }

  .rule-connector.initial {
    color: var(--text-dim);
  }

  .filter-field.col-picker {
    width: 220px;
    flex-shrink: 0;
  }

  .filter-field.op-picker {
    width: 180px;
    flex-shrink: 0;
  }

  .filter-field.val-input {
    flex: 1;
    min-width: 160px;
  }

  .sieve-filter-actions-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 2px;
  }
</style>
