//! Built-in spreadsheet function catalog.

use std::collections::HashMap;
use std::sync::OnceLock;

use chrono::{Datelike, Timelike};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::error::{FormulaError, FormulaErrorKind};
use crate::value::Value;

/// Describes a function's arity constraints.
#[derive(Debug, Clone)]
pub struct FunctionSpec {
    /// Canonical upper-case name.
    pub name: String,
    /// Minimum number of arguments.
    pub min_args: usize,
    /// Maximum number of arguments (usize::MAX for variadic).
    pub max_args: usize,
    /// Category label for UI grouping.
    pub category: &'static str,
}

/// A registry of all built-in functions.
pub struct FunctionRegistry {
    specs: HashMap<String, FunctionSpec>,
    handlers: HashMap<String, fn(&[Value]) -> Result<Value, FormulaError>>,
}

static REGISTRY: OnceLock<FunctionRegistry> = OnceLock::new();

impl FunctionRegistry {
    fn global() -> &'static FunctionRegistry {
        REGISTRY.get_or_init(FunctionRegistry::build)
    }

    fn build() -> Self {
        let mut reg = Self {
            specs: HashMap::new(),
            handlers: HashMap::new(),
        };
        reg.register_defaults();
        reg
    }

    fn register_defaults(&mut self) {
        let mut register = |name: &str, min: usize, max: usize, cat: &'static str, handler: fn(&[Value]) -> Result<Value, FormulaError>| {
            self.specs.insert(name.to_string(), FunctionSpec { name: name.to_string(), min_args: min, max_args: max, category: cat });
            self.handlers.insert(name.to_string(), handler);
        };

        fn sum(args: &[Value]) -> Result<Value, FormulaError> {
            let mut sum = Decimal::ZERO;
            for v in args {
                match v {
                    Value::Number(n) => sum += *n,
                    Value::Array(arr) => {
                        for row in arr {
                            for cell in row {
                                if let Value::Number(n) = cell {
                                    sum += *n;
                                }
                            }
                        }
                    }
                    _ => return Err(FormulaError::new(FormulaErrorKind::Value, "expected number or range")),
                }
            }
            Ok(Value::number(sum))
        }
        register("SUM", 1, usize::MAX, "Math", sum);

        fn average(args: &[Value]) -> Result<Value, FormulaError> {
            let mut sum = Decimal::ZERO;
            for v in args { sum += need_number(v)?; }
            let count = args.len() as u32;
            Ok(Value::number(sum / Decimal::from(count)))
        }
        register("AVERAGE", 1, usize::MAX, "Math", average);

        fn min(args: &[Value]) -> Result<Value, FormulaError> {
            let mut min = need_number(&args[0])?;
            for v in args.iter().skip(1) {
                let n = need_number(v)?;
                if n < min { min = n; }
            }
            Ok(Value::number(min))
        }
        register("MIN", 1, usize::MAX, "Math", min);

        fn max(args: &[Value]) -> Result<Value, FormulaError> {
            let mut max = need_number(&args[0])?;
            for v in args.iter().skip(1) {
                let n = need_number(v)?;
                if n > max { max = n; }
            }
            Ok(Value::number(max))
        }
        register("MAX", 1, usize::MAX, "Math", max);

        fn count(args: &[Value]) -> Result<Value, FormulaError> {
            let mut count = 0usize;
            for v in args {
                if v.is_number() || matches!(v, Value::Text(_) if !v.as_text().unwrap_or_default().is_empty()) { count += 1; }
            }
            Ok(Value::number(Decimal::from(count)))
        }
        register("COUNT", 1, usize::MAX, "Statistical", count);

        fn abs(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::number(need_number(&args[0])?.abs()))
        }
        register("ABS", 1, 1, "Math", abs);

        fn round(args: &[Value]) -> Result<Value, FormulaError> {
            let n = need_number(&args[0])?;
            let digits = need_number(&args[1])?.to_i64().unwrap_or(0);
            let factor = Decimal::from(10i64.saturating_pow(digits as u32));
            Ok(Value::number((n * factor).round() / factor))
        }
        register("ROUND", 2, 2, "Math", round);

        fn if_fn(args: &[Value]) -> Result<Value, FormulaError> {
            let cond = need_boolean(&args[0])?;
            Ok(if cond { args[1].clone() } else { args[2].clone() })
        }
        register("IF", 3, 3, "Logical", if_fn);

        fn and(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(args.iter().all(|v| need_boolean(v).unwrap_or(false))))
        }
        register("AND", 2, usize::MAX, "Logical", and);

        fn or(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(args.iter().any(|v| need_boolean(v).unwrap_or(false))))
        }
        register("OR", 2, usize::MAX, "Logical", or);

        fn not(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(!need_boolean(&args[0])?))
        }
        register("NOT", 1, 1, "Logical", not);

        fn text(args: &[Value]) -> Result<Value, FormulaError> {
            let value = need_number(&args[0])?;
            let _fmt = need_text(&args[1])?;
            Ok(Value::text(format!("{value}")))
        }
        register("TEXT", 2, 2, "Text", text);

        fn len(args: &[Value]) -> Result<Value, FormulaError> {
            let s = need_text(&args[0])?;
            Ok(Value::number(Decimal::from(s.len() as u64)))
        }
        register("LEN", 1, 1, "Text", len);

        fn upper(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::text(need_text(&args[0])?.to_uppercase()))
        }
        register("UPPER", 1, 1, "Text", upper);

        fn lower(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::text(need_text(&args[0])?.to_lowercase()))
        }
        register("LOWER", 1, 1, "Text", lower);

        fn concat(args: &[Value]) -> Result<Value, FormulaError> {
            let mut out = String::new();
            for v in args { out.push_str(need_text(v)?); }
            Ok(Value::text(out))
        }
        register("CONCAT", 1, usize::MAX, "Text", concat);

        fn trim(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::text(need_text(&args[0])?.trim().to_string()))
        }
        register("TRIM", 1, 1, "Text", trim);

        fn left(args: &[Value]) -> Result<Value, FormulaError> {
            let s = need_text(&args[0])?;
            let n = need_number(&args[1])?.to_usize().unwrap_or(0);
            Ok(Value::text(s.chars().take(n).collect::<String>()))
        }
        register("LEFT", 2, 2, "Text", left);

        fn right(args: &[Value]) -> Result<Value, FormulaError> {
            let s = need_text(&args[0])?;
            let n = need_number(&args[1])?.to_usize().unwrap_or(0);
            let len = s.chars().count();
            Ok(Value::text(s.chars().skip(len.saturating_sub(n)).collect::<String>()))
        }
        register("RIGHT", 2, 2, "Text", right);

        fn mid(args: &[Value]) -> Result<Value, FormulaError> {
            let s = need_text(&args[0])?;
            let start = need_number(&args[1])?.to_usize().unwrap_or(1);
            let count = need_number(&args[2])?.to_usize().unwrap_or(0);
            let chars: Vec<char> = s.chars().collect();
            let start_idx = start.saturating_sub(1);
            let end_idx = start_idx + count;
            Ok(Value::text(chars.get(start_idx..end_idx).unwrap_or(&[]).iter().collect::<String>()))
        }
        register("MID", 3, 3, "Text", mid);

        fn find(args: &[Value]) -> Result<Value, FormulaError> {
            let find_text = need_text(&args[0])?;
            let within_text = need_text(&args[1])?;
            let start = if args.len() == 3 { need_number(&args[2])?.to_usize().unwrap_or(1) } else { 1 };
            let chars: Vec<char> = within_text.chars().collect();
            let start_idx = start.saturating_sub(1);
            for (i, window) in chars.windows(find_text.chars().count()).enumerate() {
                if i < start_idx { continue; }
                if window.iter().collect::<String>() == find_text {
                    return Ok(Value::number(Decimal::from(i as u64 + 1)));
                }
            }
            Ok(Value::number(Decimal::ZERO))
        }
        register("FIND", 2, 3, "Text", find);

        fn substitute(args: &[Value]) -> Result<Value, FormulaError> {
            let text = need_text(&args[0])?;
            let old_text = need_text(&args[1])?;
            let new_text = need_text(&args[2])?;
            let instance = if args.len() == 4 { need_number(&args[3])?.to_usize() } else { None };
            let mut result = text.to_string();
            let mut count = 0usize;
            if old_text.is_empty() {
                return Ok(Value::text(text));
            }
            let mut start = 0usize;
            loop {
                if let Some((idx, _)) = result[start..].match_indices(old_text).next() {
                    count += 1;
                    if instance.map(|i| count == i).unwrap_or(true) {
                        let absolute_idx = start + idx;
                        result.replace_range(absolute_idx..absolute_idx + old_text.len(), new_text);
                        start = absolute_idx + new_text.len();
                    } else {
                        start += idx + old_text.len();
                    }
                } else { break; }
            }
            Ok(Value::text(result))
        }
        register("SUBSTITUTE", 3, 4, "Text", substitute);

        fn replace(args: &[Value]) -> Result<Value, FormulaError> {
            let text = need_text(&args[0])?;
            let start = need_number(&args[1])?.to_usize().unwrap_or(1);
            let count = need_number(&args[2])?.to_usize().unwrap_or(0);
            let new_text = need_text(&args[3])?;
            let chars: Vec<char> = text.chars().collect();
            let start_idx = start.saturating_sub(1);
            let end_idx = start_idx + count;
            let mut result: String = chars[..start_idx].iter().collect();
            result.push_str(new_text);
            if let Some(slice) = chars.get(end_idx..) {
                for &c in slice {
                    result.push(c);
                }
            }
            Ok(Value::text(result))
        }
        register("REPLACE", 4, 4, "Text", replace);

        fn value(args: &[Value]) -> Result<Value, FormulaError> {
            let s = need_text(&args[0])?;
            match s.trim().parse::<Decimal>() {
                Ok(d) => Ok(Value::number(d)),
                Err(_) => Err(FormulaError::new(FormulaErrorKind::Value, "text cannot be parsed as number"))
            }
        }
        register("VALUE", 1, 1, "Text", value);

        fn is_number(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(args[0].is_number()))
        }
        register("ISNUMBER", 1, 1, "Information", is_number);

        fn is_text(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(matches!(args[0], Value::Text(_))))
        }
        register("ISTEXT", 1, 1, "Information", is_text);

        fn is_blank(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(matches!(args[0], Value::Empty)))
        }
        register("ISBLANK", 1, 1, "Information", is_blank);

        fn is_error(args: &[Value]) -> Result<Value, FormulaError> {
            Ok(Value::boolean(matches!(args[0], Value::Error(_))))
        }
        register("ISERROR", 1, 1, "Information", is_error);

        fn iferror(args: &[Value]) -> Result<Value, FormulaError> {
            match &args[0] {
                Value::Error(_) => Ok(args[1].clone()),
                other => Ok(other.clone()),
            }
        }
        register("IFERROR", 2, 2, "Logical", iferror);

        fn date(args: &[Value]) -> Result<Value, FormulaError> {
            let year = need_number(&args[0])?.to_i64().unwrap_or(1900);
            let month = need_number(&args[1])?.to_i64().unwrap_or(1);
            let day = need_number(&args[2])?.to_i64().unwrap_or(1);
            let y = if year < 1900 { year + 1900 } else { year };
            let m = month.max(1).min(12) as u32;
            let d = day.max(1).min(31) as u32;
            let date = chrono::NaiveDate::from_ymd_opt(y as i32, m, d)
                .ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "invalid date"))?;
            Ok(Value::DateTime(date.and_hms_opt(0, 0, 0).unwrap()))
        }
        register("DATE", 3, 3, "Date", date);

        fn time(args: &[Value]) -> Result<Value, FormulaError> {
            let hour = need_number(&args[0])?.to_i64().unwrap_or(0);
            let minute = need_number(&args[1])?.to_i64().unwrap_or(0);
            let second = need_number(&args[2])?.to_i64().unwrap_or(0);
            let h = hour.rem_euclid(24) as u32;
            let m = minute.rem_euclid(60) as u32;
            let s = second.rem_euclid(60) as u32;
            let time = chrono::NaiveTime::from_hms_opt(h, m, s).unwrap();
            Ok(Value::DateTime(chrono::NaiveDate::from_ymd_opt(1899, 12, 30).unwrap().and_time(time)))
        }
        register("TIME", 3, 3, "Date", time);

        fn year(args: &[Value]) -> Result<Value, FormulaError> {
            let dt = as_datetime(&args[0])?;
            Ok(Value::number(Decimal::from(dt.year() as i64 + 1900)))
        }
        register("YEAR", 1, 1, "Date", year);

        fn month(args: &[Value]) -> Result<Value, FormulaError> {
            let dt = as_datetime(&args[0])?;
            Ok(Value::number(Decimal::from(dt.month() as i64)))
        }
        register("MONTH", 1, 1, "Date", month);

        fn day(args: &[Value]) -> Result<Value, FormulaError> {
            let dt = as_datetime(&args[0])?;
            Ok(Value::number(Decimal::from(dt.day() as i64)))
        }
        register("DAY", 1, 1, "Date", day);

        fn hour(args: &[Value]) -> Result<Value, FormulaError> {
            let dt = as_datetime(&args[0])?;
            Ok(Value::number(Decimal::from(dt.hour() as i64)))
        }
        register("HOUR", 1, 1, "Date", hour);

        fn minute(args: &[Value]) -> Result<Value, FormulaError> {
            let dt = as_datetime(&args[0])?;
            Ok(Value::number(Decimal::from(dt.minute() as i64)))
        }
        register("MINUTE", 1, 1, "Date", minute);

        fn second(args: &[Value]) -> Result<Value, FormulaError> {
            let dt = as_datetime(&args[0])?;
            Ok(Value::number(Decimal::from(dt.second() as i64)))
        }
        register("SECOND", 1, 1, "Date", second);

        fn days(args: &[Value]) -> Result<Value, FormulaError> {
            let a = as_datetime(&args[0])?;
            let b = as_datetime(&args[1])?;
            let diff = b.signed_duration_since(a).num_days();
            Ok(Value::number(Decimal::from(diff)))
        }
        register("DAYS", 2, 2, "Date", days);

        fn edate(args: &[Value]) -> Result<Value, FormulaError> {
            let start = as_datetime(&args[0])?;
            let months = need_number(&args[1])?.to_i64().unwrap_or(0);
            let mut y = start.year() as i32;
            let mut m = start.month() as i32;
            let total = m as i64 + months - 1;
            y += total.div_euclid(12) as i32;
            m = total.rem_euclid(12) as i32 + 1;
            if m <= 0 { y -= 1; m += 12; }
            let max_day = chrono::NaiveDate::from_ymd_opt(y, m as u32, 1).map(|d| d.succ_opt().unwrap().pred_opt().unwrap().day()).unwrap_or(30);
            let d = start.day().min(max_day);
            let out = chrono::NaiveDate::from_ymd_opt(y, m as u32, d).expect("valid date").and_time(start.time());
            Ok(Value::DateTime(out))
        }
        register("EDATE", 2, 2, "Date", edate);

        fn eomonth(args: &[Value]) -> Result<Value, FormulaError> {
            let months = if args.len() == 1 { 1 } else { need_number(&args[1])?.to_i64().unwrap_or(1) };
            let start = as_datetime(&args[0])?;
            let mut y = start.year() as i32 + (months.div_euclid(12)) as i32;
            let mut m = (start.month() as i64 + months.rem_euclid(12)) as i32;
            if m <= 0 { y -= 1; m += 12; }
            let max_day = chrono::NaiveDate::from_ymd_opt(y, m as u32, 1).map(|d| d.succ_opt().unwrap().pred_opt().unwrap().day()).unwrap_or(30);
            Ok(Value::DateTime(chrono::NaiveDate::from_ymd_opt(y, m as u32, max_day).expect("valid date").and_time(start.time())))
        }
        register("EOMONTH", 1, 2, "Date", eomonth);
    }

    pub fn call(name: &str, args: &[Value]) -> Result<Value, FormulaError> {
        let reg = FunctionRegistry::global();
        let handler = reg.handlers.get(name).ok_or_else(|| FormulaError::new(FormulaErrorKind::UnknownFunction, name.to_string()))?;
        handler(args)
    }
}

fn as_datetime(v: &Value) -> Result<chrono::NaiveDateTime, FormulaError> {
    match v {
        Value::DateTime(dt) => Ok(*dt),
        _ => Err(FormulaError::new(FormulaErrorKind::Value, "expected datetime value")),
    }
}

fn need_number(v: &Value) -> Result<Decimal, FormulaError> {
    v.as_number().ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "expected number"))
}

fn need_text(v: &Value) -> Result<&str, FormulaError> {
    match v {
        Value::Text(s) => Ok(s),
        _ => Err(FormulaError::new(FormulaErrorKind::Value, "expected text")),
    }
}

fn need_boolean(v: &Value) -> Result<bool, FormulaError> {
    v.as_bool().ok_or_else(|| FormulaError::new(FormulaErrorKind::Value, "expected boolean"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use std::collections::HashSet;

    struct BlankResolver;
    impl crate::eval::SheetResolver for BlankResolver {
        fn cell_value(&self, _sheet: Option<&str>, _r: u32, _c: u32) -> Value { Value::empty() }
        fn range_values(&self, _sheet: Option<&str>, _r1: u32, _c1: u32, _r2: u32, _c2: u32) -> Vec<Vec<Value>> { Vec::new() }
    }

    fn eval(formula: &str) -> Result<Value, FormulaError> {
        let parsed = parse(formula)?;
        crate::eval::evaluate(&parsed.expr, None, &BlankResolver, &mut HashSet::new())
    }

    #[test]
    fn test_sum() { if let Ok(v) = eval("=SUM(1,2,3)") { assert_eq!(v, Value::number(6)); } }
}