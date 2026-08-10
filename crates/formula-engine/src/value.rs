//! Spreadsheet value type.

use chrono::{NaiveDate, NaiveDateTime, Timelike, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::{FormulaError, FormulaErrorKind};

/// A value in a spreadsheet cell or computed by the formula engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Empty,
    Number(Decimal),
    Text(String),
    Bool(bool),
    Error(FormulaError),
    DateTime(NaiveDateTime),
    Array(Vec<Vec<Value>>),
}

impl Value {
    pub fn empty() -> Self {
        Value::Empty
    }

    pub fn number(v: impl Into<Decimal>) -> Self {
        Value::Number(v.into())
    }

    pub fn text(s: impl Into<String>) -> Self {
        Value::Text(s.into())
    }

    pub fn boolean(b: bool) -> Self {
        Value::Bool(b)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn as_number(&self) -> Option<Decimal> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Text(s) => s.trim().parse().ok(),
            Value::Bool(b) => Some(if *b { Decimal::ONE } else { Decimal::ZERO }),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match self {
            Value::Text(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(if *b { "TRUE".into() } else { "FALSE".into() }),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            Value::Number(n) => Some(*n != Decimal::ZERO),
            Value::Text(s) => {
                let t = s.trim().to_lowercase();
                if t == "true" { Some(true) } else if t == "false" { Some(false) } else { None }
            }
            _ => None,
        }
    }

    pub fn display_string(&self) -> String {
        match self {
            Value::Empty => String::new(),
            Value::Number(n) => format!("{}", n),
            Value::Text(s) => s.clone(),
            Value::Bool(b) => if *b { "TRUE".into() } else { "FALSE".into() },
            Value::Error(e) => e.error_code().to_string(),
            Value::DateTime(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            Value::Array(rows) => rows
                .iter()
                .map(|row| row.iter().map(|v| v.display_string()).collect::<Vec<_>>().join(", "))
                .collect::<Vec<_>>()
                .join("; "),
        }
    }

    pub fn to_excel_serial(&self) -> Option<Decimal> {
        match self {
            Value::DateTime(dt) => {
                let epoch = NaiveDate::from_ymd_opt(1899, 12, 30)?;
                let days = dt.date().signed_duration_since(epoch).num_days();
                let secs = dt.time().num_seconds_from_midnight() as i64;
                let frac = Decimal::from(secs) / Decimal::from(86_400);
                Some(Decimal::from(days) + frac)
            }
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Empty => "Empty",
            Value::Number(_) => "Number",
            Value::Text(_) => "Text",
            Value::Bool(_) => "Boolean",
            Value::Error(_) => "Error",
            Value::DateTime(_) => "DateTime",
            Value::Array(_) => "Array",
        }
    }

    pub fn value_err(msg: impl Into<String>) -> Self {
        Value::Error(FormulaError::new(FormulaErrorKind::Value, msg))
    }
}

impl Value {
    pub fn from_unix(secs: i64) -> Self {
        Value::DateTime(Utc.timestamp_opt(secs, 0).unwrap().naive_utc())
    }
}