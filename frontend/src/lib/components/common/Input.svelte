<script lang="ts">
  import Icon from '$lib/icons/Icon.svelte';

  let {
    value = $bindable(''),
    placeholder = '',
    type = 'text',
    prefixIcon = '',
    clearable = false,
    disabled = false,
    error = '',
    size = 'md',
    textAlign = 'left',
    min,
    max,
    id,
    class: className = '',
    oninput,
    onkeydown,
  }: {
    value?: string | number;
    placeholder?: string;
    type?: string;
    prefixIcon?: string;
    clearable?: boolean;
    disabled?: boolean;
    error?: string;
    size?: 'sm' | 'md' | 'lg';
    textAlign?: 'left' | 'center' | 'right';
    min?: number | string;
    max?: number | string;
    id?: string;
    class?: string;
    oninput?: (e: Event) => void;
    onkeydown?: (e: KeyboardEvent) => void;
  } = $props();

  function handleClear() {
    value = '';
  }
</script>

<div
  class="sieve-input-wrapper size-{size} {className} {disabled ? 'disabled' : ''} {error ? 'has-error' : ''}"
>
  {#if prefixIcon}
    <div class="sieve-input-prefix">
      <Icon name={prefixIcon} size={size === 'sm' ? 12 : 14} color="var(--text-dim)" />
    </div>
  {/if}

  <input
    {id}
    {type}
    {placeholder}
    {disabled}
    {min}
    {max}
    bind:value
    {oninput}
    {onkeydown}
    style="text-align: {textAlign};"
    class="sieve-input font-sans"
  />

  {#if clearable && value && !disabled}
    <button
      type="button"
      class="sieve-input-clear"
      onclick={handleClear}
      title="Clear input"
      tabindex="-1"
    >
      <Icon name="xmark" size={size === 'sm' ? 11 : 13} color="var(--text-dim)" />
    </button>
  {/if}

  {#if error}
    <span class="sieve-input-error-msg">{error}</span>
  {/if}
</div>

<style>
  .sieve-input-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast), background-color var(--transition-fast);
    width: 100%;
  }

  .sieve-input-wrapper.size-sm {
    height: 28px;
    padding: 0 6px;
    border-radius: var(--radius-sm);
  }

  .sieve-input-wrapper.size-md {
    height: 36px;
    padding: 0 10px;
  }

  .sieve-input-wrapper.size-lg {
    height: 42px;
    padding: 0 12px;
  }

  .sieve-input-wrapper:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-subtle);
  }

  .sieve-input-wrapper.has-error {
    border-color: var(--danger);
    box-shadow: 0 0 0 2px var(--danger-subtle);
  }

  .sieve-input-wrapper.disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .sieve-input-prefix {
    display: flex;
    align-items: center;
    margin-right: 6px;
    flex-shrink: 0;
  }

  .sieve-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-main);
    font-size: 13px;
    height: 100%;
    width: 100%;
    min-width: 0;
  }

  .size-sm .sieve-input {
    font-size: 12px;
  }

  .sieve-input::placeholder {
    color: var(--text-dim);
  }

  /* Hide native number spinner arrows */
  .sieve-input::-webkit-outer-spin-button,
  .sieve-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  .sieve-input[type='number'] {
    appearance: textfield;
    -moz-appearance: textfield;
  }

  .sieve-input-clear {
    background: transparent;
    border: none;
    padding: 2px;
    margin-left: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    transition: opacity var(--transition-fast);
  }

  .sieve-input-clear:hover {
    opacity: 0.8;
  }

  .sieve-input-error-msg {
    position: absolute;
    bottom: -18px;
    left: 4px;
    font-size: 11px;
    color: var(--danger);
  }
</style>
