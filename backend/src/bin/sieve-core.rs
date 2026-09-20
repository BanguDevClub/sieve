use std::env;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use sieve::duckdb_engine::DuckDbEngine;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[no_mangle]
pub unsafe extern "C" fn res_init() -> std::ffi::c_int {
    extern "C" {
        fn __res_init() -> std::ffi::c_int;
    }
    __res_init()
}

fn main() {
    println!("Sieve Core CLI v0.1.0 by BanguDevClub");
    println!("High-performance DuckDB out-of-core CSV processor");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("\nUsage:");
        println!("  sieve-core <csv_path> [sql_query]");
        println!("\nExamples:");
        println!("  sieve-core dataset.csv");
        println!("  sieve-core dataset.csv \"SELECT * FROM sieve LIMIT 10;\"");
        return;
    }

    let csv_path = &args[1];
    let query = args.get(2).cloned().unwrap_or_else(|| "SELECT * FROM sieve LIMIT 5;".to_string());

    let mut engine = match DuckDbEngine::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error initializing DuckDB engine: {}", e);
            std::process::exit(1);
        }
    };

    let cancel = Arc::new(AtomicBool::new(false));
    println!("Opening CSV file: {}", csv_path);
    let meta = match engine.open_csv(csv_path, cancel.clone(), |p| {
        println!("[{}] {}% - {}", p.phase, p.percentage, p.message);
    }) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to open CSV: {}", e);
            std::process::exit(1);
        }
    };

    println!("\nFile Metadata:");
    println!("  Rows: {}", meta.total_rows);
    println!("  Columns: {}", meta.total_columns);
    println!("  Load time: {} ms", meta.load_time_ms);

    println!("\nExecuting query: {}", query);
    match engine.execute_sql(&query, Some(20), cancel) {
        Ok(res) => {
            println!("Query executed in {} ms ({} rows returned):", res.execution_time_ms, res.total_rows);
            for col in &res.columns {
                print!("{:<20} | ", col.name);
            }
            println!();
            println!("{}", "-".repeat(res.columns.len() * 23));
            for row in res.rows {
                for val in row {
                    print!("{:<20} | ", val.to_string());
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("Query failed: {}", e);
            std::process::exit(1);
        }
    }
}
