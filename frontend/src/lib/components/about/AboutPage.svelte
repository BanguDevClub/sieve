<script lang="ts">
  import Badge from '$lib/components/common/Badge.svelte';
  import Button from '$lib/components/common/Button.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState } from '$lib/stores/appState.svelte';

  const SHORTCUTS = [
    { key: 'Ctrl + O / ⌘O', action: 'Open CSV File dialog' },
    { key: 'Ctrl + Enter / ⌘Enter', action: 'Run current DuckDB SQL query' },
    { key: 'Esc', action: 'Dismiss modal, popover, or active filter' },
    { key: '← / →', action: 'Navigate previous / next 50 columns' },
  ];

  const TECH_STACK = [
    { name: 'Tauri v2', desc: 'Lightweight, secure cross-platform desktop shell in Rust' },
    { name: 'DuckDB', desc: 'Vectorized columnar out-of-core SQL analytical engine' },
    { name: 'Svelte 5', desc: 'High-performance reactive UI with fine-grained runes' },
    { name: 'TypeScript', desc: 'Strict end-to-end typed contract across IPC' },
  ];
</script>

<div class="sieve-about-page">
  <div class="about-container">
    <!-- Hero Branding Section -->
    <div class="about-hero">
      <div class="about-logo">
        <Icon name="database" size={44} color="var(--accent)" />
      </div>
      <h1 class="about-title">Sieve</h1>
      <p class="about-tagline">
        Engineered by <strong>BanguDevClub</strong>
      </p>
      <div class="about-badges">
        <Badge variant="accent">v0.1.0</Badge>
        <Badge variant="success">MIT License</Badge>
        <Badge variant="info">DuckDB In-Process</Badge>
        <Badge variant="neutral">Tauri 2.0</Badge>
      </div>
    </div>

    <!-- Description Card -->
    <div class="about-card">
      <h3 class="card-title">
        <Icon name="bolt" size={16} color="var(--accent)" />
        High-Performance CSV Processing
      </h3>
      <p class="card-text">
        <strong>Sieve</strong> was developed to eliminate the frustration of loading gigabyte-scale CSV and delimited datasets into traditional spreadsheet applications that freeze, consume tens of gigabytes of RAM, or crash.
      </p>
      <p class="card-text">
        Powered by an embedded DuckDB engine within a multithreaded Rust architecture, Sieve streams queries directly from disk using SIMD-accelerated columnar operations, providing immediate responsiveness with a tiny memory footprint.
      </p>
    </div>

    <!-- Shortcuts & Architecture Grid -->
    <div class="about-grid">
      <!-- Shortcuts -->
      <div class="about-card">
        <h3 class="card-title">
          <Icon name="terminal" size={16} color="var(--accent)" />
          Keyboard Shortcuts
        </h3>
        <div class="shortcuts-list">
          {#each SHORTCUTS as sc}
            <div class="shortcut-row">
              <span class="shortcut-key">{sc.key}</span>
              <span class="shortcut-action">{sc.action}</span>
            </div>
          {/each}
        </div>
      </div>

      <!-- Tech Stack -->
      <div class="about-card">
        <h3 class="card-title">
          <Icon name="columns" size={16} color="var(--accent)" />
          Technology Stack
        </h3>
        <div class="stack-list">
          {#each TECH_STACK as item}
            <div class="stack-item">
              <span class="stack-name">{item.name}</span>
              <span class="stack-desc">{item.desc}</span>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- License Card -->
    <div class="about-card license-card">
      <h3 class="card-title">
        <Icon name="check" size={16} color="var(--success)" />
        MIT License
      </h3>
      <div class="license-box">
        <code>
          Copyright (c) 2026 BanguDevClub<br /><br />
          Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software...
        </code>
      </div>
    </div>

    <div class="about-footer">
      <Button
        variant="secondary"
        size="md"
        icon="table"
        onclick={() => appState.setActiveTab('grid')}
      >
        Back to Data Grid
      </Button>
    </div>
  </div>
</div>

<style>
  .sieve-about-page {
    flex: 1;
    overflow-y: auto;
    padding: 32px 20px;
    display: flex;
    justify-content: center;
    background-color: var(--bg-base);
  }

  .about-container {
    max-width: 720px;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .about-hero {
    text-align: center;
    padding: 24px 0 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .about-logo {
    width: 80px;
    height: 80px;
    border-radius: var(--radius-lg);
    background-color: var(--accent-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 4px;
    box-shadow: var(--shadow-md);
  }

  .about-title {
    font-size: 28px;
    font-weight: 800;
    letter-spacing: -0.5px;
    color: var(--text-main);
  }

  .about-tagline {
    font-size: 14px;
    color: var(--text-muted);
  }

  .about-badges {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 6px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .about-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 20px;
    box-shadow: var(--shadow-sm);
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .card-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-main);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .card-text {
    font-size: 13px;
    color: var(--text-muted);
    line-height: 1.6;
  }

  .about-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  @media (max-width: 600px) {
    .about-grid {
      grid-template-columns: 1fr;
    }
  }

  .shortcuts-list,
  .stack-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
    padding: 4px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .shortcut-key {
    font-family: var(--font-mono);
    font-size: 11px;
    background-color: var(--bg-base);
    border: 1px solid var(--border-subtle);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    color: var(--accent);
  }

  .shortcut-action {
    color: var(--text-muted);
  }

  .stack-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 4px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .stack-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
  }

  .stack-desc {
    font-size: 11px;
    color: var(--text-dim);
  }

  .license-box {
    background-color: var(--bg-base);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 12px;
    font-size: 11px;
    font-family: var(--font-mono);
    color: var(--text-dim);
    line-height: 1.5;
  }

  .about-footer {
    display: flex;
    justify-content: center;
    margin-top: 10px;
  }
</style>
