//! Expression evaluator with dependency tracking.

use std::collections::HashSet;

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::ast::{BinOp, Expr};
use crate::error::{FormulaError, FormulaErrorKind};
use crate::functions::FunctionRegistry;
use crate::value::Value;

/// A trait that abstracts access to cell values during evaluation.
pub trait SheetResolver {
    /// Fetch the value of a single cell.
    fn cell_value(&self, sheet: Option<&str>, r: u32, c: u32) -> Value;

    /// Evaluate a range, returning a 2-D vector of values.
    fn range_values(
        &self,
        sheet: Option<&str>,
        r1: u32,
        c1: u32,
        r2: u32,
        c2: u32,
    ) -> Vec<Vec<Value>>;
}

/// Evaluate an expression against a sheet resolver.
///
/// Detects circular references by tracking the chain of visited cells.
pub fn evaluate(
    expr: &Expr,
    sheet: Option<&str>,
    resolver: &impl SheetResolver,
    visited: &mut HashSet<(Option<String>, u32, u32)>,
) -> Result<Value, FormulaError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Text(s) => Ok(Value::Text(s.clone())),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::CellRef(cell) => {
            let key = (
                sheet.map(|s| s.to_string()),
                cell.row,
                cell.col,
            );
            if visited.contains(&key) {
                return Err(FormulaError::plain(FormulaErrorKind::CircularRef));
            }
            visited.insert(key.clone());
            let val = resolver.cell_value(sheet, cell.row, cell.col);
            visited.remove(&key);
            Ok(val)
        }
        Expr::Range(range) => {
            let values = resolver.range_values(
                sheet,
                range.start.row,
                range.start.col,
                range.end.row,
                range.end.col,
            );
            Ok(Value::Array(values))
        }
        Expr::Name(_name) => {
            // Named ranges are not supported in the minimal evaluator.
            Err(FormulaError::new(
                FormulaErrorKind::Name,
                "named ranges are not supported",
            ))
        }
        Expr::Function { name, args } => {
            let mut evaled = Vec::new();
            for a in args {
                evaled.push(evaluate(a, sheet, resolver, visited)?);
            }
            FunctionRegistry::call(name, &evaled)
        }
        Expr::Unary { op, expr } => {
            let v = evaluate(expr, sheet, resolver, visited)?;
            match op {
                crate::ast::UnOp::Plus => Ok(v),
                crate::ast::UnOp::Minus => match v.as_number() {
                    Some(n) => Ok(Value::Number(-n)),
                    None => Err(FormulaError::new(
                        FormulaErrorKind::Value,
                        "unary minus requires a number",
                    )),
                },
                crate::ast::UnOp::Not => Ok(Value::Bool(!v.as_bool().unwrap_or(false))),
            }
        }
        Expr::Binary { op, left, right } => {
            let l = evaluate(left, sheet, resolver, visited)?;
            let r = evaluate(right, sheet, resolver, visited)?;
            match op {
                BinOp::Add => apply_number_op(l, r, |a, b| Ok(Value::Number(a + b))),
                BinOp::Sub => apply_number_op(l, r, |a, b| Ok(Value::Number(a - b))),
                BinOp::Mul => apply_number_op(l, r, |a, b| Ok(Value::Number(a * b))),
                BinOp::Div => apply_number_op(l, r, |a, b| {
                    if b == Decimal::ZERO {
                        return Err(FormulaError::plain(FormulaErrorKind::DivByZero));
                    }
                    Ok(Value::Number(a / b))
                }),
                BinOp::Pow => apply_number_op(l, r, |a, b| {
                    let r = a.to_f64().unwrap_or(0.0).powf(b.to_f64().unwrap_or(0.0));
                    Decimal::from_f64_retain(r)
                        .map(Value::Number)
                        .ok_or_else(|| FormulaError::new(FormulaErrorKind::NumberParse, "overflow"))
                }),
                BinOp::Concat => Ok(Value::Text(format!(
                    "{}{}",
                    l.as_text().unwrap_or_default(),
                    r.as_text().unwrap_or_default()
                ))),
                BinOp::Eq => Ok(Value::Bool(l == r)),
                BinOp::Ne => Ok(Value::Bool(l != r)),
                BinOp::Lt => apply_number_cmp(l, r, |a, b| a < b),
                BinOp::Le => apply_number_cmp(l, r, |a, b| a <= b),
                BinOp::Gt => apply_number_cmp(l, r, |a, b| a > b),
                BinOp::Ge => apply_number_cmp(l, r, |a, b| a >= b),
                BinOp::And => {
                    let lb = l.as_bool().unwrap_or(false);
                    let rb = r.as_bool().unwrap_or(false);
                    Ok(Value::Bool(lb && rb))
                }
                BinOp::Or => {
                    let lb = l.as_bool().unwrap_or(false);
                    let rb = r.as_bool().unwrap_or(false);
                    Ok(Value::Bool(lb || rb))
                }
            }
        }
    }
}

fn apply_number_op<F>(l: Value, r: Value, f: F) -> Result<Value, FormulaError>
where
    F: FnOnce(Decimal, Decimal) -> Result<Value, FormulaError>,
{
    let a = l.as_number().ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "expected number"))?;
    let b = r.as_number().ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "expected number"))?;
    f(a, b)
}

fn apply_number_cmp<F>(l: Value, r: Value, f: F) -> Result<Value, FormulaError>
where
    F: Fn(Decimal, Decimal) -> bool,
{
    let a = l.as_number().ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "expected number"))?;
    let b = r.as_number().ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "expected number"))?;
    Ok(Value::Bool(f(a, b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeResolver;

    impl SheetResolver for FakeResolver {
        fn cell_value(&self, _sheet: Option<&str>, r: u32, c: u32) -> Value {
            Value::number(100 + r * 10 + c)
        }

        fn range_values(
            &self,
            _sheet: Option<&str>,
            r1: u32,
            c1: u32,
            r2: u32,
            c2: u32,
        ) -> Vec<Vec<Value>> {
            let mut rows = Vec::new();
            for r in r1..=r2 {
                let mut row = Vec::new();
                for c in c1..=c2 {
                    row.push(Value::number(100 + r * 10 + c));
                }
                rows.push(row);
            }
            rows
        }
    }

    #[test]
    fn eval_arithmetic() {
        let f = crate::parser::parse("=1+2*3").unwrap();
        let mut visited = HashSet::new();
        let v = evaluate(&f.expr, None, &FakeResolver, &mut visited).unwrap();
        assert_eq!(v.as_number(), Some(Decimal::new(7, 0)));
    }

    #[test]
    fn eval_cell_ref() {
        let f = crate::parser::parse("=A1").unwrap();
        let mut visited = HashSet::new();
        let v = evaluate(&f.expr, None, &FakeResolver, &mut visited).unwrap();
        // FakeResolver returns 100 + 0*10 + 0 = 100
        assert_eq!(v.as_number(), Some(Decimal::new(100, 0)));
    }

        #[test]
    fn eval_function() {
        let f = crate::parser::parse("=SUM(A1:A3)").unwrap();
        let mut visited = HashSet::new();
        let v = evaluate(&f.expr, None, &FakeResolver, &mut visited).unwrap();
        // A1=100, A2=110, A3=120, so sum should be 330
        assert_eq!(v.as_number(), Some(Decimal::new(330, 0)));
    }
}