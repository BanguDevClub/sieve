#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[link(name = "resolv")]
extern "C" {}

#[cfg(feature = "gui")]
pub mod commands;
pub mod duckdb_engine;
pub mod state;

#[cfg(feature = "gui")]
use commands::*;
#[cfg(feature = "gui")]
use state::AppState;

#[cfg(feature = "gui")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new().expect("Failed to initialize DuckDB engine");

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            open_csv_file,
            pick_csv_file,
            load_sample_dataset,
            query_page,
            run_custom_sql,
            export_data,
            cancel_current_operation,
            get_metadata,
            optimize_to_parquet,
            pick_save_parquet_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running Sieve application");
}
