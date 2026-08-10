// Simple spreadsheet example demonstrating Nexivora's formula engine

use nexivora_spreadsheet_engine::Spreadsheet;
use nexivora_formula_engine::{parser::parse, eval::evaluate};

fn main() {
    println!("Nexivora Simple Spreadsheet Example");
    println!("====================================\n");

    // Create a new spreadsheet
    let mut spreadsheet = Spreadsheet::new();

    // Set some values
    spreadsheet.set_value(0, 0, 10.0);  // A1 = 10
    spreadsheet.set_value(1, 0, 20.0);  // A2 = 20
    spreadsheet.set_value(2, 0, 30.0);  // A3 = 30

    // Set a formula
    spreadsheet.set_formula(3, 0, "=SUM(A1:A3)");  // A4 = SUM(A1:A3)

    // Display the values
    println!("Cell values:");
    println!("A1: {}", spreadsheet.get_value(0, 0).unwrap_or(0.0));
    println!("A2: {}", spreadsheet.get_value(1, 0).unwrap_or(0.0));
    println!("A3: {}", spreadsheet.get_value(2, 0).unwrap_or(0.0));

    // Evaluate the formula
    let formula = spreadsheet.sheets.get("Sheet1").unwrap().get_cell(3, 0).formula.unwrap();
    println!("\nFormula in A4: {}", formula);

    // Parse and evaluate the formula
    let parsed = parse(&formula).expect("Failed to parse formula");
    let mut visited = std::collections::HashSet::new();
    let result = evaluate(&parsed.expr, Some("Sheet1"), &spreadsheet, &mut visited).expect("Failed to evaluate formula");

    println!("Result: {}", result);

    // Demonstrate more complex formulas
    println!("\nComplex formula examples:");
    
    // Set more values for complex calculations
    spreadsheet.set_value(0, 1, 5.0);   // B1 = 5
    spreadsheet.set_value(1, 1, 15.0);  // B2 = 15
    
    // Average formula
    let avg_formula = "=AVERAGE(A1:B2)";
    println!("Formula: {}", avg_formula);
    let avg_parsed = parse(avg_formula).expect("Failed to parse average formula");
    let avg_result = evaluate(&avg_parsed.expr, Some("Sheet1"), &spreadsheet, &mut visited).expect("Failed to evaluate average");
    println!("Average result: {}", avg_result);
    
    // Conditional formula
    let if_formula = "=IF(A1>B1, "A1 is larger", "B1 is larger or equal")";
    println!("\nFormula: {}", if_formula);
    let if_parsed = parse(if_formula).expect("Failed to parse IF formula");
    let if_result = evaluate(&if_parsed.expr, Some("Sheet1"), &spreadsheet, &mut visited).expect("Failed to evaluate IF");
    println!("IF result: {}", if_result);

    println!("\nExample completed successfully!");
}