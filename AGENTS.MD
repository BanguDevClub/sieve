# AGENTS.MD: Architecture, Best Practices & Optimization Guidelines

> **Project**: Sieve  
> **Organization**: BanguDevClub  
> **License**: MIT  
> **Stack**: Rust, Tauri v2, DuckDB, Svelte 5, TypeScript, Vite

---

## 1. Project Goal & Core Philosophy

**Sieve** is an ultra-fast, cross-platform desktop application designed to inspect, search, filter, and run analytical SQL against multi-gigabyte or multi-million-row CSV files with instant responsiveness and minimal memory consumption.

Traditional desktop spreadsheet tools attempt to load entire datasets into RAM, leading to memory exhaustion, UI freezing, and crashes. Sieve solves this through **out-of-core streaming** powered by DuckDB's columnar vectorized execution engine, a multithreaded Rust backend, and a reactive, virtualized Svelte 5 frontend.

### Primary Directives:
1. **Never Block the UI Thread**: Long-running operations (file scans, aggregations, exports) must run asynchronously in background thread pools with streaming progress reports.
2. **Predictable Memory Footprint**: Even for a 50GB CSV file, host RAM usage should remain under ~100MB by streaming paginated windows (max 100 rows per page, 50 columns per window).
3. **No Placeholders or Emojis**: Use clean Nerd Fonts and Font Awesome SVG glyphs for consistent, modern, and professional iconography across all operating systems.
4. **Theme Consistency**: Every UI element (inputs, selects, buttons, scrollbars, modals) must strictly adhere to the 6-theme design system.

---

## 2. Rust & DuckDB Best Practices

### 2.1 DuckDB Out-of-Core Execution
- **Virtual View Mapping**: Never ingest whole CSV files into in-memory tables. Use DuckDB's `read_csv_auto` inside a virtual view:
  ```sql
  CREATE OR REPLACE VIEW sieve AS SELECT * FROM read_csv_auto('/path/to/data.csv', union_by_name=True);
  ```
- **Zero-Full-Scan Metadata**: Use `PRAGMA table_info('sieve')` or `DESCRIBE sieve` to immediately extract column headers and inferred data types without waiting for a full file scan.
- **Estimated Row Counts**: For large files, initial row counts can be sampled or calculated via background threads using `SELECT COUNT(*) FROM sieve`, emitting progress events to the frontend.
- **Query Cancellation**: Store DuckDB `Connection` cancellation tokens or atomic interruption flags (`Arc<AtomicBool>`) so user cancellation takes effect immediately.

### 2.2 Multithreading & Tauri Runtime
- **Thread Pool Offloading**: Tauri command handlers must NEVER run CPU-intensive or blocking disk I/O on the main thread. Always wrap DuckDB queries in `tokio::task::spawn_blocking`:
  ```rust
  #[tauri::command]
  pub async fn run_sql(
      state: tauri::State<'_, AppState>,
      query: String,
  ) -> Result<SqlResult, String> {
      tokio::task::spawn_blocking(move || {
          state.engine.lock().unwrap().execute_sql(&query)
      })
      .await
      .map_err(|e| e.to_string())?
  }
  ```
- **State Synchronization**: Guard the DuckDB engine with an `Arc<Mutex<DuckEngine>>` or dedicated worker channels to avoid race conditions.
- **Error Handling**: Use explicit typed errors using `thiserror` or descriptive strings that convey actionable error messages to the frontend.

### 2.3 Serialization & IPC Efficiency
- Limit IPC payload sizes. The frontend strictly requires at most **100 rows** and **50 columns** per page. Do not serialize millions of records over the Tauri IPC bridge.
- Format row values into compact JSON arrays or string representations on the Rust side to minimize serialization overhead.

---

## 3. TypeScript & Svelte 5 Best Practices

### 3.1 Svelte 5 Runes & Reactivity
- Use modern Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`) instead of deprecated Svelte 3/4 stores and legacy syntax:
  ```svelte
  <script lang="ts">
    let { value = $bindable(), options = [], onChange }: Props = $props();
    let isOpen = $state(false);
    let selectedLabel = $derived(
      options.find(o => o.value === value)?.label ?? 'Select...'
    );
  </script>
  ```
- Keep components small, focused, and single-purpose. Break complex views into subcomponents (e.g. `TableHeader`, `TableRow`, `PaginationBar`, `FilterRow`).

### 3.2 Design System & Theming Tokens
- **Theme Variables**: All colors, borders, shadows, and fonts are controlled via CSS custom properties in `app.css`.
- The 6 supported themes:
  1. `light`
  2. `dark`
  3. `catppuccin-latte`
  4. `catppuccin-frappe`
  5. `catppuccin-macchiato`
  6. `catppuccin-mocha`
- **Custom Components**:
  - Do not use default unstyled browser `<select>` or native input widgets.
  - Use `Select.svelte` for accessible, searchable, themed dropdowns.
  - Use `Input.svelte` for inputs with icon adornments, validation states, and clear buttons.

### 3.3 DOM Virtualization & Layout
- **Sticky / Frozen Header**: Use `position: sticky; top: 0; z-index: 10` on table headers with solid background tokens (`var(--bg-surface)`) to prevent bleed-through while scrolling.
- **Resizable Columns**: Keep column width state in the table store. Use mouse event listeners (`pointerdown`, `pointermove`, `pointerup`) with `requestAnimationFrame` for fluid 60fps column dragging.
- **Pagination Boundary**: Cap rows at 100 per page and display at most 50 columns horizontally to prevent DOM node bloat.

---

## 4. Cross-Platform Build & Packaging

### 4.1 Docker Local Build Pipeline
- Base image: `rust:latest` (Debian-based).
- Target architectures:
  - **Linux GNU**: `x86_64-unknown-linux-gnu` -> standalone binary, `.deb`, `.rpm`, `.AppImage`.
  - **Linux MUSL**: `x86_64-unknown-linux-musl` -> statically linked standalone binary.
  - **Windows**: `x86_64-pc-windows-gnu` / `cargo-xwin` -> `.exe` binary and NSIS installer (`sieve-setup.exe`).
  - **macOS**: Bundle layout and preparation scripts in `dist-build/macos/`.
- All commands execute inside the container when invoking `docker compose up`.

### 4.2 GitHub Actions Native Matrix
- Pure native runners without Docker:
  - `ubuntu-latest` for Linux GNU and MUSL.
  - `windows-latest` for Windows MSVC standalone & installers.
  - `macos-13` (AMD64) and `macos-14` (Apple Silicon ARM64) for native Darwin `.app` and `.dmg`.

---

## 5. Security & Maintenance Checklist

- [x] Tauri permissions strictly restricted via `tauri.conf.json` capabilities.
- [x] Input sanitization for user-generated SQL: queries parameterized where possible; raw SQL console sandboxed to read-only views unless export is explicitly initiated.
- [x] No plaintext sensitive logging in production builds.
- [x] Dependency auditing via `cargo audit` and `npm audit`.
