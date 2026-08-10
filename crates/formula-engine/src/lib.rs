//! Nexivora Formula Engine
//!
//! Provides spreadsheet-style formula parsing and evaluation.
//!
//! This crate re-exports the public API from the submodules.

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod eval;
pub mod functions;
pub mod value;

pub use parser::parse;
pub use eval::{evaluate, SheetResolver};
pub use error::{FormulaError, FormulaErrorKind};
pub use ast::{Expr, BinOp, UnOp, CellReference, RangeReference};
