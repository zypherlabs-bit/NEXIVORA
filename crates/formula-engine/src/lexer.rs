//! Formula tokenizer.

use crate::ast::{BinOp, UnOp};
use crate::error::{FormulaError, FormulaErrorKind};

/// A lexical token in a formula.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A numeric literal.
    Number(String),
    /// A string literal (contents without quotes).
    String(String),
    /// An identifier (function name, cell ref, or named range).
    Identifier(String),
    /// A boolean literal TRUE/FALSE.
    Bool(bool),
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `^` operator.
    Caret,
    /// The `&` concatenation operator.
    Amp,
    /// The `=` operator (also formula prefix).
    Eq,
    /// The `<>` operator.
    Ne,
    /// The `<` operator.
    Lt,
    /// The `<=` operator.
    Le,
    /// The `>` operator.
    Gt,
    /// The `>=` operator.
    Ge,
    /// The `(` left parenthesis.
    LParen,
    /// The `)` right parenthesis.
    RParen,
    /// The `,` comma (argument separator).
    Comma,
    /// The `:` colon (range separator).
    Colon,
    /// The `$` absolute-reference marker.
    Dollar,
    /// The `!` sheet separator.
    Bang,
    /// The `%` percent operator.
    Percent,
}

/// A token with its kind and position.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    /// Byte offset in the source.
    pub pos: usize,
}

/// Tokenize a formula body (without the leading `=`).
pub fn tokenize(input: &str) -> Result<Vec<Token>, FormulaError> {
    let b = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;

    while i < b.len() {
        let c = b[i] as char;

        // Skip whitespace
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // String literal
        if c == '"' {
            let start = i;
            i += 1;
            let mut buf = String::new();
            let mut closed = false;
            while i < b.len() {
                if b[i] as char == '"' {
                    // Handle doubled quotes as escaped quote
                    if i + 1 < b.len() && b[i + 1] as char == '"' {
                        buf.push('"');
                        i += 2;
                    } else {
                        closed = true;
                        i += 1;
                        break;
                    }
                } else {
                    // Decode UTF-8 by indexing full char
                    let ch = input[i..].chars().next().unwrap();
                    buf.push(ch);
                    i += ch.len_utf8();
                }
            }
            if !closed {
                return Err(FormulaError::new(
                    FormulaErrorKind::UnexpectedEnd,
                    format!("unterminated string at offset {start}"),
                ));
            }
            tokens.push(Token {
                kind: TokenKind::String(buf),
                pos: start,
            });
            continue;
        }

        // Number
        if c.is_ascii_digit() || (c == '.' && i + 1 < b.len() && (b[i + 1] as char).is_ascii_digit())
        {
            let start = i;
            let mut has_dot = false;
            let mut has_exp = false;
            while i < b.len() {
                let ch = b[i] as char;
                if ch.is_ascii_digit() {
                    i += 1;
                } else if ch == '.' && !has_dot && !has_exp {
                    has_dot = true;
                    i += 1;
                } else if (ch == 'e' || ch == 'E') && !has_exp {
                    // Look ahead for optional sign
                    let next_is_digit = i + 1 < b.len()
                        && (((b[i + 1] as char).is_ascii_digit())
                            || ((b[i + 1] as char) == '+' || (b[i + 1] as char) == '-')
                                && i + 2 < b.len()
                                && (b[i + 2] as char).is_ascii_digit());
                    if next_is_digit {
                        has_exp = true;
                        i += 1;
                        if i < b.len() && ((b[i] as char) == '+' || (b[i] as char) == '-') {
                            i += 1;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            tokens.push(Token {
                kind: TokenKind::Number(input[start..i].to_string()),
                pos: start,
            });
            continue;
        }

        // Identifiers and booleans
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < b.len() && ((b[i] as char).is_ascii_alphanumeric() || b[i] as char == '_') {
                i += 1;
            }
            let ident = &input[start..i];
            let upper = ident.to_uppercase();
            let kind = if upper == "TRUE" {
                TokenKind::Bool(true)
            } else if upper == "FALSE" {
                TokenKind::Bool(false)
            } else {
                TokenKind::Identifier(ident.to_string())
            };
            tokens.push(Token {
                kind,
                pos: start,
            });
            continue;
        }

        // Operators and punctuation
        let (kind, advance) = match c {
            '+' => (TokenKind::Plus, 1),
            '-' => (TokenKind::Minus, 1),
            '*' => (TokenKind::Star, 1),
            '/' => (TokenKind::Slash, 1),
            '^' => (TokenKind::Caret, 1),
            '&' => (TokenKind::Amp, 1),
            '%' => (TokenKind::Percent, 1),
            '(' => (TokenKind::LParen, 1),
            ')' => (TokenKind::RParen, 1),
            ',' => (TokenKind::Comma, 1),
            ':' => (TokenKind::Colon, 1),
            '$' => (TokenKind::Dollar, 1),
            '!' => (TokenKind::Bang, 1),
            '=' => (TokenKind::Eq, 1),
            '<' => {
                if i + 1 < b.len() && b[i + 1] as char == '>' {
                    (TokenKind::Ne, 2)
                } else if i + 1 < b.len() && b[i + 1] as char == '=' {
                    (TokenKind::Le, 2)
                } else {
                    (TokenKind::Lt, 1)
                }
            }
            '>' => {
                if i + 1 < b.len() && b[i + 1] as char == '=' {
                    (TokenKind::Ge, 2)
                } else {
                    (TokenKind::Gt, 1)
                }
            }
            _ => {
                return Err(FormulaError::new(
                    FormulaErrorKind::UnexpectedToken,
                    format!("unexpected character '{}' at offset {i}", c),
                ));
            }
        };
        tokens.push(Token { kind, pos: i });
        i += advance;
    }

    Ok(tokens)
}

/// Helper to map a comparison operator token to a `BinOp`.
pub fn comparison_binop(kind: &TokenKind) -> Option<BinOp> {
    match kind {
        TokenKind::Eq => Some(BinOp::Eq),
        TokenKind::Ne => Some(BinOp::Ne),
        TokenKind::Lt => Some(BinOp::Lt),
        TokenKind::Le => Some(BinOp::Le),
        TokenKind::Gt => Some(BinOp::Gt),
        TokenKind::Ge => Some(BinOp::Ge),
        _ => None,
    }
}

/// Helper to map an arithmetic operator token to a `BinOp`.
pub fn arithmetic_binop(kind: &TokenKind) -> Option<BinOp> {
    match kind {
        TokenKind::Plus => Some(BinOp::Add),
        TokenKind::Minus => Some(BinOp::Sub),
        TokenKind::Star => Some(BinOp::Mul),
        TokenKind::Slash => Some(BinOp::Div),
        TokenKind::Caret => Some(BinOp::Pow),
        TokenKind::Amp => Some(BinOp::Concat),
        _ => None,
    }
}

/// Helper to map unary operator tokens.
pub fn unary_op(kind: &TokenKind) -> Option<UnOp> {
    match kind {
        TokenKind::Plus => Some(UnOp::Plus),
        TokenKind::Minus => Some(UnOp::Minus),
        _ => None,
    }
}