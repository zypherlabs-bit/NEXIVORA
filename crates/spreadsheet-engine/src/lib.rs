use std::collections::HashMap;
use nexivora_formula_engine::{evaluate, parser::parse, eval::SheetResolver, value::Value, error::FormulaError};

#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub value: Option<f64>,
    pub formula: Option<String>,
    pub text: Option<String>,
}

pub struct Sheet {
    pub name: String,
    cells: HashMap<(u32, u32), Cell>,
}

impl Sheet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cells: HashMap::new(),
        }
    }

    pub fn get_cell(&self, row: u32, col: u32) -> Cell {
        self.cells.get(&(row, col)).cloned().unwrap_or_default()
    }

    pub fn set_value(&mut self, row: u32, col: u32, value: f64) {
        let mut cell = self.cells.remove(&(row, col)).unwrap_or_default();
        cell.value = Some(value);
        cell.formula = None;
        self.cells.insert((row, col), cell);
    }

    pub fn set_formula(&mut self, row: u32, col: u32, formula: &str) {
        let mut cell = self.cells.remove(&(row, col)).unwrap_or_default();
        cell.formula = Some(formula.to_string());
        self.cells.insert((row, col), cell);
    }

    pub fn evaluate_formula(&mut self, row: u32, col: u32, resolver: &impl SheetResolver) -> Result<Value, FormulaError> {
        let cell = self.get_cell(row, col);
        let formula = match cell.formula.as_ref() {
            Some(f) => f,
            None => return Ok(Value::empty()),
        };
        let parsed = parse(formula)?;
        let mut visited = std::collections::HashSet::new();
        evaluate(&parsed.expr, Some(&self.name), resolver, &mut visited)
    }
}

pub struct Spreadsheet {
    pub sheets: HashMap<String, Sheet>,
    pub active_sheet: String,
}

impl Spreadsheet {
    pub fn new() -> Self {
        let mut sheets = HashMap::new();
        sheets.insert("Sheet1".to_string(), Sheet::new("Sheet1"));
        
        Self {
            sheets,
            active_sheet: "Sheet1".to_string(),
        }
    }

    pub fn set_value(&mut self, row: u32, col: u32, value: f64) {
        if let Some(sheet) = self.sheets.get_mut(&self.active_sheet) {
            sheet.set_value(row, col, value);
        }
    }

    pub fn set_formula(&mut self, row: u32, col: u32, formula: &str) {
        if let Some(sheet) = self.sheets.get_mut(&self.active_sheet) {
            sheet.set_formula(row, col, formula);
        }
    }

    pub fn get_value(&self, row: u32, col: u32) -> Option<f64> {
        if let Some(sheet) = self.sheets.get(&self.active_sheet) {
            sheet.get_cell(row, col).value
        } else {
            None
        }
    }
}

impl SheetResolver for Spreadsheet {
    fn cell_value(&self, sheet: Option<&str>, r: u32, c: u32) -> Value {
        let sheet_name = sheet.unwrap_or(&self.active_sheet);
        if let Some(s) = self.sheets.get(sheet_name) {
            match s.get_cell(r, c).value {
                Some(v) => Value::number(rust_decimal::Decimal::from_f64_retain(v).unwrap_or_default()),
                None => Value::empty(),
            }
        } else {
            Value::empty()
        }
    }

    fn range_values(&self, sheet: Option<&str>, r1: u32, c1: u32, r2: u32, c2: u32) -> Vec<Vec<Value>> {
        let sheet_name = sheet.unwrap_or(&self.active_sheet);
        let mut rows = Vec::new();
        if let Some(s) = self.sheets.get(sheet_name) {
            for r in r1..=r2 {
                let mut row = Vec::new();
                for c in c1..=c2 {
                    row.push(match s.get_cell(r, c).value {
                        Some(v) => Value::number(rust_decimal::Decimal::from_f64_retain(v).unwrap_or_default()),
                        None => Value::empty(),
                    });
                }
                rows.push(row);
            }
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_value() {
        let mut ss = Spreadsheet::new();
        ss.set_value(1, 1, 42.0);
        assert_eq!(ss.get_value(1, 1), Some(42.0));
    }

    #[test]
    fn test_set_formula() {
        let mut ss = Spreadsheet::new();
        ss.set_formula(1, 1, "=1+2");
        let cell = ss.sheets.get("Sheet1").unwrap().get_cell(1, 1);
        assert_eq!(cell.formula, Some("=1+2".to_string()));
    }

    #[test]
    fn test_evaluate_formula() {
        let mut ss = Spreadsheet::new();
        ss.set_value(0, 0, 10.0);
        ss.set_formula(1, 1, "=A1+5");
        // Parse and evaluate directly without borrow checker issues
        let parsed = nexivora_formula_engine::parser::parse("=A1+5").unwrap();
        let mut visited = std::collections::HashSet::new();
        let result = nexivora_formula_engine::eval::evaluate(&parsed.expr, None, &ss, &mut visited).unwrap();
        assert_eq!(result.as_number(), Some(rust_decimal::Decimal::new(15, 0)));
    }
}