use crate::{
    ast::{
        gate::{GateDecl, OpaqueDecl}, program::Program, stmt::{BarrierStmt, CRegDecl, GateCall, IfStmt, IncludeStmt, MeasureStmt, QRegDecl, ResetStmt, Stmt}, types::{Identifier, IndexedIdentifier}
    }, lexer::{Lexer, Token, TokenKind},

};

use super::error::ParserError;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, ParserError> {
        if self.current.kind != kind {
            return Err(ParserError::UnexpectedToken {
                expected: kind,
                found: self.current.kind,
                span: self.current.span,
            });
        }
        let token = self.current.clone();
        self.advance();

        Ok(token)
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        self.expect(TokenKind::OpenQasm)?;

        let version = self.expect(TokenKind::Real)?.lexeme.to_string();

        self.expect(TokenKind::Semicolon)?;

        let mut statements = Vec::new();

        while !self.check(TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }

        Ok(Program {
            version,
            statements,
        })
    }

    fn parse_include(&mut self) -> Result<IncludeStmt, ParserError> {
        self.expect(TokenKind::Include)?;

        let filename = self
            .expect(TokenKind::StringLiteral)?
            .lexeme
            .trim_matches('"')
            .to_string();
        self.expect(TokenKind::Semicolon)?;
        Ok(IncludeStmt { filename })
    }

    fn parse_indexed_identifier(&mut self) -> Result<IndexedIdentifier, ParserError> {
        let name = self.expect(TokenKind::Identifier)?.lexeme.to_string();
        self.expect(TokenKind::LeftBracket)?;

        let index = self.expect(TokenKind::Integer)?.lexeme.parse().unwrap();

        self.expect(TokenKind::RightBracket)?;

        Ok(IndexedIdentifier { name, index })
    }

    fn parse_qreg(&mut self) -> Result<QRegDecl, ParserError> {
        self.expect(TokenKind::QReg)?;

        let indexed_identifier = self.parse_indexed_identifier()?;

        self.expect(TokenKind::Semicolon)?;
        Ok(QRegDecl {
            name: indexed_identifier.name,
            size: indexed_identifier.index,
        })
    }

    fn parse_creg(&mut self) -> Result<CRegDecl, ParserError> {
        self.expect(TokenKind::CReg)?;

        let indexed_identifier = self.parse_indexed_identifier()?;

        self.expect(TokenKind::Semicolon)?;
        Ok(CRegDecl {
            name: indexed_identifier.name,
            size: indexed_identifier.index,
        })
    }

    fn parse_measure(&mut self) -> Result<MeasureStmt, ParserError> {
        self.expect(TokenKind::Measure)?;

        let qubit = self.parse_indexed_identifier()?;

        self.expect(TokenKind::Arrow)?;

        let target = self.parse_indexed_identifier()?;

        self.expect(TokenKind::Semicolon)?;

        Ok(MeasureStmt { qubit, target })
    }

    fn parse_reset(&mut self) -> Result<ResetStmt, ParserError> {
        self.expect(TokenKind::Reset)?;

        let qubit = self.parse_indexed_identifier()?;

        self.expect(TokenKind::Semicolon)?;

        Ok(ResetStmt { qubit })
    }

    fn parse_barrier(&mut self) -> Result<BarrierStmt, ParserError> {
        self.expect(TokenKind::Barrier)?;

        let mut qubits = Vec::new();

        qubits.push(self.parse_indexed_identifier()?);

        while self.check(TokenKind::Comma) {
            self.advance();
            qubits.push(self.parse_indexed_identifier()?);
        }

        self.expect(TokenKind::Semicolon)?;

        Ok(BarrierStmt { qubits })
    }

    fn parse_gate_call(&mut self) -> Result<GateCall, ParserError> {
        let name = self.expect(TokenKind::Identifier)?.lexeme.to_string();

        let mut parameters = Vec::new();
        if self.check(TokenKind::LeftParen) {
            self.advance();
            parameters.push(self.parse_expression()?);

            while self.check(TokenKind::Comma) {
                self.advance();
                parameters.push(self.parse_expression()?);
            }

            self.expect(TokenKind::RightParen)?;
        }

        let mut arguments = Vec::new();
        arguments.push(self.parse_indexed_identifier()?);

        while self.check(TokenKind::Comma) {
            self.advance();
            arguments.push(self.parse_indexed_identifier()?);
        }

        self.expect(TokenKind::Semicolon)?;

        Ok(GateCall {
            name,
            parameters,
            arguments,
        })
    }

    fn parse_if(&mut self) -> Result<IfStmt, ParserError> {
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;

        let creg = 
            self.expect(TokenKind::Identifier)?
                .lexeme
                .to_string();

        self.expect(TokenKind::EqualEqual)?;

        let value = 
            self.expect(TokenKind::Integer)?
                .lexeme
                .parse()
                .map_err(|_err| ParserError::InvalidInteger { span: self.current.span })?;
        
        self.expect(TokenKind::RightParen)?;

        let statement = Box::new(self.parse_statement()?);

        Ok(IfStmt { creg, value, statement})

    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>, ParserError> {
        let mut identifiers = Vec::new();
        identifiers.push(self.expect(TokenKind::Identifier)?.lexeme.to_string());

        while self.check(TokenKind::Comma) {
            self.advance();
            let identifier = self.expect(TokenKind::Identifier)?.lexeme.to_string();
            identifiers.push(identifier);
        }

        Ok(identifiers)
    }

    fn parse_identifier_list_in_parens(&mut self) -> Result<Vec<String>, ParserError> {
        self.expect(TokenKind::LeftParen)?;
        let identifiers = self.parse_identifier_list()?;
        self.expect(TokenKind::RightParen)?;

        Ok(identifiers)
    }

    fn parse_opaque(&mut self) -> Result<OpaqueDecl, ParserError> {
        self.expect(TokenKind::Opaque)?;

        let name = 
            self.expect(TokenKind::Identifier)?
                .lexeme
                .to_string();

        let parameters = self.parse_identifier_list_in_parens()?;

        let qubits = self.parse_identifier_list()?;
        self.expect(TokenKind::Semicolon);
        Ok(OpaqueDecl { name, parameters, qubits})
    }

    fn parse_gate_decl(&mut self) -> Result<GateDecl, ParserError> {
        self.expect(TokenKind::Gate)?;

        let name =
            self.expect(TokenKind::Identifier)?
                .lexeme
                .to_string();
        
        let parameters = self.parse_identifier_list_in_parens()?;
        let qubits = self.parse_identifier_list()?;

        self.expect(TokenKind::LeftBrace)?;
        let mut body = Vec::new();

        while !self.check(TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(GateDecl { name, parameters, qubits, body })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParserError> {
        match self.current.kind {
            TokenKind::Include => Ok(Stmt::Include(self.parse_include()?)),

            TokenKind::QReg => Ok(Stmt::QReg(self.parse_qreg()?)),

            TokenKind::CReg => Ok(Stmt::CReg(self.parse_creg()?)),

            TokenKind::Gate => Ok(Stmt::Gate(self.parse_gate_decl()?)),

            TokenKind::Opaque => Ok(Stmt::Opaque(self.parse_opaque()?)),

            TokenKind::Measure => Ok(Stmt::Measure(self.parse_measure()?)),

            TokenKind::Reset => Ok(Stmt::Reset(self.parse_reset()?)),

            TokenKind::Barrier => Ok(Stmt::Barrier(self.parse_barrier()?)),

            TokenKind::If => Ok(Stmt::If(self.parse_if()?)),

            TokenKind::Identifier => Ok(Stmt::GateCall(self.parse_gate_call()?)),

            _ => Err(ParserError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: self.current.kind,
                span: self.current.span,
            })
        }
    }
}
