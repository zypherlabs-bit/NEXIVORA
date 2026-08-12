//! Shared types for the document engine.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)] // Public API type, constructed by higher-level document operations
pub struct Document {
    pub id: String,
    pub title: Option<String>,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Section {
    pub properties: SectionProperties,
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionProperties {
    pub page_width: f64,
    pub page_height: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub margin_right: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paragraph {
    pub style: String,
    pub runs: Vec<Run>,
}

impl Paragraph {
    pub fn new(style: impl Into<String>) -> Self {
        Self {
            style: style.into(),
            runs: Vec::new(),
        }
    }

    pub fn add_run(&mut self, run: Run) {
        self.runs.push(run);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Run {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font: Option<String>,
    pub size: Option<f64>,
    pub color: Option<String>,
}
