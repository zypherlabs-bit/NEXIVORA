//! Core shared data types for Nexivora.

use serde::{Deserialize, Serialize};

/// A cell reference identifying a position in a sheet, in A1 notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellRef {
    /// 0-based row index.
    pub row: u32,
    /// 0-based column index.
    pub col: u32,
}

impl CellRef {
    /// Create a new cell reference.
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }

    /// Parse an A1-style reference like "B3" (0-based: row 2, col 1).
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        let bytes = s.as_bytes();
        let mut col: u32 = 0;
        let mut idx = 0;
        while idx < bytes.len() && bytes[idx].is_ascii_alphabetic() {
            let c = bytes[idx].to_ascii_uppercase();
            col = col * 26 + (c - b'A') as u32 + 1;
            idx += 1;
        }
        if col == 0 {
            return None;
        }
        let row_str = &s[idx..];
        if row_str.is_empty() {
            return None;
        }
        let row: u32 = row_str.parse().ok()?;
        if row == 0 {
            return None;
        }
        Some(CellRef::new(row - 1, col - 1))
    }

    /// Render as A1 notation using 1-based row numbers.
    pub fn to_a1(&self) -> String {
        let mut col = self.col + 1;
        let mut name = String::new();
        while col > 0 {
            let rem = (col - 1) % 26;
            name.insert(0, (b'A' + rem as u8) as char);
            col = (col - 1) / 26;
        }
        format!("{}{}", name, self.row + 1)
    }
}

/// A rectangular cell range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellRange {
    /// Top-left corner.
    pub start: CellRef,
    /// Bottom-right corner (inclusive).
    pub end: CellRef,
}

impl CellRange {
    /// Create a new range from two corners, normalizing order.
    pub fn new(a: CellRef, b: CellRef) -> Self {
        let start = CellRef::new(a.row.min(b.row), a.col.min(b.col));
        let end = CellRef::new(a.row.max(b.row), a.col.max(b.col));
        Self { start, end }
    }

    /// The number of rows in this range.
    pub fn row_count(&self) -> u32 {
        self.end.row - self.start.row + 1
    }

    /// The number of columns in this range.
    pub fn col_count(&self) -> u32 {
        self.end.col - self.start.col + 1
    }

    /// Iterate over all cells in the range, row-major.
    pub fn cells(&self) -> impl Iterator<Item = CellRef> {
        let start = self.start;
        let end = self.end;
        (start.row..=end.row).flat_map(move |r| {
            (start.col..=end.col).map(move |c| CellRef::new(r, c))
        })
    }

    /// Render as A1 range notation.
    pub fn to_a1(&self) -> String {
        format!("{}:{}", self.start.to_a1(), self.end.to_a1())
    }
}

/// The category or kind of a Nexivora document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentKind {
    /// Text document (.nxdoc)
    Document,
    /// Spreadsheet (.nxsheet)
    Spreadsheet,
    /// Presentation (.nxpres)
    Presentation,
    /// Database (.nxdb)
    Database,
    /// Drawing (.nxdraw)
    Drawing,
    /// Form (.nxform)
    Form,
}

impl DocumentKind {
    /// The canonical file extension for this document kind.
    pub fn extension(&self) -> &'static str {
        match self {
            DocumentKind::Document => "nxdoc",
            DocumentKind::Spreadsheet => "nxsheet",
            DocumentKind::Presentation => "nxpres",
            DocumentKind::Database => "nxdb",
            DocumentKind::Drawing => "nxdraw",
            DocumentKind::Form => "nxform",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_ref_a1_roundtrip() {
        for (row, col, a1) in [
            (0u32, 0u32, "A1"),
            (0, 1, "B1"),
            (2, 2, "C3"),
            (9, 25, "Z10"),
            (9, 26, "AA10"),
            (9, 27, "AB10"),
            (99, 701, "ZZ100"),
            (999, 702, "AAA1000"),
        ] {
            let r = CellRef::new(row, col);
            assert_eq!(r.to_a1(), a1);
            assert_eq!(CellRef::parse(a1), Some(r));
        }
    }

    #[test]
    fn cell_ref_parse_invalid() {
        assert_eq!(CellRef::parse(""), None);
        assert_eq!(CellRef::parse("A0"), None);
        assert_eq!(CellRef::parse("1A"), None);
        assert_eq!(CellRef::parse("A"), None);
    }

    #[test]
    fn range_normalizes() {
        let a = CellRef::new(5, 5);
        let b = CellRef::new(0, 0);
        let r = CellRange::new(a, b);
        assert_eq!(r.start, CellRef::new(0, 0));
        assert_eq!(r.end, CellRef::new(5, 5));
        assert_eq!(r.row_count(), 6);
        assert_eq!(r.col_count(), 6);
        assert_eq!(r.cells().count(), 36);
    }

    #[test]
    fn document_kind_extensions() {
        assert_eq!(DocumentKind::Document.extension(), "nxdoc");
        assert_eq!(DocumentKind::Spreadsheet.extension(), "nxsheet");
        assert_eq!(DocumentKind::Database.extension(), "nxdb");
    }
}