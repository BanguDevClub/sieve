use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, State};
use crate::duckdb_engine::{
    CsvMetadata, OptimizationResult, PageRequest, PageResult, ParquetOptimizationOptions, SqlResult,
};
use crate::state::AppState;

#[tauri::command]
pub async fn pick_csv_file() -> Result<Option<String>, String> {
    #[cfg(feature = "gui")]
    {
        let file = rfd::AsyncFileDialog::new()
            .set_title("Open Dataset - Sieve")
            .add_filter(
                "All Supported Datasets (*.csv, *.parquet, *.xlsx, *.json, etc.)",
                &[
                    "csv", "tsv", "tab", "txt", "parquet", "pq", "xlsx", "xls", "xlsb", "ods",
                    "json", "jsonl", "ndjson",
                ],
            )
            .add_filter("Apache Parquet (*.parquet, *.pq)", &["parquet", "pq"])
            .add_filter(
                "Excel Workbooks (*.xlsx, *.xls, *.xlsb, *.ods)",
                &["xlsx", "xls", "xlsb", "ods"],
            )
            .add_filter(
                "CSV & Delimited Files (*.csv, *.tsv, *.txt)",
                &["csv", "tsv", "tab", "txt"],
            )
            .add_filter(
                "JSON Datasets (*.json, *.jsonl, *.ndjson)",
                &["json", "jsonl", "ndjson"],
            )
            .add_filter("All Files (*.*)", &["*"])
            .pick_file()
            .await;

        Ok(file.map(|f| f.path().to_string_lossy().to_string()))
    }
    #[cfg(not(feature = "gui"))]
    {
        Ok(None)
    }
}

async fn open_csv_internal(
    app: AppHandle,
    state: State<'_, AppState>,
    file_path: String,
) -> Result<CsvMetadata, String> {
    state.reset_cancel();
    let engine = state.engine.clone();
    let cancel_token = state.cancel_token.clone();

    tokio::task::spawn_blocking(move || {
        let mut eng = engine
            .lock()
            .map_err(|e| format!("Lock acquisition failed: {}", e))?;

        eng.open_csv(&file_path, cancel_token, |update| {
            let _ = app.emit("sieve://progress", &update);
        })
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

#[tauri::command]
pub async fn load_sample_dataset(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CsvMetadata, String> {
    let sample_path = std::env::temp_dir().join("sieve_sample_100k.csv");
    let sample_str = sample_path.to_string_lossy().to_string();

    if !sample_path.exists() {
        let temp_conn = duckdb::Connection::open_in_memory()
            .map_err(|e| format!("Failed to initialize DuckDB in-memory engine: {}", e))?;

        let path_clean = sample_str.replace('\\', "/");
        let gen_query = format!(
            "COPY (
                SELECT 
                    i AS id,
                    'Employee ' || i AS full_name,
                    'user' || i || '@bangudev.club' AS email,
                    CASE (i % 6)
                        WHEN 0 THEN 'Engineering'
                        WHEN 1 THEN 'Data Science'
                        WHEN 2 THEN 'Operations'
                        WHEN 3 THEN 'Finance'
                        WHEN 4 THEN 'Marketing'
                        ELSE 'Product'
                    END AS department,
                    CASE (i % 4)
                        WHEN 0 THEN 'Senior'
                        WHEN 1 THEN 'Lead'
                        WHEN 2 THEN 'Staff'
                        ELSE 'Principal'
                    END AS seniority,
                    ROUND(50000 + (i * 137.5) % 120000, 2) AS salary,
                    CASE (i % 5)
                        WHEN 0 THEN 'Brazil'
                        WHEN 1 THEN 'USA'
                        WHEN 2 THEN 'Germany'
                        WHEN 3 THEN 'Japan'
                        ELSE 'UK'
                    END AS country,
                    CASE (i % 6)
                        WHEN 0 THEN 'Rio de Janeiro'
                        WHEN 1 THEN 'San Francisco'
                        WHEN 2 THEN 'Berlin'
                        WHEN 3 THEN 'Tokyo'
                        WHEN 4 THEN 'London'
                        ELSE 'Sao Paulo'
                    END AS city,
                    (i % 5 != 0) AS is_active,
                    ROUND(((i * 3.1415) % 100) / 10.0, 2) AS score,
                    '2024-' || LPAD(((i % 12) + 1)::VARCHAR, 2, '0') || '-' || LPAD(((i % 28) + 1)::VARCHAR, 2, '0') AS hire_date,
                    'Sample benchmark record #' || i AS notes
                FROM generate_series(1, 100000) t(i)
            ) TO '{}' (HEADER, DELIMITER ',');",
            path_clean
        );
        temp_conn
            .execute_batch(&gen_query)
            .map_err(|e| format!("Failed to generate sample dataset: {}", e))?;
    }

    open_csv_internal(app, state, sample_str).await
}

#[tauri::command]
pub async fn open_csv_file(
    app: AppHandle,
    state: State<'_, AppState>,
    file_path: String,
) -> Result<CsvMetadata, String> {
    if file_path == "__sample__" || file_path == "/demo/large_dataset_100k.csv" {
        return load_sample_dataset(app, state).await;
    }

    // Smart path resolution: if relative or not found in CWD, check common folders
    let resolved_path = if Path::new(&file_path).exists() {
        file_path
    } else if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        let home_path = PathBuf::from(&home);
        let in_downloads = home_path.join("Downloads").join(&file_path);
        let in_documents = home_path.join("Documents").join(&file_path);
        let in_home = home_path.join(&file_path);

        if in_downloads.exists() {
            in_downloads.to_string_lossy().to_string()
        } else if in_documents.exists() {
            in_documents.to_string_lossy().to_string()
        } else if in_home.exists() {
            in_home.to_string_lossy().to_string()
        } else {
            file_path
        }
    } else {
        file_path
    };

    open_csv_internal(app, state, resolved_path).await
}

#[tauri::command]
pub async fn query_page(
    state: State<'_, AppState>,
    req: PageRequest,
) -> Result<PageResult, String> {
    state.reset_cancel();
    let engine = state.engine.clone();
    let cancel_token = state.cancel_token.clone();

    tokio::task::spawn_blocking(move || {
        let eng = engine
            .lock()
            .map_err(|e| format!("Lock acquisition failed: {}", e))?;
        eng.query_page(req, cancel_token)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

#[tauri::command]
pub async fn run_custom_sql(
    state: State<'_, AppState>,
    query: String,
    max_rows: Option<usize>,
) -> Result<SqlResult, String> {
    state.reset_cancel();
    let engine = state.engine.clone();
    let cancel_token = state.cancel_token.clone();

    tokio::task::spawn_blocking(move || {
        let eng = engine
            .lock()
            .map_err(|e| format!("Lock acquisition failed: {}", e))?;
        eng.execute_sql(&query, max_rows, cancel_token)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

#[tauri::command]
pub async fn export_data(
    state: State<'_, AppState>,
    target_path: String,
    format: String,
    custom_sql: Option<String>,
) -> Result<String, String> {
    let engine = state.engine.clone();

    tokio::task::spawn_blocking(move || {
        let eng = engine
            .lock()
            .map_err(|e| format!("Lock acquisition failed: {}", e))?;
        eng.export_data(&target_path, &format, custom_sql)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

#[tauri::command]
pub fn cancel_current_operation(state: State<'_, AppState>) -> Result<(), String> {
    state.trigger_cancel();
    Ok(())
}

#[tauri::command]
pub fn get_metadata(state: State<'_, AppState>) -> Result<Option<CsvMetadata>, String> {
    let eng = state
        .engine
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    Ok(eng.get_metadata())
}

#[tauri::command]
pub async fn optimize_to_parquet(
    app: AppHandle,
    state: State<'_, AppState>,
    options: ParquetOptimizationOptions,
) -> Result<OptimizationResult, String> {
    state.reset_cancel();
    let engine = state.engine.clone();
    let cancel_token = state.cancel_token.clone();

    let _ = app.emit(
        "sieve://progress",
        &crate::duckdb_engine::ProgressUpdate {
            phase: "Optimizing".to_string(),
            percentage: 25,
            elapsed_ms: 0,
            estimated_rows: 0,
            message: format!(
                "Compressing dataset into columnar Parquet ({}, row group {})...",
                options.compression, options.row_group_size
            ),
        },
    );

    let res = tokio::task::spawn_blocking(move || {
        let mut eng = engine
            .lock()
            .map_err(|e| format!("Lock acquisition failed: {}", e))?;
        eng.optimize_to_parquet(&options, cancel_token)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))??;

    let _ = app.emit(
        "sieve://progress",
        &crate::duckdb_engine::ProgressUpdate {
            phase: "Complete".to_string(),
            percentage: 100,
            elapsed_ms: res.execution_time_ms,
            estimated_rows: res.total_rows,
            message: format!(
                "Parquet generated: {:.1}% space saved in {} ms!",
                res.space_saved_percent, res.execution_time_ms
            ),
        },
    );

    Ok(res)
}

#[tauri::command]
pub async fn pick_save_parquet_file(default_name: Option<String>) -> Result<Option<String>, String> {
    #[cfg(feature = "gui")]
    {
        let name = default_name.unwrap_or_else(|| "optimized_dataset.parquet".to_string());
        let file = rfd::AsyncFileDialog::new()
            .set_title("Save Optimized Parquet File - Sieve")
            .set_file_name(&name)
            .add_filter("Apache Parquet (*.parquet)", &["parquet"])
            .save_file()
            .await;

        Ok(file.map(|f| f.path().to_string_lossy().to_string()))
    }
    #[cfg(not(feature = "gui"))]
    {
        Ok(None)
    }
}
