//! Formula error types and spreadsheet error values.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Kinds of errors that can arise during parsing or evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormulaErrorKind {
    /// Unexpected token in the input.
    UnexpectedToken,
    /// Unexpected end of input.
    UnexpectedEnd,
    /// Mismatched parentheses or brackets.
    MismatchedParen,
    /// Division by zero.
    DivByZero,
    /// Wrong argument count for a function.
    WrongArgCount,
    /// A value was used in an incompatible context.
    Value,
    /// A name or function was not recognized.
    Name,
    /// A number could not be parsed.
    NumberParse,
    /// A circular reference was detected during evaluation.
    CircularRef,
    /// Reference to a cell that is out of bounds.
    Ref,
    /// An unknown or internal error.
    Other,
    /// Unknown function name.
    UnknownFunction,
}

/// A formula error with a kind and optional message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormulaError {
    /// The kind of error.
    pub kind: FormulaErrorKind,
    /// A human-readable message, when available.
    pub message: Option<String>,
}

impl FormulaError {
    /// Create a new formula error.
    pub fn new(kind: FormulaErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: Some(message.into()),
        }
    }

    /// Create an error with no message.
    pub fn plain(kind: FormulaErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }

    /// The spreadsheet error code string (e.g. "#DIV/0!").
    pub fn error_code(&self) -> &'static str {
        match self.kind {
            FormulaErrorKind::DivByZero => "#DIV/0!",
            FormulaErrorKind::Value => "#VALUE!",
            FormulaErrorKind::Name => "#NAME?",
            FormulaErrorKind::WrongArgCount => "#N/A",
            FormulaErrorKind::CircularRef => "#CIRC!",
            FormulaErrorKind::Ref => "#REF!",
            FormulaErrorKind::NumberParse => "#NUM!",
            _ => "#ERROR!",
        }
    }
}

impl fmt::Display for FormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = &self.message {
            write!(f, "{}: {}", self.error_code(), msg)
        } else {
            write!(f, "{}", self.error_code())
        }
    }
}

impl std::error::Error for FormulaError {}
