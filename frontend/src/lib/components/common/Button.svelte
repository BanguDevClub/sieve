<script lang="ts">
  import Icon from '$lib/icons/Icon.svelte';
  import type { Snippet } from 'svelte';

  let {
    variant = 'secondary',
    size = 'md',
    icon = '',
    iconRight = '',
    loading = false,
    disabled = false,
    type = 'button',
    title = '',
    class: className = '',
    onclick,
    children,
  }: {
    variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    icon?: string;
    iconRight?: string;
    loading?: boolean;
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
    title?: string;
    class?: string;
    onclick?: (e: MouseEvent) => void;
    children?: Snippet;
  } = $props();
</script>

<button
  {type}
  {title}
  disabled={disabled || loading}
  {onclick}
  class="sieve-btn variant-{variant} size-{size} {className} {loading ? 'loading' : ''}"
>
  {#if loading}
    <Icon name="spinner" size={size === 'sm' ? 12 : 14} spin={true} />
  {:else if icon}
    <Icon name={icon} size={size === 'sm' ? 12 : 14} />
  {/if}

  {#if children}
    <span class="sieve-btn-label">
      {@render children()}
    </span>
  {/if}

  {#if iconRight && !loading}
    <Icon name={iconRight} size={size === 'sm' ? 12 : 14} />
  {/if}
</button>

<style>
  .sieve-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-family: var(--font-sans);
    font-weight: 500;
    border-radius: var(--radius-md);
    border: 1px solid transparent;
    cursor: pointer;
    white-space: nowrap;
    outline: none;
    transition: background-color var(--transition-fast), border-color var(--transition-fast), transform 50ms, box-shadow var(--transition-fast);
    user-select: none;
  }

  .sieve-btn:active:not(:disabled) {
    transform: translateY(1px);
  }

  .sieve-btn:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  /* Sizes */
  .size-sm {
    height: 28px;
    padding: 0 8px;
    font-size: 12px;
  }

  .size-md {
    height: 36px;
    padding: 0 14px;
    font-size: 13px;
  }

  .size-lg {
    height: 42px;
    padding: 0 18px;
    font-size: 14px;
  }

  /* Variants */
  .variant-primary {
    background-color: var(--accent);
    color: var(--accent-contrast);
  }

  .variant-primary:hover:not(:disabled) {
    background-color: var(--accent-hover);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .variant-secondary {
    background-color: var(--bg-surface);
    border-color: var(--border-subtle);
    color: var(--text-main);
  }

  .variant-secondary:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
    border-color: var(--border-strong);
  }

  .variant-danger {
    background-color: var(--danger-subtle);
    border-color: var(--danger);
    color: var(--danger);
  }

  .variant-danger:hover:not(:disabled) {
    background-color: var(--danger);
    color: #ffffff;
  }

  .variant-ghost {
    background: transparent;
    color: var(--text-muted);
  }

  .variant-ghost:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
    color: var(--text-main);
  }
</style>
