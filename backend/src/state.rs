use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use crate::duckdb_engine::DuckDbEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<DuckDbEngine>>,
    pub cancel_token: Arc<AtomicBool>,
}

impl AppState {
    pub fn new() -> Result<Self, String> {
        let engine = DuckDbEngine::new()?;
        Ok(Self {
            engine: Arc::new(Mutex::new(engine)),
            cancel_token: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn reset_cancel(&self) {
        self.cancel_token.store(false, Ordering::Relaxed);
    }

    pub fn trigger_cancel(&self) {
        self.cancel_token.store(true, Ordering::Relaxed);
    }
}
