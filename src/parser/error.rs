use crate::lexer::{Span, TokenKind};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    },

    ExpectedIdentifier {
        span: Span,
    },

    InvalidInteger {
        span: Span,
    },

    UnexpectedEof,
}
