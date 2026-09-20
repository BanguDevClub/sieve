use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use duckdb::types::ValueRef;
use duckdb::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvMetadata {
    pub file_path: String,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub total_rows: u64,
    pub total_columns: usize,
    pub columns: Vec<ColumnInfo>,
    pub load_time_ms: u128,
    pub file_format: String,
    pub is_optimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParquetOptimizationOptions {
    pub target_path: String,
    #[serde(default)]
    pub source_path: Option<String>,
    pub compression: String,
    pub row_group_size: usize,
    pub sort_column: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub target_path: String,
    pub original_size_bytes: u64,
    pub parquet_size_bytes: u64,
    pub space_saved_percent: f64,
    pub execution_time_ms: u128,
    pub total_rows: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRule {
    pub column: String,
    pub operator: String, // "equals", "contains", "starts_with", "ends_with", "in", "gt", "lt", "gte", "lte", "is_empty", "is_not_empty"
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortRule {
    pub column: String,
    pub ascending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: usize,           // 1-based index
    pub page_size: usize,      // max 100
    pub col_offset: usize,     // 0-based
    pub col_limit: usize,      // max 50
    pub filters: Vec<FilterRule>,
    pub filter_conjunction: String, // "AND" or "OR"
    pub sort: Option<SortRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult {
    pub page: usize,
    pub page_size: usize,
    pub total_filtered_rows: u64,
    pub total_pages: usize,
    pub col_offset: usize,
    pub col_limit: usize,
    pub total_columns: usize,
    pub displayed_columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub execution_time_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub total_rows: usize,
    pub execution_time_ms: u128,
    pub sql_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub phase: String,
    pub percentage: u8,
    pub elapsed_ms: u128,
    pub estimated_rows: u64,
    pub message: String,
}

pub struct DuckDbEngine {
    conn: Connection,
    current_file: Option<String>,
    metadata: Option<CsvMetadata>,
}

impl DuckDbEngine {
    pub fn new() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;

        // Optimize DuckDB for multi-format out-of-core scanning
        let _ = conn.execute("SET preserve_insertion_order = false;", []);
        let _ = conn.execute("PRAGMA enable_progress_bar = true;", []);
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        let _ = conn.execute(&format!("SET threads = {};", threads), []);

        Ok(Self {
            conn,
            current_file: None,
            metadata: None,
        })
    }

    pub fn open_csv<F>(
        &mut self,
        file_path: &str,
        cancel_token: Arc<AtomicBool>,
        mut progress_cb: F,
    ) -> Result<CsvMetadata, String>
    where
        F: FnMut(ProgressUpdate),
    {
        let start_time = Instant::now();

        let metadata_fs = std::fs::metadata(file_path).map_err(|e| {
            format!("Cannot access file '{}': {}", file_path, e)
        })?;
        let file_size = metadata_fs.len();
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("dataset.csv")
            .to_string();

        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let (file_format, is_optimized) = match ext.as_str() {
            "parquet" | "pq" => ("parquet".to_string(), true),
            "json" | "jsonl" | "ndjson" => ("json".to_string(), false),
            "xlsx" | "xls" | "xlsb" | "ods" => ("excel".to_string(), false),
            "tsv" | "tab" => ("tsv".to_string(), false),
            _ => ("csv".to_string(), false),
        };

        progress_cb(ProgressUpdate {
            phase: "Initializing".to_string(),
            percentage: 10,
            elapsed_ms: start_time.elapsed().as_millis(),
            estimated_rows: 0,
            message: format!("Mapping {} file '{}' into DuckDB...", file_format.to_uppercase(), file_name),
        });

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Operation cancelled by user".to_string());
        }

        // Escape path safely for SQL string literal
        let escaped_path = file_path.replace('\'', "''");

        match file_format.as_str() {
            "parquet" => {
                let sql = format!(
                    "CREATE OR REPLACE VIEW sieve AS SELECT * FROM read_parquet('{}');",
                    escaped_path
                );
                self.conn.execute(&sql, []).map_err(|e| format!("Failed to read Parquet: {}", e))?;
            }
            "json" => {
                let sql = format!(
                    "CREATE OR REPLACE VIEW sieve AS SELECT * FROM read_json_auto('{}');",
                    escaped_path
                );
                self.conn.execute(&sql, []).map_err(|e| format!("Failed to read JSON: {}", e))?;
            }
            "excel" => {
                let temp_csv = load_excel_to_temp_csv(file_path)?;
                let temp_path = temp_csv.path().to_str().ok_or("Invalid temp buffer path")?;
                let escaped_temp = temp_path.replace('\'', "''");
                let sql = format!(
                    "CREATE OR REPLACE TABLE sieve AS SELECT * FROM read_csv_auto('{}', union_by_name=True);",
                    escaped_temp
                );
                self.conn.execute(&sql, []).map_err(|e| format!("Failed to ingest Excel workbook: {}", e))?;
            }
            _ => {
                let sql = format!(
                    "CREATE OR REPLACE VIEW sieve AS SELECT * FROM read_csv_auto('{}', union_by_name=True);",
                    escaped_path
                );
                self.conn.execute(&sql, []).map_err(|e| format!("Failed to read CSV: {}", e))?;
            }
        }

        progress_cb(ProgressUpdate {
            phase: "Inspecting Schema".to_string(),
            percentage: 35,
            elapsed_ms: start_time.elapsed().as_millis(),
            estimated_rows: 0,
            message: "Inferring column types and data schema...".to_string(),
        });

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Operation cancelled by user".to_string());
        }

        // Fetch column schema
        let mut columns = Vec::new();
        let mut stmt = self
            .conn
            .prepare("DESCRIBE sieve;")
            .map_err(|e| format!("Failed to describe view: {}", e))?;

        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let col_name: String = row.get(0).map_err(|e| e.to_string())?;
            let col_type: String = row.get(1).map_err(|e| e.to_string())?;
            columns.push(ColumnInfo {
                name: col_name,
                data_type: col_type,
            });
        }

        progress_cb(ProgressUpdate {
            phase: "Counting Records".to_string(),
            percentage: 60,
            elapsed_ms: start_time.elapsed().as_millis(),
            estimated_rows: 0,
            message: "Calculating total record count...".to_string(),
        });

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Operation cancelled by user".to_string());
        }

        // Count total rows
        let total_rows: u64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM sieve;", [], |row| row.get(0))
            .unwrap_or(0);

        let elapsed = start_time.elapsed().as_millis();

        progress_cb(ProgressUpdate {
            phase: "Complete".to_string(),
            percentage: 100,
            elapsed_ms: elapsed,
            estimated_rows: total_rows,
            message: format!("Loaded {} records across {} columns ({} format).", total_rows, columns.len(), file_format.to_uppercase()),
        });

        let meta = CsvMetadata {
            file_path: file_path.to_string(),
            file_name,
            file_size_bytes: file_size,
            total_rows,
            total_columns: columns.len(),
            columns,
            load_time_ms: elapsed,
            file_format,
            is_optimized,
        };

        self.current_file = Some(file_path.to_string());
        self.metadata = Some(meta.clone());

        Ok(meta)
    }

    pub fn optimize_to_parquet(
        &mut self,
        options: &ParquetOptimizationOptions,
        cancel_token: Arc<AtomicBool>,
    ) -> Result<OptimizationResult, String> {
        let start_time = Instant::now();

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Operation cancelled by user".to_string());
        }

        // If a specific source_path was requested, open it first into DuckDB
        if let Some(src) = &options.source_path {
            if self.current_file.as_deref() != Some(src) {
                self.open_csv(src, cancel_token.clone(), |_| {})?;
            }
        }

        let meta = self.metadata.as_ref().ok_or("No dataset loaded to optimize")?;
        let orig_size = meta.file_size_bytes;

        let escaped_target = options.target_path.replace('\'', "''");
        let compression = match options.compression.to_uppercase().as_str() {
            "SNAPPY" => "SNAPPY",
            "GZIP" => "GZIP",
            "UNCOMPRESSED" => "UNCOMPRESSED",
            _ => "ZSTD",
        };
        let row_group_size = if options.row_group_size > 0 {
            options.row_group_size
        } else {
            122880
        };

        let mut select_sql = "SELECT * FROM sieve".to_string();
        if let Some(sort_col) = &options.sort_column {
            if !sort_col.trim().is_empty() {
                let escaped_col = format!("\"{}\"", sort_col.replace('"', "\"\""));
                select_sql.push_str(&format!(" ORDER BY {}", escaped_col));
            }
        }

        let copy_sql = format!(
            "COPY ({}) TO '{}' (FORMAT PARQUET, COMPRESSION '{}', ROW_GROUP_SIZE {});",
            select_sql, escaped_target, compression, row_group_size
        );

        self.conn
            .execute(&copy_sql, [])
            .map_err(|e| format!("Parquet optimization failed: {}", e))?;

        let parquet_meta = std::fs::metadata(&options.target_path)
            .map_err(|e| format!("Cannot read created Parquet file: {}", e))?;
        let parquet_size = parquet_meta.len();

        let space_saved_percent = if orig_size > 0 && parquet_size < orig_size {
            ((orig_size - parquet_size) as f64 / orig_size as f64) * 100.0
        } else {
            0.0
        };

        Ok(OptimizationResult {
            target_path: options.target_path.clone(),
            original_size_bytes: orig_size,
            parquet_size_bytes: parquet_size,
            space_saved_percent,
            execution_time_ms: start_time.elapsed().as_millis(),
            total_rows: meta.total_rows,
        })
    }

    pub fn get_metadata(&self) -> Option<CsvMetadata> {
        self.metadata.clone()
    }

    pub fn query_page(
        &self,
        req: PageRequest,
        cancel_token: Arc<AtomicBool>,
    ) -> Result<PageResult, String> {
        let start_time = Instant::now();
        let meta = self
            .metadata
            .as_ref()
            .ok_or_else(|| "No CSV file currently loaded".to_string())?;

        // Enforce max 100 rows per page
        let page_size = req.page_size.clamp(1, 100);
        let page = req.page.max(1);
        let offset = (page - 1) * page_size;

        // Enforce max 50 columns per window
        let col_limit = req.col_limit.clamp(1, 50);
        let col_offset = req.col_offset.min(meta.columns.len().saturating_sub(1));
        let col_end = (col_offset + col_limit).min(meta.columns.len());
        let displayed_cols = meta.columns[col_offset..col_end].to_vec();

        // Build WHERE clause
        let where_clause = self.build_where_clause(&req.filters, &req.filter_conjunction)?;

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Query cancelled".to_string());
        }

        // Count filtered rows
        let count_sql = if where_clause.is_empty() {
            "SELECT COUNT(*) FROM sieve;".to_string()
        } else {
            format!("SELECT COUNT(*) FROM sieve WHERE {};", where_clause)
        };

        let total_filtered_rows: u64 = self
            .conn
            .query_row(&count_sql, [], |row| row.get(0))
            .map_err(|e| format!("Error counting filtered rows: {}", e))?;

        // Build SELECT columns
        let col_select = displayed_cols
            .iter()
            .map(|c| format!("\"{}\"", c.name.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query_sql = if where_clause.is_empty() {
            format!("SELECT {} FROM sieve", col_select)
        } else {
            format!("SELECT {} FROM sieve WHERE {}", col_select, where_clause)
        };

        // Add ORDER BY
        if let Some(sort) = &req.sort {
            let escaped_col = format!("\"{}\"", sort.column.replace('"', "\"\""));
            let dir = if sort.ascending { "ASC" } else { "DESC" };
            query_sql.push_str(&format!(" ORDER BY {} {}", escaped_col, dir));
        }

        query_sql.push_str(&format!(" LIMIT {} OFFSET {};", page_size, offset));

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Query cancelled".to_string());
        }

        let mut stmt = self
            .conn
            .prepare(&query_sql)
            .map_err(|e| format!("Query preparation failed: {}", e))?;

        let mut rows = Vec::new();
        let mut query_rows = stmt.query([]).map_err(|e| e.to_string())?;

        while let Some(row) = query_rows.next().map_err(|e| e.to_string())? {
            let mut row_values = Vec::new();
            for i in 0..displayed_cols.len() {
                let val_ref = row.get_ref(i).map_err(|e| e.to_string())?;
                row_values.push(value_ref_to_json(val_ref));
            }
            rows.push(row_values);
        }

        let total_pages = if total_filtered_rows == 0 {
            1
        } else {
            ((total_filtered_rows as f64) / (page_size as f64)).ceil() as usize
        };

        Ok(PageResult {
            page,
            page_size,
            total_filtered_rows,
            total_pages,
            col_offset,
            col_limit: displayed_cols.len(),
            total_columns: meta.columns.len(),
            displayed_columns: displayed_cols,
            rows,
            execution_time_ms: start_time.elapsed().as_millis(),
        })
    }

    pub fn execute_sql(
        &self,
        sql: &str,
        max_rows: Option<usize>,
        cancel_token: Arc<AtomicBool>,
    ) -> Result<SqlResult, String> {
        let start_time = Instant::now();

        if cancel_token.load(Ordering::Relaxed) {
            return Err("Query cancelled".to_string());
        }

        let mut stmt = self
            .conn
            .prepare(sql)
            .map_err(|e| format!("SQL syntax error: {}", e))?;

        let mut query_rows = stmt.query([]).map_err(|e| e.to_string())?;

        let col_names = query_rows
            .as_ref()
            .map(|s| s.column_names())
            .unwrap_or_default();

        let columns: Vec<ColumnInfo> = col_names
            .into_iter()
            .map(|name| ColumnInfo {
                name,
                data_type: "UNKNOWN".to_string(),
            })
            .collect();

        let mut rows = Vec::new();
        while let Some(row) = query_rows.next().map_err(|e| e.to_string())? {
            if cancel_token.load(Ordering::Relaxed) {
                return Err("Query cancelled".to_string());
            }

            let mut row_values = Vec::new();
            for i in 0..columns.len() {
                let val_ref = row.get_ref(i).map_err(|e| e.to_string())?;
                row_values.push(value_ref_to_json(val_ref));
            }
            rows.push(row_values);

            if let Some(limit) = max_rows {
                if rows.len() >= limit {
                    break;
                }
            }
        }

        Ok(SqlResult {
            columns,
            total_rows: rows.len(),
            rows,
            execution_time_ms: start_time.elapsed().as_millis(),
            sql_query: sql.to_string(),
        })
    }

    pub fn export_data(
        &self,
        target_path: &str,
        format: &str,
        custom_sql: Option<String>,
    ) -> Result<String, String> {
        let base_sql = custom_sql.unwrap_or_else(|| "SELECT * FROM sieve".to_string());
        let escaped_path = target_path.replace('\'', "''");

        let copy_sql = match format.to_uppercase().as_str() {
            "PARQUET" => format!("COPY ({}) TO '{}' (FORMAT PARQUET);", base_sql, escaped_path),
            _ => format!("COPY ({}) TO '{}' (FORMAT CSV, HEADER);", base_sql, escaped_path),
        };

        self.conn
            .execute(&copy_sql, [])
            .map_err(|e| format!("Export failed: {}", e))?;

        Ok(format!("Exported to {}", target_path))
    }

    fn build_where_clause(
        &self,
        filters: &[FilterRule],
        conjunction: &str,
    ) -> Result<String, String> {
        if filters.is_empty() {
            return Ok(String::new());
        }

        let conj = if conjunction.eq_ignore_ascii_case("OR") {
            " OR "
        } else {
            " AND "
        };

        let mut clauses = Vec::new();

        for f in filters {
            let col = format!("\"{}\"", f.column.replace('"', "\"\""));
            let val_escaped = f.value.replace('\'', "''");

            let clause = match f.operator.as_str() {
                "equals" => format!("CAST({} AS VARCHAR) = '{}'", col, val_escaped),
                "contains" | "fuzzy" => {
                    format!("LOWER(CAST({} AS VARCHAR)) LIKE LOWER('%{}%')", col, val_escaped)
                }
                "starts_with" => {
                    format!("LOWER(CAST({} AS VARCHAR)) LIKE LOWER('{}%')", col, val_escaped)
                }
                "ends_with" => {
                    format!("LOWER(CAST({} AS VARCHAR)) LIKE LOWER('%{}')", col, val_escaped)
                }
                "in" => {
                    let items: Vec<String> = f
                        .value
                        .split(',')
                        .map(|s| format!("'{}'", s.trim().replace('\'', "''")))
                        .collect();
                    if items.is_empty() {
                        continue;
                    }
                    format!("CAST({} AS VARCHAR) IN ({})", col, items.join(", "))
                }
                "gt" => format!("TRY_CAST({} AS DOUBLE) > {}", col, val_escaped),
                "gte" => format!("TRY_CAST({} AS DOUBLE) >= {}", col, val_escaped),
                "lt" => format!("TRY_CAST({} AS DOUBLE) < {}", col, val_escaped),
                "lte" => format!("TRY_CAST({} AS DOUBLE) <= {}", col, val_escaped),
                "is_empty" => format!("({} IS NULL OR TRIM(CAST({} AS VARCHAR)) = '')", col, col),
                "is_not_empty" => {
                    format!("({} IS NOT NULL AND TRIM(CAST({} AS VARCHAR)) != '')", col, col)
                }
                _ => format!("CAST({} AS VARCHAR) = '{}'", col, val_escaped),
            };

            clauses.push(clause);
        }

        if clauses.is_empty() {
            Ok(String::new())
        } else {
            Ok(clauses.join(conj))
        }
    }
}

fn load_excel_to_temp_csv(file_path: &str) -> Result<tempfile::NamedTempFile, String> {
    use calamine::{Data, Reader};
    use std::io::Write;

    let mut workbook = calamine::open_workbook_auto(file_path)
        .map_err(|e| format!("Failed to open Excel workbook: {}", e))?;
    let sheet_names = workbook.sheet_names();
    let first_sheet = sheet_names.first().cloned().ok_or("No sheets found in workbook")?;
    let range = workbook
        .worksheet_range(&first_sheet)
        .map_err(|e| format!("Failed to read sheet '{}': {}", first_sheet, e))?;

    let mut temp_file = tempfile::Builder::new()
        .prefix("sieve_excel_")
        .suffix(".csv")
        .tempfile()
        .map_err(|e| format!("Failed to create temporary buffer file: {}", e))?;

    for row in range.rows() {
        let mut line = String::new();
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                line.push(',');
            }
            match cell {
                Data::Empty => {}
                Data::String(s) => {
                    line.push('"');
                    line.push_str(&s.replace('"', "\"\""));
                    line.push('"');
                }
                Data::Float(f) => line.push_str(&f.to_string()),
                Data::Int(n) => line.push_str(&n.to_string()),
                Data::Bool(b) => line.push_str(if *b { "true" } else { "false" }),
                Data::DateTime(d) => line.push_str(&d.to_string()),
                Data::DateTimeIso(iso) => line.push_str(iso),
                Data::DurationIso(dur) => line.push_str(dur),
                Data::Error(e) => line.push_str(&format!("{:?}", e)),
            }
        }
        line.push('\n');
        temp_file
            .write_all(line.as_bytes())
            .map_err(|e| format!("Error writing Excel data to buffer: {}", e))?;
    }

    temp_file.flush().map_err(|e| format!("Failed to flush Excel buffer: {}", e))?;
    Ok(temp_file)
}

fn value_ref_to_json(val: ValueRef) -> serde_json::Value {
    match val {
        ValueRef::Null => serde_json::Value::Null,
        ValueRef::Boolean(b) => serde_json::Value::Bool(b),
        ValueRef::TinyInt(i) => serde_json::json!(i),
        ValueRef::SmallInt(i) => serde_json::json!(i),
        ValueRef::Int(i) => serde_json::json!(i),
        ValueRef::BigInt(i) => serde_json::json!(i),
        ValueRef::HugeInt(i) => serde_json::json!(i.to_string()),
        ValueRef::UTinyInt(u) => serde_json::json!(u),
        ValueRef::USmallInt(u) => serde_json::json!(u),
        ValueRef::UInt(u) => serde_json::json!(u),
        ValueRef::UBigInt(u) => serde_json::json!(u),
        ValueRef::Float(f) => serde_json::json!(f),
        ValueRef::Double(d) => serde_json::json!(d),
        ValueRef::Decimal(d) => serde_json::json!(d.to_string()),
        ValueRef::Text(s) => {
            serde_json::Value::String(String::from_utf8_lossy(s).into_owned())
        }
        ValueRef::Blob(b) => {
            serde_json::Value::String(format!("<Blob {} bytes>", b.len()))
        }
        ValueRef::Date32(d) => {
            if let Some(dt) = chrono::DateTime::from_timestamp((d as i64) * 86400, 0) {
                serde_json::Value::String(dt.format("%Y-%m-%d").to_string())
            } else {
                serde_json::json!(d)
            }
        }
        ValueRef::Time64(unit, val) => {
            let micros = match unit {
                duckdb::types::TimeUnit::Second => val * 1_000_000,
                duckdb::types::TimeUnit::Millisecond => val * 1_000,
                duckdb::types::TimeUnit::Microsecond => val,
                duckdb::types::TimeUnit::Nanosecond => val / 1_000,
            };
            let secs = (micros / 1_000_000) % 86400;
            let h = secs / 3600;
            let m = (secs % 3600) / 60;
            let s = secs % 60;
            serde_json::Value::String(format!("{:02}:{:02}:{:02}", h, m, s))
        }
        ValueRef::Timestamp(unit, val) => {
            let (secs, nanos) = match unit {
                duckdb::types::TimeUnit::Second => (val, 0),
                duckdb::types::TimeUnit::Millisecond => (val / 1000, ((val.rem_euclid(1000)) * 1_000_000) as u32),
                duckdb::types::TimeUnit::Microsecond => (val / 1_000_000, ((val.rem_euclid(1_000_000)) * 1_000) as u32),
                duckdb::types::TimeUnit::Nanosecond => (val / 1_000_000_000, (val.rem_euclid(1_000_000_000)) as u32),
            };
            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
                serde_json::Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string())
            } else {
                serde_json::json!(val)
            }
        }
        _ => serde_json::Value::String(format!("{:?}", val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_test_csv() -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            file,
            "id,name,department,salary,active\n1,Alice,Engineering,95000,true\n2,Bob,Finance,80000,false\n3,Charlie,Engineering,110000,true\n4,Diana,Marketing,72000,true\n5,Evan,Finance,85000,true"
        )
        .unwrap();
        file
    }

    #[test]
    fn test_open_csv_and_query() {
        let csv_file = create_test_csv();
        let path = csv_file.path().to_str().unwrap();

        let mut engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));

        let meta = engine.open_csv(path, cancel.clone(), |_| {}).unwrap();
        assert_eq!(meta.total_rows, 5);
        assert_eq!(meta.total_columns, 5);

        // Test paginated query
        let page_req = PageRequest {
            page: 1,
            page_size: 100,
            col_offset: 0,
            col_limit: 50,
            filters: vec![],
            filter_conjunction: "AND".to_string(),
            sort: None,
        };

        let res = engine.query_page(page_req, cancel.clone()).unwrap();
        assert_eq!(res.rows.len(), 5);
        assert_eq!(res.total_filtered_rows, 5);
    }

    #[test]
    fn test_filter_exact_and_fuzzy() {
        let csv_file = create_test_csv();
        let path = csv_file.path().to_str().unwrap();

        let mut engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        engine.open_csv(path, cancel.clone(), |_| {}).unwrap();

        // Exact filter: department = Engineering
        let page_req = PageRequest {
            page: 1,
            page_size: 100,
            col_offset: 0,
            col_limit: 50,
            filters: vec![FilterRule {
                column: "department".to_string(),
                operator: "equals".to_string(),
                value: "Engineering".to_string(),
            }],
            filter_conjunction: "AND".to_string(),
            sort: None,
        };

        let res = engine.query_page(page_req, cancel.clone()).unwrap();
        assert_eq!(res.total_filtered_rows, 2);

        // Fuzzy filter: name contains 'li' (Alice, Charlie)
        let page_req2 = PageRequest {
            page: 1,
            page_size: 100,
            col_offset: 0,
            col_limit: 50,
            filters: vec![FilterRule {
                column: "name".to_string(),
                operator: "contains".to_string(),
                value: "li".to_string(),
            }],
            filter_conjunction: "AND".to_string(),
            sort: None,
        };

        let res2 = engine.query_page(page_req2, cancel.clone()).unwrap();
        assert_eq!(res2.total_filtered_rows, 2);
    }

    #[test]
    fn test_custom_sql() {
        let csv_file = create_test_csv();
        let path = csv_file.path().to_str().unwrap();

        let mut engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        engine.open_csv(path, cancel.clone(), |_| {}).unwrap();

        let sql = "SELECT department, COUNT(*) as count FROM sieve GROUP BY department ORDER BY department;";
        let res = engine.execute_sql(sql, None, cancel).unwrap();
        assert_eq!(res.rows.len(), 3); // Engineering, Finance, Marketing
    }

    #[test]
    fn test_custom_sql_unlimited_rows() {
        let engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));

        // Test querying 250 rows with max_rows = None (should return all 250 rows, not clamped to 100)
        let sql = "SELECT range AS id FROM range(250);";
        let res = engine.execute_sql(sql, None, cancel.clone()).unwrap();
        assert_eq!(res.rows.len(), 250);

        // Test querying with explicit max_rows = Some(150) (should return 150 rows, not clamped to 100)
        let res_limited = engine.execute_sql(sql, Some(150), cancel).unwrap();
        assert_eq!(res_limited.rows.len(), 150);
    }

    #[test]
    fn test_date_formatting() {
        let engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        let sql = "SELECT DATE '2024-02-01' as d, TIMESTAMP '2024-02-01 14:30:00' as ts;";
        let res = engine.execute_sql(sql, Some(10), cancel).unwrap();
        assert_eq!(res.rows.len(), 1);
        assert_eq!(res.rows[0][0], serde_json::json!("2024-02-01"));
        assert_eq!(res.rows[0][1], serde_json::json!("2024-02-01 14:30:00"));
    }

    #[test]
    fn test_parquet_optimization_and_reading() {
        let csv_file = create_test_csv();
        let path = csv_file.path().to_str().unwrap();

        let mut engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        let meta = engine.open_csv(path, cancel.clone(), |_| {}).unwrap();
        assert_eq!(meta.file_format, "csv");
        assert_eq!(meta.is_optimized, false);

        let temp_parquet = tempfile::Builder::new().suffix(".parquet").tempfile().unwrap();
        let parquet_path = temp_parquet.path().to_str().unwrap().to_string();

        let opt_options = ParquetOptimizationOptions {
            target_path: parquet_path.clone(),
            source_path: None,
            compression: "ZSTD".to_string(),
            row_group_size: 122880,
            sort_column: Some("department".to_string()),
        };

        let opt_res = engine.optimize_to_parquet(&opt_options, cancel.clone()).unwrap();
        assert_eq!(opt_res.total_rows, 5);
        assert!(opt_res.parquet_size_bytes > 0);

        // Now open the generated Parquet file directly
        let mut engine2 = DuckDbEngine::new().unwrap();
        let p_meta = engine2.open_csv(&parquet_path, cancel.clone(), |_| {}).unwrap();
        assert_eq!(p_meta.file_format, "parquet");
        assert_eq!(p_meta.is_optimized, true);
        assert_eq!(p_meta.total_rows, 5);

        let page_req = PageRequest {
            page: 1,
            page_size: 10,
            col_offset: 0,
            col_limit: 10,
            filters: vec![],
            filter_conjunction: "AND".to_string(),
            sort: None,
        };
        let page_res = engine2.query_page(page_req, cancel).unwrap();
        assert_eq!(page_res.rows.len(), 5);
    }

    #[test]
    fn test_pre_open_parquet_optimization() {
        let cancel = Arc::new(AtomicBool::new(false));
        let csv_data = "id,name,score\n1,Alpha,95.5\n2,Beta,88.0\n3,Gamma,92.3\n";
        let temp_csv = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
        std::fs::write(temp_csv.path(), csv_data).unwrap();
        let csv_path = temp_csv.path().to_str().unwrap().to_string();

        let temp_parquet = tempfile::Builder::new().suffix(".parquet").tempfile().unwrap();
        let parquet_path = temp_parquet.path().to_str().unwrap().to_string();

        // Fresh engine with nothing opened yet
        let mut engine = DuckDbEngine::new().unwrap();
        let opt_options = ParquetOptimizationOptions {
            target_path: parquet_path.clone(),
            source_path: Some(csv_path),
            compression: "ZSTD".to_string(),
            row_group_size: 65536,
            sort_column: None,
        };

        let opt_res = engine.optimize_to_parquet(&opt_options, cancel.clone()).unwrap();
        assert_eq!(opt_res.total_rows, 3);
        assert!(opt_res.parquet_size_bytes > 0);
        assert!(opt_res.target_path == parquet_path);

        // Open newly created Parquet file and verify
        let meta = engine.open_csv(&parquet_path, cancel, |_| {}).unwrap();
        assert_eq!(meta.file_format, "parquet");
        assert_eq!(meta.total_rows, 3);
    }

    #[test]
    fn test_json_ingestion() {
        let mut file = tempfile::Builder::new().suffix(".ndjson").tempfile().unwrap();
        writeln!(file, "{{\"id\": 1, \"title\": \"Notebook\", \"price\": 12.50}}").unwrap();
        writeln!(file, "{{\"id\": 2, \"title\": \"Keyboard\", \"price\": 45.00}}").unwrap();
        file.flush().unwrap();
        let path = file.path().to_str().unwrap();

        let mut engine = DuckDbEngine::new().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        let meta = engine.open_csv(path, cancel.clone(), |_| {}).unwrap();
        assert_eq!(meta.file_format, "json");
        assert_eq!(meta.total_rows, 2);
    }
}
