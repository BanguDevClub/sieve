<script lang="ts">
  import Modal from '$lib/components/common/Modal.svelte';
  import Button from '$lib/components/common/Button.svelte';
  import Badge from '$lib/components/common/Badge.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import { appState } from '$lib/stores/appState.svelte';

  let progress = $derived(appState.progress);
  let pct = $derived(Math.min(100, Math.max(0, progress?.percentage ?? 65)));

  let elapsedLive = $state(0);
  let timerId: any = null;

  $effect(() => {
    if (appState.isLoading) {
      const initial = progress?.elapsed_ms || 0;
      const start = Date.now() - initial;
      timerId = setInterval(() => {
        elapsedLive = Date.now() - start;
      }, 40);
    } else {
      if (timerId) clearInterval(timerId);
      timerId = null;
      elapsedLive = 0;
    }

    return () => {
      if (timerId) clearInterval(timerId);
    };
  });

  let displayElapsed = $derived(
    progress?.elapsed_ms && progress.elapsed_ms > elapsedLive
      ? progress.elapsed_ms
      : elapsedLive
  );

  let phaseTitle = $derived.by(() => {
    if (!progress) return 'Processing...';
    if (progress.phase === 'Paging') return 'Navigating Data';
    if (progress.phase === 'Scanning' || progress.phase === 'Connecting') return 'Loading CSV File';
    if (progress.phase === 'Querying') return 'Executing DuckDB Query';
    return progress.phase;
  });

  let operationType = $derived.by(() => {
    if (!progress) return 'DuckDB';
    if (progress.phase === 'Paging') return 'Page Jump';
    if (progress.message?.includes('rows per page') || progress.message?.includes('Row Batch')) return 'Row Batch';
    if (progress.message?.includes('columns')) return 'Column Window';
    if (progress.phase === 'Scanning') return 'CSV Ingestion';
    return 'Vectorized Query';
  });
</script>

<Modal
  isOpen={appState.isLoading && !!progress}
  title={phaseTitle}
  icon="database"
  closable={false}
  width="480px"
>
  <div class="sieve-progress-body">
    <!-- Visual Radial Progress -->
    <div class="progress-radial-container">
      <div class="radial-ambient-glow"></div>
      <div class="progress-radial">
        <svg class="radial-svg" viewBox="0 0 100 100">
          <defs>
            <linearGradient id="sieve-spin-grad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="var(--accent)" />
              <stop offset="100%" stop-color="var(--accent-hover)" />
            </linearGradient>
            <filter id="glow-effect" x="-20%" y="-20%" width="140%" height="140%">
              <feGaussianBlur stdDeviation="3" result="blur" />
              <feComposite in="SourceGraphic" in2="blur" operator="over" />
            </filter>
          </defs>

          <!-- Outer background track -->
          <circle class="radial-bg" cx="50" cy="50" r="40" />

          <!-- Animated indeterminate rotating spinner ring -->
          <circle class="radial-spin" cx="50" cy="50" r="40" />

          <!-- Value ring -->
          <circle
            class="radial-fg"
            cx="50"
            cy="50"
            r="40"
            style="stroke-dashoffset: {251.2 - (251.2 * pct) / 100};"
          />
        </svg>

        <div class="radial-center">
          <div class="center-icon-pulse">
            <Icon name="bolt" size={26} color="var(--accent)" />
          </div>
          <span class="radial-pct">{pct}%</span>
        </div>
      </div>
    </div>

    <!-- Status Message & Linear Shimmer Bar -->
    <div class="progress-details">
      <div class="progress-msg-wrapper">
        <p class="progress-msg">{progress?.message || 'Executing vectorized DuckDB query...'}</p>
        <span class="progress-submsg">In-Process Vectorized Engine</span>
      </div>

      <!-- Linear Shimmer Progress Bar -->
      <div class="linear-bar-track">
        <div
          class="linear-bar-fill"
          style="width: {pct}%;"
        >
          <div class="linear-shimmer"></div>
        </div>
      </div>

      <!-- Real-time Metadata Cards -->
      <div class="progress-meta-grid">
        <div class="meta-card">
          <span class="meta-label">Operation</span>
          <Badge variant="accent">{operationType}</Badge>
        </div>

        <div class="meta-card">
          <span class="meta-label">Elapsed</span>
          <span class="meta-val font-mono">{displayElapsed} ms</span>
        </div>

        {#if appState.pageResult}
          <div class="meta-card">
            <span class="meta-label">Active Page</span>
            <span class="meta-val font-mono">
              {appState.currentPage} / {appState.pageResult.total_pages.toLocaleString()}
            </span>
          </div>

          <div class="meta-card">
            <span class="meta-label">Batch Limit</span>
            <span class="meta-val font-mono">{appState.pageSize} rows</span>
          </div>
        {:else if appState.metadata}
          <div class="meta-card">
            <span class="meta-label">Total Rows</span>
            <span class="meta-val font-mono">
              {appState.metadata.total_rows.toLocaleString()}
            </span>
          </div>
        {/if}
      </div>
    </div>
  </div>

  {#snippet footer()}
    <div class="progress-footer-row">
      <span class="footer-hint">Sieve multi-threaded query execution</span>
      <Button
        variant="danger"
        size="sm"
        icon="xmark"
        onclick={() => appState.cancelOperation()}
      >
        Cancel Query
      </Button>
    </div>
  {/snippet}
</Modal>

<style>
  .sieve-progress-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 20px;
    padding: 10px 4px 6px 4px;
  }

  .progress-radial-container {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 124px;
    height: 124px;
  }

  .radial-ambient-glow {
    position: absolute;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: var(--accent);
    opacity: 0.15;
    filter: blur(20px);
    pointer-events: none;
    animation: pulse-glow 2s ease-in-out infinite alternate;
  }

  .progress-radial {
    position: relative;
    width: 116px;
    height: 116px;
  }

  .radial-svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  .radial-bg {
    fill: none;
    stroke: var(--bg-surface-hover);
    stroke-width: 6;
  }

  .radial-spin {
    fill: none;
    stroke: var(--border-focus);
    stroke-width: 2;
    stroke-dasharray: 40 180;
    animation: spin-cw 2s linear infinite;
    transform-origin: center;
    opacity: 0.6;
  }

  .radial-fg {
    fill: none;
    stroke: url(#sieve-spin-grad);
    stroke-width: 7;
    stroke-linecap: round;
    stroke-dasharray: 251.2;
    transition: stroke-dashoffset 220ms ease-out;
  }

  .radial-center {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 3px;
  }

  .center-icon-pulse {
    display: flex;
    align-items: center;
    justify-content: center;
    animation: pulse-icon 1.8s ease-in-out infinite;
  }

  .radial-pct {
    font-size: 14px;
    font-weight: 700;
    font-family: var(--font-mono);
    color: var(--text-main);
    letter-spacing: -0.5px;
  }

  .progress-details {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .progress-msg-wrapper {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .progress-msg {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-main);
    margin: 0;
    line-height: 1.4;
  }

  .progress-submsg {
    font-size: 11px;
    font-weight: 500;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.6px;
  }

  /* Linear Progress Bar */
  .linear-bar-track {
    width: 100%;
    height: 6px;
    background-color: var(--bg-surface-hover);
    border-radius: var(--radius-full);
    overflow: hidden;
    position: relative;
  }

  .linear-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent), var(--accent-hover));
    border-radius: var(--radius-full);
    position: relative;
    transition: width 200ms ease-out;
    overflow: hidden;
  }

  .linear-shimmer {
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.4),
      transparent
    );
    animation: shimmer 1.5s infinite;
  }

  .progress-meta-grid {
    display: flex;
    justify-content: center;
    gap: 10px;
    flex-wrap: wrap;
    width: 100%;
  }

  .meta-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    min-width: 96px;
    flex: 1 1 96px;
    max-width: 130px;
    box-shadow: var(--shadow-sm);
  }

  .meta-label {
    font-size: 10px;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-weight: 600;
  }

  .meta-val {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-main);
    white-space: nowrap;
  }

  .font-mono {
    font-family: var(--font-mono);
  }

  .progress-footer-row {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .footer-hint {
    font-size: 11px;
    color: var(--text-dim);
  }

  @keyframes spin-cw {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes pulse-icon {
    0%, 100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.15);
      opacity: 0.8;
    }
  }

  @keyframes pulse-glow {
    from {
      opacity: 0.12;
      transform: scale(0.9);
    }
    to {
      opacity: 0.28;
      transform: scale(1.15);
    }
  }

  @keyframes shimmer {
    100% {
      left: 100%;
    }
  }
</style>
