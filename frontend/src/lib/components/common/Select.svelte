<script lang="ts">
  import Icon from '$lib/icons/Icon.svelte';
  import { onMount } from 'svelte';

  interface OptionItem {
    value: string | number;
    label: string;
    icon?: string;
    description?: string;
  }

  let {
    value = $bindable(''),
    options = [],
    placeholder = 'Select an option...',
    searchable = false,
    disabled = false,
    prefixIcon = '',
    size = 'md',
    direction = 'auto',
    class: className = '',
    onchange,
  }: {
    value?: string | number;
    options?: OptionItem[];
    placeholder?: string;
    searchable?: boolean;
    disabled?: boolean;
    prefixIcon?: string;
    size?: 'sm' | 'md' | 'lg';
    direction?: 'down' | 'up' | 'auto';
    class?: string;
    onchange?: (val: string | number) => void;
  } = $props();

  let computedDirection = $state<'down' | 'up'>('down');

  let isOpen = $state(false);
  let searchQuery = $state('');
  let highlightedIndex = $state(0);
  let containerRef = $state<HTMLDivElement | null>(null);
  let searchInputRef = $state<HTMLInputElement | null>(null);

  let selectedOption = $derived(
    options.find((opt) => opt.value === value)
  );

  let filteredOptions = $derived(
    searchable && searchQuery.trim()
      ? options.filter(
          (opt) =>
            opt.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
            String(opt.value).toLowerCase().includes(searchQuery.toLowerCase())
        )
      : options
  );

  function toggleOpen() {
    if (disabled) return;
    isOpen = !isOpen;
    if (isOpen) {
      if (direction === 'up') {
        computedDirection = 'up';
      } else if (direction === 'down') {
        computedDirection = 'down';
      } else {
        if (containerRef) {
          const rect = containerRef.getBoundingClientRect();
          const spaceBelow = window.innerHeight - rect.bottom;
          computedDirection = spaceBelow < 220 ? 'up' : 'down';
        } else {
          computedDirection = 'down';
        }
      }

      searchQuery = '';
      const idx = options.findIndex((o) => o.value === value);
      highlightedIndex = idx >= 0 ? idx : 0;
      setTimeout(() => searchInputRef?.focus(), 50);
    }
  }

  function selectOption(opt: OptionItem) {
    value = opt.value;
    isOpen = false;
    searchQuery = '';
    onchange?.(opt.value);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (disabled) return;

    if (!isOpen) {
      if (e.key === 'Enter' || e.key === ' ' || e.key === 'ArrowDown') {
        e.preventDefault();
        toggleOpen();
      }
      return;
    }

    if (e.key === 'Escape') {
      isOpen = false;
      return;
    }

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      highlightedIndex = (highlightedIndex + 1) % Math.max(1, filteredOptions.length);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      highlightedIndex =
        (highlightedIndex - 1 + filteredOptions.length) % Math.max(1, filteredOptions.length);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filteredOptions[highlightedIndex]) {
        selectOption(filteredOptions[highlightedIndex]);
      }
    }
  }

  function handleClickOutside(e: MouseEvent) {
    if (containerRef && !containerRef.contains(e.target as Node)) {
      isOpen = false;
    }
  }

  onMount(() => {
    window.addEventListener('click', handleClickOutside);
    return () => window.removeEventListener('click', handleClickOutside);
  });
  const popoverId = `sieve-select-list-${Math.random().toString(36).substring(2, 7)}`;
</script>

<div
  bind:this={containerRef}
  class="sieve-select-container {className} {disabled ? 'disabled' : ''} {isOpen ? 'open' : ''}"
  onkeydown={handleKeyDown}
  role="combobox"
  aria-controls={popoverId}
  aria-expanded={isOpen}
  aria-haspopup="listbox"
  tabindex={disabled ? -1 : 0}
>
  <!-- Trigger Button -->
  <button
    type="button"
    class="sieve-select-trigger size-{size}"
    onclick={toggleOpen}
    {disabled}
  >
    {#if prefixIcon}
      <span class="sieve-select-prefix">
        <Icon name={prefixIcon} size={14} color="var(--text-dim)" />
      </span>
    {/if}

    <span class="sieve-select-label {selectedOption ? '' : 'placeholder'}">
      {#if selectedOption}
        {#if selectedOption.icon}
          <Icon name={selectedOption.icon} size={13} class="opt-icon" />
        {/if}
        {selectedOption.label}
      {:else}
        {placeholder}
      {/if}
    </span>

    <span class="sieve-select-chevron">
      <Icon name={isOpen ? 'chevron-up' : 'chevron-down'} size={13} color="var(--text-dim)" />
    </span>
  </button>

  <!-- Dropdown Popover -->
  {#if isOpen}
    <div class="sieve-select-popover direction-{computedDirection}" role="listbox">
      {#if searchable}
        <div class="sieve-select-search-box">
          <Icon name="search" size={13} color="var(--text-dim)" />
          <input
            bind:this={searchInputRef}
            type="text"
            placeholder="Search..."
            bind:value={searchQuery}
            class="sieve-select-search-input"
            onclick={(e) => e.stopPropagation()}
          />
        </div>
      {/if}

      <div class="sieve-select-options">
        {#if filteredOptions.length === 0}
          <div class="sieve-select-no-results">No options found</div>
        {:else}
          {#each filteredOptions as opt, i}
            {@const isSelected = opt.value === value}
            {@const isHighlighted = i === highlightedIndex}
            <button
              type="button"
              class="sieve-select-option {isSelected ? 'selected' : ''} {isHighlighted ? 'highlighted' : ''}"
              onclick={() => selectOption(opt)}
              role="option"
              aria-selected={isSelected}
            >
              <div class="sieve-select-option-content">
                {#if opt.icon}
                  <Icon name={opt.icon} size={13} class="opt-icon" />
                {/if}
                <span class="sieve-select-option-title">{opt.label}</span>
                {#if opt.description}
                  <span class="sieve-select-option-desc">{opt.description}</span>
                {/if}
              </div>

              {#if isSelected}
                <Icon name="check" size={14} color="var(--accent)" />
              {/if}
            </button>
          {/each}
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .sieve-select-container {
    position: relative;
    width: 100%;
    outline: none;
  }

  .sieve-select-trigger {
    width: 100%;
    height: 36px;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    color: var(--text-main);
    font-family: var(--font-sans);
    font-size: 13px;
    cursor: pointer;
    outline: none;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast), background-color var(--transition-fast);
  }

  .sieve-select-trigger.size-sm {
    height: 28px;
    padding: 0 8px;
    font-size: 12px;
    border-radius: var(--radius-sm);
  }

  .sieve-select-trigger.size-lg {
    height: 42px;
    padding: 0 12px;
  }

  .sieve-select-container:focus-within .sieve-select-trigger,
  .sieve-select-container.open .sieve-select-trigger {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-subtle);
  }

  .sieve-select-container.disabled .sieve-select-trigger {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .sieve-select-prefix {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .sieve-select-label {
    flex: 1;
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .sieve-select-label.placeholder {
    color: var(--text-dim);
  }

  .sieve-select-chevron {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .sieve-select-popover {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    z-index: 100;
    background-color: var(--bg-overlay);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    max-height: 260px;
    animation: fadeIn 120ms ease-out;
  }

  .sieve-select-popover.direction-up {
    top: auto;
    bottom: calc(100% + 4px);
    animation: fadeInBottom 120ms ease-out;
  }

  .sieve-select-search-box {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-bottom: 1px solid var(--border-subtle);
    background-color: var(--bg-surface);
  }

  .sieve-select-search-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    font-family: var(--font-sans);
    font-size: 12px;
    color: var(--text-main);
  }

  .sieve-select-options {
    overflow-y: auto;
    padding: 4px;
    max-height: 200px;
  }

  .sieve-select-no-results {
    padding: 12px;
    text-align: center;
    font-size: 12px;
    color: var(--text-dim);
  }

  .sieve-select-option {
    width: 100%;
    background: transparent;
    border: none;
    padding: 8px 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    color: var(--text-main);
    font-family: var(--font-sans);
    font-size: 13px;
    text-align: left;
    transition: background-color var(--transition-fast);
  }

  .sieve-select-option:hover,
  .sieve-select-option.highlighted {
    background-color: var(--bg-surface-hover);
  }

  .sieve-select-option.selected {
    background-color: var(--accent-subtle);
    font-weight: 500;
  }

  .sieve-select-option-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .sieve-select-option-title {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sieve-select-option-desc {
    font-size: 11px;
    color: var(--text-dim);
  }

  :global(.opt-icon) {
    margin-right: 4px;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes fadeInBottom {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
