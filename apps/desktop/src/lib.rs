//! Nexivora Desktop Application
//!
//! Tauri-based desktop shell that integrates the spreadsheet and document engines.

use nexivora_spreadsheet_engine::{Spreadsheet, Sheet};
use nexivora_document_engine::Document;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub spreadsheet: Spreadsheet,
    pub document: Document,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            spreadsheet: Spreadsheet::new(),
            document: Document::new(),
        }
    }
}