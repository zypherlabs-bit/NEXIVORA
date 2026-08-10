//! Formula abstract syntax tree.

use std::fmt;

use nexivora_core::CellRef;

/// A reference to a cell or range, with support for absolute/mixed addressing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellReference {
    /// 0-based row.
    pub row: u32,
    /// 0-based column.
    pub col: u32,
    /// Whether the row part is absolute (`$`).
    pub row_absolute: bool,
    /// Whether the column part is absolute (`$`).
    pub col_absolute: bool,
}

impl CellReference {
    /// Create a new cell reference.
    pub fn new(row: u32, col: u32) -> Self {
        Self {
            row,
            col,
            row_absolute: false,
            col_absolute: false,
        }
    }

    /// Convert to a plain cell reference.
    pub fn to_cell(&self) -> CellRef {
        CellRef::new(self.row, self.col)
    }

    /// Translate (shift) this reference by a row/col offset.
    /// Only shifts relative components.
    pub fn translate(&self, dr: i32, dc: i32) -> Self {
        let mut out = *self;
        if !self.row_absolute {
            out.row = (self.row as i64 + dr as i64).max(0) as u32;
        }
        if !self.col_absolute {
            out.col = (self.col as i64 + dc as i64).max(0) as u32;
        }
        out
    }

    /// Render in A1 notation with `$` markers.
    pub fn to_a1(&self) -> String {
        let mut col = self.col + 1;
        let mut name = String::new();
        while col > 0 {
            let rem = (col - 1) % 26;
            name.insert(0, (b'A' + rem as u8) as char);
            col = (col - 1) / 26;
        }
        let col_s = if self.col_absolute {
            format!("${}", name)
        } else {
            name
        };
        let row_s = if self.row_absolute {
            format!("${}", self.row + 1)
        } else {
            (self.row + 1).to_string()
        };
        format!("{}{}", col_s, row_s)
    }
}

impl fmt::Display for CellReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_a1())
    }
}

/// A reference to a rectangular range of cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangeReference {
    /// Top-left cell.
    pub start: CellReference,
    /// Bottom-right cell (inclusive).
    pub end: CellReference,
}

impl RangeReference {
    /// Create a new range reference from two corners.
    pub fn new(start: CellReference, end: CellReference) -> Self {
        let s_row = start.row.min(end.row);
        let e_row = start.row.max(end.row);
        let s_col = start.col.min(end.col);
        let e_col = start.col.max(end.col);
        Self {
            start: CellReference::new(s_row, s_col),
            end: CellReference::new(e_row, e_col),
        }
    }

    /// Number of rows.
    pub fn row_count(&self) -> u32 {
        self.end.row - self.start.row + 1
    }

    /// Number of columns.
    pub fn col_count(&self) -> u32 {
        self.end.col - self.start.col + 1
    }

    /// Iterate over all cells row-major.
    pub fn cells(&self) -> impl Iterator<Item = CellReference> {
        let s = self.start;
        let e = self.end;
        (s.row..=e.row).flat_map(move |r| {
            (s.col..=e.col).map(move |c| CellReference::new(r, c))
        })
    }

    /// Render as A1 range notation.
    pub fn to_a1(&self) -> String {
        format!("{}:{}", self.start.to_a1(), self.end.to_a1())
    }
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnOp {
    Plus,
    Minus,
    Not,
}

/// A parsed formula expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal.
    Number(rust_decimal::Decimal),
    /// A text literal (the raw string without quotes).
    Text(String),
    /// A boolean literal.
    Bool(bool),
    /// A cell reference.
    CellRef(CellReference),
    /// A range reference.
    Range(RangeReference),
    /// A named range reference.
    Name(String),
    /// A function call.
    Function {
        /// Function name, upper-cased.
        name: String,
        /// Arguments.
        args: Vec<Expr>,
    },
    /// A unary operation.
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    /// A binary operation.
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    /// Collect all cell and range references in this expression.
    pub fn collect_references<'a>(&'a self, out: &mut Vec<RangeReference>) {
        match self {
            Expr::CellRef(c) => {
                out.push(RangeReference::new(*c, *c));
            }
            Expr::Range(r) => out.push(*r),
            Expr::Function { args, .. } => {
                for a in args {
                    a.collect_references(out);
                }
            }
            Expr::Unary { expr, .. } => expr.collect_references(out),
            Expr::Binary { left, right, .. } => {
                left.collect_references(out);
                right.collect_references(out);
            }
            _ => {}
        }
    }
}
