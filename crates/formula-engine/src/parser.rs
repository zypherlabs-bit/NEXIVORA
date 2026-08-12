//! Recursive-descent parser for spreadsheet formulas.

use regex::Regex;
use rust_decimal::Decimal;

use crate::ast::{BinOp, CellReference, Expr, RangeReference, UnOp};
use crate::error::{FormulaError, FormulaErrorKind};
use crate::lexer::{tokenize, Token, TokenKind};

/// A parsed formula: the original source and the root expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFormula {
    /// The original formula text (without leading `=`).
    pub source: String,
    /// The root expression.
    pub expr: Expr,
}

/// Parse a formula string. If it begins with `=`, the `=` is stripped.
/// Otherwise it is treated as a bare expression (for unit tests and
/// expression evaluation).
pub fn parse(input: &str) -> Result<ParsedFormula, FormulaError> {
    let src = input.trim();
    let body = src.strip_prefix('=').unwrap_or(src).trim();
    let tokens = tokenize(body)?;
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse_expression(0)?;
    if !parser.at_end() {
        return Err(FormulaError::new(
            FormulaErrorKind::UnexpectedToken,
            "unexpected trailing tokens",
        ));
    }
    Ok(ParsedFormula {
        source: body.to_string(),
        expr,
    })
}

/// Operator precedence, higher binds tighter.
fn precedence(kind: &TokenKind) -> Option<u8> {
    match kind {
        TokenKind::Eq
        | TokenKind::Ne
        | TokenKind::Lt
        | TokenKind::Le
        | TokenKind::Gt
        | TokenKind::Ge => Some(1),
        TokenKind::Amp => Some(2),
        TokenKind::Plus | TokenKind::Minus => Some(3),
        TokenKind::Star | TokenKind::Slash => Some(4),
        TokenKind::Caret => Some(5),
        _ => None,
    }
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    _cell_re: std::cell::OnceCell<Regex>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
            _cell_re: std::cell::OnceCell::new(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<Token, FormulaError> {
        match self.advance() {
            Some(t) if std::mem::discriminant(&t.kind) == std::mem::discriminant(kind) => {
                Ok(t.clone())
            }
            Some(t) => Err(FormulaError::new(
                FormulaErrorKind::UnexpectedToken,
                format!("expected {:?}, found {:?}", kind, t.kind),
            )),
            None => Err(FormulaError::plain(FormulaErrorKind::UnexpectedEnd)),
        }
    }

    fn cell_regex(&self) -> &Regex {
        self._cell_re
            .get_or_init(|| Regex::new(r"^[A-Za-z]{1,3}[0-9]{1,7}$").unwrap())
    }

    /// Determine whether an identifier looks like an A1 cell reference.
    fn is_cell_ref(&self, ident: &str) -> bool {
        let cleaned = ident.trim_start_matches('$');
        self.cell_regex().is_match(cleaned)
    }

    /// Parse an identifier, possibly with $ prefix, into a CellReference.
    fn parse_cell_ident(&self, ident: &str, col_abs: bool, row_abs: bool) -> CellReference {
        let bytes = ident.as_bytes();
        let mut col: u32 = 0;
        let mut idx = 0;
        while idx < bytes.len() && bytes[idx].is_ascii_alphabetic() {
            let c = bytes[idx].to_ascii_uppercase();
            col = col * 26 + (c - b'A') as u32 + 1;
            idx += 1;
        }
        let row: u32 = ident[idx..].parse().unwrap_or(0);
        CellReference {
            row: row.saturating_sub(1),
            col: col.saturating_sub(1),
            row_absolute: row_abs,
            col_absolute: col_abs,
        }
    }

    /// Parse an expression using precedence climbing.
    pub fn parse_expression(&mut self, min_prec: u8) -> Result<Expr, FormulaError> {
        let mut lhs = self.parse_prefix()?;

        while let Some(tok) = self.peek() {
            // Postfix percent
            if matches!(tok.kind, TokenKind::Percent) {
                self.advance();
                lhs = Expr::Binary {
                    op: BinOp::Div,
                    left: Box::new(lhs),
                    right: Box::new(Expr::Number(Decimal::new(100, 0))),
                };
                continue;
            }

            let Some(prec) = precedence(&tok.kind) else {
                break;
            };
            if prec < min_prec {
                break;
            }
            let op = match &tok.kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Caret => BinOp::Pow,
                TokenKind::Amp => BinOp::Concat,
                TokenKind::Eq => BinOp::Eq,
                TokenKind::Ne => BinOp::Ne,
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            // Right-associative for ^
            let next_min = if op == BinOp::Pow { prec } else { prec + 1 };
            let rhs = self.parse_expression(next_min)?;
            lhs = Expr::Binary {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, FormulaError> {
        let Some(tok) = self.advance() else {
            return Err(FormulaError::plain(FormulaErrorKind::UnexpectedEnd));
        };

        match &tok.kind {
            TokenKind::Number(s) => {
                let d = s.parse::<Decimal>().map_err(|_| {
                    FormulaError::new(FormulaErrorKind::NumberParse, format!("bad number '{s}'"))
                })?;
                Ok(Expr::Number(d))
            }
            TokenKind::String(s) => Ok(Expr::Text(s.clone())),
            TokenKind::Bool(b) => Ok(Expr::Bool(*b)),
            TokenKind::Plus | TokenKind::Minus => {
                let op = if matches!(tok.kind, TokenKind::Plus) {
                    UnOp::Plus
                } else {
                    UnOp::Minus
                };
                let expr = self.parse_expression(6)?;
                Ok(Expr::Unary {
                    op,
                    expr: Box::new(expr),
                })
            }
            TokenKind::LParen => {
                let e = self.parse_expression(0)?;
                self.expect(&TokenKind::RParen)?;
                Ok(e)
            }
            TokenKind::Dollar => {
                let col_abs = true;
                let mut row_abs = false;

                // Check if we have a second dollar sign immediately after
                if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Dollar)) {
                    self.advance();
                    row_abs = true;
                }

                // Collect the identifier parts (column letters)
                let mut col_parts = Vec::new();
                if let Some(t) = self.peek() {
                    if let TokenKind::Identifier(part) = &t.kind {
                        col_parts.push(part.clone());
                        self.advance();
                    }
                }

                // Now handle the row number
                let mut row_parts = Vec::new();
                while let Some(t) = self.peek() {
                    match &t.kind {
                        TokenKind::Dollar => {
                            // Another dollar sign before row - this makes row absolute
                            self.advance();
                            row_abs = true;
                        }
                        TokenKind::Number(num) => {
                            row_parts.push(num.clone());
                            self.advance();
                            break;
                        }
                        TokenKind::Identifier(part) => {
                            // Handle case where row is also an identifier (like in mixed references)
                            row_parts.push(part.clone());
                            self.advance();
                            break;
                        }
                        _ => break,
                    }
                }

                let col_ident = col_parts.join("");
                let row_ident = row_parts.join("");
                let ident = format!("{}{}", col_ident, row_ident);

                if self.is_cell_ref(&ident) {
                    let c = self.parse_cell_ident(&ident, col_abs, row_abs);
                    Ok(Expr::CellRef(c))
                } else {
                    Err(FormulaError::new(
                        FormulaErrorKind::UnexpectedToken,
                        "expected cell reference after $",
                    ))
                }
            }
            TokenKind::Identifier(ident) => {
                let ident_owned = ident.clone();
                // Cell reference?
                if self.is_cell_ref(&ident_owned) {
                    let has_dollar = ident_owned.starts_with('$');
                    let (col_abs, row_abs, clean_ident) = if has_dollar {
                        let rest = ident_owned.trim_start_matches('$');
                        // Count the number of dollar signs
                        let dollar_count = ident_owned.matches('$').count();
                        let col_abs = dollar_count >= 1;
                        let row_abs = dollar_count >= 2;
                        (col_abs, row_abs, rest)
                    } else {
                        (false, false, ident_owned.as_str())
                    };
                    let c = self.parse_cell_ident(clean_ident, col_abs, row_abs);
                    // Range?
                    if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Colon)) {
                        self.advance();
                        let end = self.parse_range_end()?;
                        return Ok(Expr::Range(RangeReference::new(c, end)));
                    }
                    return Ok(Expr::CellRef(c));
                }
                // Function call?
                if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::LParen)) {
                    let name = ident_owned.to_uppercase();
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek().map(|t| &t.kind), Some(TokenKind::RParen)) {
                        loop {
                            let e = self.parse_expression(0)?;
                            args.push(e);
                            match self.peek().map(|t| &t.kind) {
                                Some(TokenKind::Comma) => {
                                    self.advance();
                                }
                                Some(TokenKind::RParen) => {
                                    break;
                                }
                                _ => {
                                    return Err(FormulaError::new(
                                        FormulaErrorKind::UnexpectedToken,
                                        "expected ',' or ')' in argument list",
                                    ));
                                }
                            }
                        }
                    }
                    self.expect(&TokenKind::RParen)?;
                    return Ok(Expr::Function { name, args });
                }
                // Named range
                Ok(Expr::Name(ident_owned))
            }
            _ => Err(FormulaError::new(
                FormulaErrorKind::UnexpectedToken,
                format!("unexpected token {:?}", tok.kind),
            )),
        }
    }

    /// Parse the right-hand side of a range (after a colon).
    fn parse_range_end(&mut self) -> Result<CellReference, FormulaError> {
        let col_abs = if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Dollar)) {
            self.advance();
            true
        } else {
            false
        };
        let row_abs = if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Dollar)) {
            self.advance();
            true
        } else {
            false
        };
        let t = self
            .advance()
            .ok_or_else(|| FormulaError::plain(FormulaErrorKind::UnexpectedEnd))?;
        let ident = match &t.kind {
            TokenKind::Identifier(ident) => ident.clone(),
            _ => {
                return Err(FormulaError::new(
                    FormulaErrorKind::UnexpectedToken,
                    "expected cell reference after ':'",
                ))
            }
        };
        if self.is_cell_ref(&ident) {
            Ok(self.parse_cell_ident(&ident, col_abs, row_abs))
        } else {
            Err(FormulaError::new(
                FormulaErrorKind::UnexpectedToken,
                "expected cell reference after ':'",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_arithmetic() {
        let f = parse("=1+2*3").unwrap();
        assert!(matches!(f.expr, Expr::Binary { op: BinOp::Add, .. }));
    }

    #[test]
    fn parse_cell_ref() {
        let f = parse("=A1").unwrap();
        match f.expr {
            Expr::CellRef(c) => {
                assert_eq!(c.row, 0);
                assert_eq!(c.col, 0);
                assert!(!c.row_absolute);
                assert!(!c.col_absolute);
            }
            _ => panic!("expected cell ref"),
        }
    }

    #[test]
    fn parse_absolute_ref() {
        let f = parse("=$B$2").unwrap();
        match f.expr {
            Expr::CellRef(c) => {
                assert_eq!(c.row, 1);
                assert_eq!(c.col, 1);
                assert!(c.row_absolute);
                assert!(c.col_absolute);
            }
            _ => panic!("expected cell ref"),
        }
    }

    #[test]
    fn parse_range() {
        let f = parse("=A1:B3").unwrap();
        match f.expr {
            Expr::Range(r) => {
                assert_eq!(r.start.row, 0);
                assert_eq!(r.start.col, 0);
                assert_eq!(r.end.row, 2);
                assert_eq!(r.end.col, 1);
            }
            _ => panic!("expected range"),
        }
    }

    #[test]
    fn parse_function() {
        let f = parse("=SUM(A1:A3, 5)").unwrap();
        match f.expr {
            Expr::Function { name, args } => {
                assert_eq!(name, "SUM");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("expected function"),
        }
    }

    #[test]
    fn parse_string_concat() {
        let f = parse("=\"a\" & \"b\"").unwrap();
        assert!(matches!(
            f.expr,
            Expr::Binary {
                op: BinOp::Concat,
                ..
            }
        ));
    }

    #[test]
    fn parse_comparison() {
        let f = parse("=1 < 2").unwrap();
        assert!(matches!(f.expr, Expr::Binary { op: BinOp::Lt, .. }));
    }

    #[test]
    fn parse_percent() {
        let f = parse("=50%").unwrap();
        assert!(matches!(f.expr, Expr::Binary { op: BinOp::Div, .. }));
    }

    #[test]
    fn parse_missing_paren_fails() {
        assert!(parse("=SUM(1,").is_err());
    }
}
