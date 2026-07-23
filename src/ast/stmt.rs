use crate::ast::gate::{GateDecl, OpaqueDecl};

use super::expr::Expr;
use super::types::IndexedIdentifier;

pub enum Stmt {
    Include(IncludeStmt),
    QReg(QRegDecl),
    CReg(CRegDecl),
    Gate(GateDecl),
    Opaque(OpaqueDecl),
    Measure(MeasureStmt),
    Reset(ResetStmt),
    Barrier(BarrierStmt),
    GateCall(GateCall),
    If(IfStmt),
}

pub struct IncludeStmt {
    pub filename: String,
}

pub struct QRegDecl {
    pub name: String,
    pub size: usize,
}

pub struct CRegDecl {
    pub name: String,
    pub size: usize,
}

pub struct MeasureStmt {
    pub qubit: IndexedIdentifier,
    pub target: IndexedIdentifier,
}

pub struct ResetStmt {
    pub qubit: IndexedIdentifier,
}

pub struct BarrierStmt {
    pub qubits: Vec<IndexedIdentifier>,
}

pub struct GateCall {
    pub name: String,
    pub parameters: Vec<Expr>,
    pub arguments: Vec<IndexedIdentifier>,
}

pub struct IfStmt {
    pub creg: String,
    pub value: usize,
    pub statement: Box<Stmt>,
}
