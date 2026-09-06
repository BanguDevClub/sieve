<script lang="ts">
  let {
    name = 'circle',
    size = 16,
    color = 'currentColor',
    spin = false,
    class: className = '',
  }: {
    name: string;
    size?: number | string;
    color?: string;
    spin?: boolean;
    class?: string;
  } = $props();

  let pxSize = $derived(typeof size === 'number' ? `${size}px` : size);
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 24 24"
  width={pxSize}
  height={pxSize}
  fill={color}
  class="sieve-icon {className} {spin ? 'spin' : ''}"
  aria-hidden="true"
>
  {#if name === 'database'}
    <!-- FontAwesome Database -->
    <path
      d="M12 2C6.48 2 2 3.79 2 6v12c0 2.21 4.48 4 10 4s10-1.79 10-4V6c0-2.21-4.48-4-10-4zm0 2c4.42 0 8 1.34 8 2s-3.58 2-8 2-8-1.34-8-2 3.58-2 8-2zm8 6c0 .66-3.58 2-8 2s-8-1.34-8-2V8.16c1.88 1.13 4.77 1.84 8 1.84s6.12-.71 8-1.84V10zm0 6c0 .66-3.58 2-8 2s-8-1.34-8-2v-1.84c1.88 1.13 4.77 1.84 8 1.84s6.12-.71 8-1.84V16z"
    />
  {:else if name === 'table'}
    <!-- FontAwesome Table -->
    <path
      d="M4 3h16a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2zm0 4h16V5H4v2zm0 4h7v-2H4v2zm9 0h7v-2h-7v2zm-9 4h7v-2H4v2zm9 0h7v-2h-7v2zm-9 4h7v-2H4v2zm9 0h7v-2h-7v2z"
    />
  {:else if name === 'filter'}
    <!-- FontAwesome Filter -->
    <path
      d="M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z"
    />
  {:else if name === 'search'}
    <!-- FontAwesome Search -->
    <path
      d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"
    />
  {:else if name === 'play'}
    <path d="M8 5v14l11-7z" />
  {:else if name === 'folder-open'}
    <path
      d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"
    />
  {:else if name === 'terminal'}
    <!-- Nerd Font Terminal Prompt -->
    <path
      d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h16v12zM6 10l4 4-4 4 1.41 1.41L12.83 14l-5.42-5.41L6 10zm6 6h6v2h-6z"
    />
  {:else if name === 'xmark'}
    <path
      d="M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
    />
  {:else if name === 'check'}
    <path d="M9 16.17 4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
  {:else if name === 'plus'}
    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
  {:else if name === 'trash'}
    <path
      d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"
    />
  {:else if name === 'chevron-down'}
    <path d="M7.41 8.59 12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z" />
  {:else if name === 'chevron-up'}
    <path d="M7.41 15.41 12 10.83l4.59 4.58L18 14l-6-6-6 6z" />
  {:else if name === 'chevron-left'}
    <path d="M15.41 7.41 14 6l-6 6 6 6 1.41-1.41L10.83 12z" />
  {:else if name === 'chevron-right'}
    <path d="M10 6 8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" />
  {:else if name === 'chevrons-left'}
    <path d="M18.41 16.59 13.82 12l4.59-4.59L17 6l-6 6 6 6zM6 6h2v12H6z" />
  {:else if name === 'chevrons-right'}
    <path d="M5.59 7.41 10.18 12l-4.59 4.59L7 18l6-6-6-6zM16 6h2v12h-2z" />
  {:else if name === 'file-csv'}
    <path
      d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"
    />
  {:else if name === 'download'}
    <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z" />
  {:else if name === 'copy'}
    <path
      d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
    />
  {:else if name === 'info'}
    <path
      d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"
    />
  {:else if name === 'palette'}
    <path
      d="M12 3c-4.97 0-9 4.03-9 9 0 2.12.74 4.07 1.97 5.61L4.35 19.4c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l2.02-2.02C9.28 19.49 10.59 20 12 20c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"
    />
  {:else if name === 'sun'}
    <path
      d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58a.996.996 0 0 0-1.41 0 .996.996 0 0 0 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.02 0-1.41L5.99 4.58zm12.37 12.37a.996.996 0 0 0-1.41 0 .996.996 0 0 0 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0a.996.996 0 0 0 0-1.41l-1.06-1.06zm1.06-10.96a.996.996 0 0 0 0-1.41.996.996 0 0 0-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.02.39 1.41 0l1.06-1.06zM7.05 18.36a.996.996 0 0 0 0-1.41.996.996 0 0 0-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06z"
    />
  {:else if name === 'moon'}
    <path
      d="M12 3c-4.97 0-9 4.03-9 9 0 2.12.74 4.07 1.97 5.61L4.35 19.4c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l2.02-2.02C9.28 19.49 10.59 20 12 20c4.97 0 9-4.03 9-9s-4.03-9-9-9z"
    />
  {:else if name === 'spinner'}
    <path
      d="M12 4V2A10 10 0 0 0 2 12h2a8 8 0 0 1 8-8z"
    />
  {:else if name === 'columns'}
    <path
      d="M10 18h4V6h-4v12zM4 18h4V6H4v12zm12 0h4V6h-4v12z"
    />
  {:else if name === 'bolt'}
    <path d="M11 21h-1l1-7H7.5c-.58 0-.57-.32-.38-.66.19-.34.05-.08.07-.12C8.48 10.94 10.42 7.54 13 3h1l-1 7h3.5c.49 0 .56.33.47.51l-.07.15C14.9 14.66 12.96 18.06 11 21z" />
  {:else}
    <circle cx="12" cy="12" r="8" />
  {/if}
</svg>

<style>
  .sieve-icon {
    display: inline-block;
    vertical-align: middle;
    flex-shrink: 0;
    transition: transform var(--transition-fast), fill var(--transition-fast);
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
