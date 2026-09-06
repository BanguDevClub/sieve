<script lang="ts">
  import Icon from '$lib/icons/Icon.svelte';
  import type { Snippet } from 'svelte';

  let {
    isOpen = false,
    title = '',
    icon = '',
    closable = true,
    width = '520px',
    onclose,
    children,
    footer,
  }: {
    isOpen?: boolean;
    title?: string;
    icon?: string;
    closable?: boolean;
    width?: string;
    onclose?: () => void;
    children?: Snippet;
    footer?: Snippet;
  } = $props();

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && closable) {
      onclose?.();
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape' && closable && isOpen) {
      onclose?.();
    }
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if isOpen}
  <div
    class="sieve-modal-backdrop"
    onclick={handleBackdropClick}
    role="presentation"
  >
    <div class="sieve-modal-container" style="max-width: {width};">
      <!-- Modal Header -->
      <div class="sieve-modal-header">
        <div class="sieve-modal-title-group">
          {#if icon}
            <Icon name={icon} size={16} color="var(--accent)" />
          {/if}
          <h3 class="sieve-modal-title">{title}</h3>
        </div>

        {#if closable}
          <button
            type="button"
            class="sieve-modal-close"
            onclick={onclose}
            title="Close modal"
          >
            <Icon name="xmark" size={14} color="var(--text-dim)" />
          </button>
        {/if}
      </div>

      <!-- Modal Body -->
      <div class="sieve-modal-body">
        {#if children}
          {@render children()}
        {/if}
      </div>

      <!-- Modal Footer -->
      {#if footer}
        <div class="sieve-modal-footer">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .sieve-modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 1000;
    background-color: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    animation: fadeIn 150ms ease-out;
  }

  .sieve-modal-container {
    width: 100%;
    background-color: var(--bg-overlay);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: popIn 180ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  .sieve-modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .sieve-modal-title-group {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .sieve-modal-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-main);
  }

  .sieve-modal-close {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--transition-fast);
  }

  .sieve-modal-close:hover {
    background-color: var(--bg-surface-hover);
  }

  .sieve-modal-body {
    padding: 20px;
    overflow-y: auto;
    max-height: 70vh;
  }

  .sieve-modal-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 10px;
    padding: 12px 20px;
    border-top: 1px solid var(--border-subtle);
    background-color: var(--bg-surface);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: scale(0.96) translateY(8px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
</style>
