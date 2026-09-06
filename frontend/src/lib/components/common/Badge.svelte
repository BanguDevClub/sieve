<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    variant = 'neutral',
    class: className = '',
    children,
  }: {
    variant?: 'neutral' | 'accent' | 'success' | 'warning' | 'danger' | 'info';
    class?: string;
    children?: Snippet;
  } = $props();
</script>

<span class="sieve-badge variant-{variant} {className}">
  {#if children}
    {@render children()}
  {/if}
</span>

<style>
  .sieve-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 500;
    line-height: 1;
    white-space: nowrap;
  }

  .variant-neutral {
    background-color: var(--bg-surface);
    color: var(--text-muted);
    border: 1px solid var(--border-subtle);
  }

  .variant-accent {
    background-color: var(--accent-subtle);
    color: var(--accent);
    border: 1px solid var(--accent);
  }

  .variant-success {
    background-color: var(--success-subtle);
    color: var(--success);
    border: 1px solid var(--success);
  }

  .variant-warning {
    background-color: var(--warning-subtle);
    color: var(--warning);
    border: 1px solid var(--warning);
  }

  .variant-danger {
    background-color: var(--danger-subtle);
    color: var(--danger);
    border: 1px solid var(--danger);
  }

  .variant-info {
    background-color: rgba(6, 182, 212, 0.12);
    color: var(--info);
    border: 1px solid var(--info);
  }
</style>
