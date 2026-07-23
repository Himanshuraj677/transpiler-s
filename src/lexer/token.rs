use super::span::Span;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Keywords
    OpenQasm,
    Include,
    QReg,
    CReg,
    Gate,
    Opaque,
    Measure,
    Reset,
    Barrier,
    If,

    // Constants
    Pi,
    Cos,
    Sin,
    Tan,
    Exp,
    Ln,
    Sqrt,
    Asin,
    Acos,
    Atan,

    // Literals
    Identifier,
    Integer,
    Real,
    StringLiteral,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    EqualEqual,
    Arrow,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // Specia;
    Eof,
    Invalid,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenKind::OpenQasm => "OpenQasm",
            TokenKind::Include => "Include",
            TokenKind::QReg => "QReg",
            TokenKind::CReg => "CReg",
            TokenKind::Gate => "Gate",
            TokenKind::Opaque => "Opaque",
            TokenKind::Measure => "Measure",
            TokenKind::Reset => "Reset",
            TokenKind::Barrier => "Barrier",
            TokenKind::If => "If",
            TokenKind::Pi => "Pi",
            TokenKind::Cos => "Cos",
            TokenKind::Sin => "Sin",
            TokenKind::Tan => "Tan",
            TokenKind::Exp => "Exp",
            TokenKind::Ln => "Ln",
            TokenKind::Sqrt => "Sqrt",
            TokenKind::Asin => "Asin",
            TokenKind::Acos => "Acos",
            TokenKind::Atan => "Atan",
            TokenKind::Identifier => "Identifier",
            TokenKind::Integer => "Integer",
            TokenKind::Real => "Real",
            TokenKind::StringLiteral => "StringLiteral",
            TokenKind::Plus => "Plus",
            TokenKind::Minus => "Minus",
            TokenKind::Star => "Star",
            TokenKind::Slash => "Slash",
            TokenKind::Caret => "Caret",
            TokenKind::EqualEqual => "EqualEqual",
            TokenKind::Arrow => "Arrow",
            TokenKind::LeftParen => "LeftParen",
            TokenKind::RightParen => "RightParen",
            TokenKind::LeftBracket => "LeftBracket",
            TokenKind::RightBracket => "RightBracket",
            TokenKind::LeftBrace => "LeftBrace",
            TokenKind::RightBrace => "RightBrace",
            TokenKind::Comma => "Comma",
            TokenKind::Semicolon => "Semicolon",
            TokenKind::Eof => "Eof",
            TokenKind::Invalid => "Invalid",
        };
        f.pad(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    lexeme: &'a str,
    span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, span: Span) -> Self {
        Self { kind, lexeme, span }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<16} {:<20} {:>4}..{:<4} {:>3}:{:<3}",
            self.kind,
            self.lexeme,
            self.span.start,
            self.span.end,
            self.span.line,
            self.span.column,
        )
    }
}

pub fn lookup_keyword(text: &str) -> TokenKind {
    match text {
        "OPENQASM" => TokenKind::OpenQasm,
        "include" => TokenKind::Include,
        "qreg" => TokenKind::QReg,
        "creg" => TokenKind::CReg,
        "gate" => TokenKind::Gate,
        "opaque" => TokenKind::Opaque,
        "measure" => TokenKind::Measure,
        "reset" => TokenKind::Reset,
        "barrier" => TokenKind::Barrier,
        "if" => TokenKind::If,

        "pi" => TokenKind::Pi,
        "sin" => TokenKind::Sin,
        "cos" => TokenKind::Cos,
        "tan" => TokenKind::Tan,
        "exp" => TokenKind::Exp,
        "ln" => TokenKind::Ln,
        "sqrt" => TokenKind::Sqrt,
        "asin" => TokenKind::Asin,
        "acos" => TokenKind::Acos,
        "atan" => TokenKind::Atan,

        _ => TokenKind::Identifier,
    }
}
