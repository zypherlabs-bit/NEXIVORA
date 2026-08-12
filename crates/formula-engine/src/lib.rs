//! Nexivora Formula Engine
//!
//! Provides spreadsheet-style formula parsing and evaluation.
//!
//! This crate re-exports the public API from the submodules.

pub mod ast;
pub mod error;
pub mod eval;
pub mod functions;
pub mod lexer;
pub mod parser;
pub mod value;

pub use ast::{BinOp, CellReference, Expr, RangeReference, UnOp};
pub use error::{FormulaError, FormulaErrorKind};
pub use eval::{evaluate, SheetResolver};
pub use parser::parse;
