use crate::lexer::span::Span;
use crate::lexer::token::{Token, TokenKind, lookup_keyword};

pub struct Lexer<'a> {
    source: &'a str,
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.pos..].chars();
        chars.next()?;
        chars.next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn make_span(&self, start: usize, line: usize, column: usize) -> Span {
        Span::new(start, self.pos, line, column)
    }

    fn make_token(&self, kind: TokenKind, start: usize, line: usize, column: usize) -> Token<'a> {
        Token::new(
            kind,
            &self.source[start..self.pos],
            self.make_span(start, line, column),
        )
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.peek() != Some('/') || self.peek_next() != Some('/') {
            return;
        }

        // Consume '//'
        self.advance();
        self.advance();

        while let Some(ch) = self.peek() {
            // Consume until you find newline character
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_ignored(&mut self) {
        loop {
            // After skipping comment, there may be more whitespace or another comment.
            self.skip_whitespace();
            if self.peek() == Some('/') && self.peek_next() == Some('/') {
                self.skip_comment();
                continue;
            }
            break;
        }
    }

    fn identifier(&mut self) -> Token<'a> {
        let start = self.pos;
        let line = self.line;
        let column = self.column;

        // Assuming this function will be called only when lexeme is identifier
        debug_assert!(matches!(
            self.peek(),
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_'
        ));
        self.advance();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[start..self.pos];
        let kind = lookup_keyword(lexeme);
        self.make_token(kind, start, line, column)
    }

    fn number(&mut self) -> Token<'a> {
        let start = self.pos;
        let column = self.column;
        let line = self.line;

        let mut is_real = false;

        // Integer Part
        while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
            self.advance();
        }

        // Fractional Part
        if self.peek() == Some('.') {
            is_real = true;
            self.advance();

            while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                self.advance();
            }
        }

        // Exponent
        if matches!(self.peek(), Some('e' | 'E')) {
            is_real = true;
            self.advance();

            if matches!(self.peek(), Some('+' | '-')) {
                self.advance();
            }

            while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                self.advance();
            }
        }

        self.make_token(
            if is_real {
                TokenKind::Real
            } else {
                TokenKind::Integer
            },
            start,
            line,
            column,
        )
    }

    fn string(&mut self) -> Token<'a> {
        let start = self.pos;
        let line = self.line;
        let column = self.column;

        // Consume opening quote
        self.advance();

        loop {
            match self.peek() {
                Some('"') => {
                    self.advance(); // Consume closing quote
                    break;
                }

                Some('\n') => {
                    panic!("Unterminated string");
                }

                Some(_) => {
                    self.advance();
                }

                None => {
                    panic!("Unterminated string");
                }
            }
        }

        self.make_token(TokenKind::StringLiteral, start, line, column)
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_ignored();

        if self.is_eof() {
            return self.make_token(TokenKind::Eof, self.pos, self.line, self.column);
        }

        let ch = self.peek().unwrap();

        // Identifier / Keyword
        if ch.is_alphabetic() || ch == '_' {
            return self.identifier();
        }

        // Number
        if ch.is_ascii_digit() {
            return self.number();
        }

        // Number beginning with .
        if ch == '.' {
            if matches!(self.peek_next(), Some(c) if c.is_ascii_digit()) {
                return self.number();
            }
        }

        if ch == '"' {
            return self.string();
        }

        // Symbols
        let start = self.pos;
        let line = self.line;
        let column = self.column;

        match ch {
            '+' => {
                self.advance();
                self.make_token(TokenKind::Plus, start, line, column)
            }

            '-' => {
                self.advance();
                let kind = if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                };

                self.make_token(kind, start, line, column)
            }

            '*' => {
                self.advance();
                self.make_token(TokenKind::Star, start, line, column)
            }

            '/' => {
                self.advance();
                self.make_token(TokenKind::Slash, start, line, column)
            }

            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    return self.make_token(TokenKind::EqualEqual, start, line, column);
                } else {
                    panic!("Unexpexted '='");
                }
            }

            '(' => {
                self.advance();
                self.make_token(TokenKind::LeftParen, start, line, column)
            }

            ')' => {
                self.advance();
                self.make_token(TokenKind::RightParen, start, line, column)
            }

            '[' => {
                self.advance();
                self.make_token(TokenKind::LeftBracket, start, line, column)
            }

            ']' => {
                self.advance();
                self.make_token(TokenKind::RightBracket, start, line, column)
            }

            '{' => {
                self.advance();
                self.make_token(TokenKind::LeftBrace, start, line, column)
            }

            '}' => {
                self.advance();
                self.make_token(TokenKind::RightBrace, start, line, column)
            }

            ',' => {
                self.advance();
                self.make_token(TokenKind::Comma, start, line, column)
            }

            ';' => {
                self.advance();
                self.make_token(TokenKind::Semicolon, start, line, column)
            }

            _ => panic!("unexpected character '{}'", ch),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}
